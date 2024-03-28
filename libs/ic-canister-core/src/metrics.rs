use prometheus::{CounterVec, Encoder, Gauge, Opts, Registry, TextEncoder};
use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};

thread_local! {
  pub static DEFAULT_SERVICE_NAME : RefCell<String> = RefCell::new("default".to_string());
  pub static STORES : RefCell<Stores> = RefCell::new(Stores::new());
}

struct Stores {
    stores: HashMap<String, MetricStore>,
}

impl Stores {
    pub fn new() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }

    pub fn get_or_create_store(&mut self, name: &str) -> &mut MetricStore {
        match self.stores.entry(name.to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let store = MetricStore::new(name.to_string());
                entry.insert(store)
            }
        }
    }
}

/// Sets the default service name for metrics collection.
pub fn set_default_service_name(name: &str) {
    DEFAULT_SERVICE_NAME.with(|default_name| {
        *default_name.borrow_mut() = name.to_string();
    });
}

/// Executes the given closure with the default metric store.
pub fn with_metric_store<T, F>(f: F) -> T
where
    F: FnOnce(&mut MetricStore) -> T,
{
    let service_name = DEFAULT_SERVICE_NAME.with(|name| name.borrow().clone());

    with_service_metric_store(&service_name, f)
}

/// Executes the given closure with the metric store for the given service name.
pub fn with_service_metric_store<T, F>(service_name: &str, f: F) -> T
where
    F: FnOnce(&mut MetricStore) -> T,
{
    STORES.with(|stores| {
        let mut stores = stores.borrow_mut();
        let store = stores.get_or_create_store(service_name);

        f(store)
    })
}

pub struct MetricStore {
    service_name: String,
    metrics: HashMap<String, Metric>,
    registry: Registry,
}

impl MetricStore {
    pub fn new(service_name: String) -> Self {
        Self {
            service_name,
            metrics: HashMap::new(),
            registry: Registry::new(),
        }
    }

    pub fn get_registry(&self) -> &Registry {
        &self.registry
    }

    pub fn status_counter_mut(&mut self, name: &str) -> &mut CounterVec {
        match self.metrics.entry(name.to_string()) {
            Entry::Occupied(entry) => match entry.into_mut() {
                Metric::Counter(counter) => counter,
                _ => panic!("Unexpected metric type for a counter entry"),
            },
            Entry::Vacant(entry) => {
                let counter = CounterVec::new(
                    Opts::new(
                        format!("{}_{}_total", self.service_name, name), // name
                        format!("number of times {} was called", name),  // help
                    ),
                    &["status"],
                )
                .unwrap();

                match entry.insert(Metric::Counter(counter)) {
                    Metric::Counter(counter) => {
                        self.registry.register(Box::new(counter.clone())).unwrap();

                        counter
                    }
                    // Since we just inserted a CounterVec, this pattern is unreachable
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn gauge_mut(&mut self, name: &str, helper: &str) -> &mut Gauge {
        match self.metrics.entry(name.to_string()) {
            Entry::Occupied(entry) => match entry.into_mut() {
                Metric::Gauge(gauge) => gauge,
                _ => panic!("Unexpected metric type for a gauge entry"),
            },
            Entry::Vacant(entry) => {
                let gauge = Gauge::new(
                    format!("{}_{}", self.service_name, name), // name
                    helper,                                    // help
                )
                .unwrap();

                match entry.insert(Metric::Gauge(gauge)) {
                    Metric::Gauge(gauge) => {
                        self.registry.register(Box::new(gauge.clone())).unwrap();

                        gauge
                    }
                    // Since we just inserted a Gauge, this pattern is unreachable
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn export_metrics(&self) -> Result<Vec<u8>, prometheus::Error> {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();

        let registry = self.get_registry();
        let metrics_family = registry.gather();

        encoder.encode(&metrics_family, &mut buffer)?;

        Ok(buffer)
    }
}

pub enum Metric {
    Counter(CounterVec),
    Gauge(Gauge),
}
