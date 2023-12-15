use std::cell::RefCell;

use prometheus::{Gauge, Registry};

const SERVICE_NAME: &str = "control_panel";

thread_local! {
    pub static GAUGE_CANISTER_CYCLES_BALANCE: RefCell<Gauge> = RefCell::new({
        Gauge::new(
            format!("{SERVICE_NAME}_canister_cycles_balance"), // name
            "cycles balance available to the canister", // help
        ).unwrap()
    });

    pub static METRICS_REGISTRY: RefCell<Registry> = RefCell::new({
        let r = Registry::new();

        GAUGE_CANISTER_CYCLES_BALANCE.with(|g| {
            let g = Box::new(g.borrow().to_owned());
            r.register(g).unwrap();
        });

        r
    });
}
