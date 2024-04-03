use prometheus::{CounterVec, Encoder, Gauge, Opts, Registry, TextEncoder};
use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};

thread_local! {
  /// The metric registries for each service name.
  static REGISTRIES : RefCell<HashMap<String, MetricsRegistry>> = RefCell::new(HashMap::new());
}

// Exported prometheus types.
pub use prometheus::labels;

use crate::api::{HeaderField, HttpResponse};
use crate::cdk::api::print;

/// Executes the given closure with the metric store for the given service name.
pub fn with_metrics_registry<T, F>(service_name: &str, f: F) -> T
where
    F: FnOnce(&mut MetricsRegistry) -> T,
{
    REGISTRIES.with(|registries| {
        let mut registries = registries.borrow_mut();
        let registry = registries
            .entry(service_name.to_string())
            .or_insert_with(|| MetricsRegistry::new(service_name.to_string()));

        f(registry)
    })
}

/// A registry for metrics collection.
pub struct MetricsRegistry {
    service_name: String,
    registry: Registry,
    metric_gauges: HashMap<String, Gauge>,
    metric_counter_vecs: HashMap<String, CounterVec>,
}

impl MetricsRegistry {
    pub fn new(service_name: String) -> Self {
        Self {
            service_name,
            metric_gauges: HashMap::new(),
            metric_counter_vecs: HashMap::new(),
            registry: Registry::new(),
        }
    }

    /// Returns the registry for the metrics.
    pub fn get_registry(&self) -> &Registry {
        &self.registry
    }

    /// Returns a counter vec metric with the given name and set of label names.
    pub fn counter_vec_mut(&mut self, name: &str, label_names: &[&str]) -> &mut CounterVec {
        match self.metric_counter_vecs.entry(name.to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let counter = CounterVec::new(
                    Opts::new(
                        format!("{}_{}", self.service_name, name),
                        format!("number of times {} was called", name),
                    ),
                    label_names,
                )
                .unwrap();

                self.registry.register(Box::new(counter.clone())).unwrap();

                entry.insert(counter)
            }
        }
    }

    /// Removes a counter vec metric with the given name.
    pub fn remove_counter_vec(&mut self, name: &str) {
        if let Some(counter) = self.metric_counter_vecs.remove(name) {
            self.registry
                .unregister(Box::new(counter))
                .expect("Failed to unregister counter vec");
        }
    }

    /// Returns a gauge metric with the given name and helper message that explains what
    /// the gauge is measuring or tracking.
    pub fn gauge_mut(&mut self, name: &str, helper_message: &str) -> &mut Gauge {
        match self.metric_gauges.entry(name.to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let gauge =
                    Gauge::new(format!("{}_{}", self.service_name, name), helper_message).unwrap();

                self.registry.register(Box::new(gauge.clone())).unwrap();

                entry.insert(gauge)
            }
        }
    }

    /// Removes a gauge metric with the given name.
    pub fn remove_gauge(&mut self, name: &str) {
        if let Some(gauge) = self.metric_gauges.remove(name) {
            self.registry
                .unregister(Box::new(gauge))
                .expect("Failed to unregister gauge");
        }
    }

    /// Exports the metrics in the registry to a buffer in text format.
    pub fn export_metrics(&self) -> Result<Vec<u8>, prometheus::Error> {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();

        let registry = self.get_registry();
        let metrics_family = registry.gather();

        encoder.encode(&metrics_family, &mut buffer)?;

        Ok(buffer)
    }

    /// Exports the metrics in the registry to an HTTP response.
    pub fn export_metrics_as_http_response(&self) -> HttpResponse {
        let metrics_result = self.export_metrics();

        match metrics_result {
            Ok(metrics) => HttpResponse {
                status_code: 200,
                headers: vec![HeaderField(
                    "Content-Type".to_string(),
                    "text/plain".to_string(),
                )],
                body: metrics,
            },
            Err(err) => {
                print(format!("Error exporting metrics: {:?}", err));

                HttpResponse {
                    status_code: 500,
                    headers: vec![HeaderField(
                        "Content-Type".to_string(),
                        "text/plain".to_string(),
                    )],
                    body: "500 Internal Server Error".as_bytes().to_owned(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::labels;

    #[test]
    fn test_metrics_registry() {
        let mut registry = MetricsRegistry::new("default".to_string());

        let counter = registry.counter_vec_mut("test_counter", &["status"]);
        counter.with(&labels! { "status" => "ok" }).inc();
        counter.with(&labels! { "status" => "fail" }).inc();

        let gauge = registry.gauge_mut("test_gauge", "test gauge");
        gauge.set(42.0);

        let buffer = registry.export_metrics().unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("default_test_counter{status=\"ok\"} 1"));
        assert!(output.contains("default_test_counter{status=\"fail\"} 1"));
        assert!(output.contains("default_test_gauge 42"));
    }

    #[test]
    fn test_with_metrics_registry() {
        let result = with_metrics_registry("custom_registry", |registry| {
            let gauge = registry.gauge_mut("test_gauge", "test gauge");
            gauge.set(10.0);

            registry.export_metrics().unwrap()
        });

        let output = String::from_utf8(result).unwrap();

        assert!(output.contains("custom_registry_test_gauge 10"));
    }

    #[test]
    fn test_registries_are_independent() {
        with_metrics_registry("first_registry", |registry| {
            let gauge = registry.gauge_mut("test_gauge", "test gauge");
            gauge.set(10.0);
        });

        with_metrics_registry("second_registry", |registry| {
            let gauge = registry.gauge_mut("test_gauge", "test gauge");
            gauge.set(20.0);
        });

        let first_output = with_metrics_registry("first_registry", |registry| {
            registry.export_metrics().unwrap()
        });

        let second_output = with_metrics_registry("second_registry", |registry| {
            registry.export_metrics().unwrap()
        });

        let first_output = String::from_utf8(first_output).unwrap();
        let second_output = String::from_utf8(second_output).unwrap();

        assert!(first_output.contains("first_registry_test_gauge 10"));
        assert!(second_output.contains("second_registry_test_gauge 20"));
    }

    #[test]
    fn test_remove_counter_vec() {
        let mut registry = MetricsRegistry::new("default".to_string());

        let counter = registry.counter_vec_mut("test_counter", &["status"]);
        counter.with(&labels! { "status" => "ok" }).inc();

        let buffer = registry.export_metrics().unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("default_test_counter{status=\"ok\"} 1"));

        registry.remove_counter_vec("test_counter");

        let buffer = registry.export_metrics().unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(!output.contains("default_test_counter{status=\"ok\"} 1"));
    }

    #[test]
    fn test_remove_gauge() {
        let mut registry = MetricsRegistry::new("default".to_string());

        let gauge = registry.gauge_mut("test_gauge", "test gauge");
        gauge.set(42.0);

        let buffer = registry.export_metrics().unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("default_test_gauge 42"));

        registry.remove_gauge("test_gauge");

        let buffer = registry.export_metrics().unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(!output.contains("default_test_gauge 42"));
    }
}
