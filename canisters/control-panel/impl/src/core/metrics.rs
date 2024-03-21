use std::cell::RefCell;

use prometheus::{CounterVec, Gauge, Opts, Registry};

const SERVICE_NAME: &str = "control_panel";

thread_local! {
    pub static COUNTER_REGISTER_USER_TOTAL: RefCell<CounterVec> = RefCell::new({
        CounterVec::new(Opts::new(
            format!("{SERVICE_NAME}_register_user_total"), // name
            "number of times register_user was called", // help
        ), &["status"]).unwrap()
    });

    pub static COUNTER_MANAGE_USER_TOTAL: RefCell<CounterVec> = RefCell::new({
        CounterVec::new(Opts::new(
            format!("{SERVICE_NAME}_manage_user_total"), // name
            "number of times manage_user was called", // help
        ), &["status"]).unwrap()
    });

    pub static COUNTER_SUBSCRIBE_TO_WAITING_LIST_TOTAL: RefCell<CounterVec> = RefCell::new({
        CounterVec::new(Opts::new(
            format!("{SERVICE_NAME}_subscribe_to_waiting_list_total"), // name
            "number of times subscribe_to_waiting_list was called", // help
        ), &["status"]).unwrap()
    });

    pub static COUNTER_DELETE_USER_TOTAL: RefCell<CounterVec> = RefCell::new({
        CounterVec::new(Opts::new(
            format!("{SERVICE_NAME}_delete_user_total"), // name
            "number of times delete_user was called", // help
        ), &["status"]).unwrap()
    });

    pub static COUNTER_DEPLOY_WALLET_TOTAL: RefCell<CounterVec> = RefCell::new({
        CounterVec::new(Opts::new(
            format!("{SERVICE_NAME}_deploy_wallet_total"), // name
            "number of times deploy_wallet was called", // help
        ), &["status"]).unwrap()
    });

    pub static GAUGE_USERS_TOTAL: RefCell<Gauge> = RefCell::new({
        Gauge::new(
            format!("{SERVICE_NAME}_users_total"), // name
            "registered users", // help
        ).unwrap()
    });

    pub static GAUGE_CANISTER_CYCLES_BALANCE: RefCell<Gauge> = RefCell::new({
        Gauge::new(
            format!("{SERVICE_NAME}_canister_cycles_balance"), // name
            "cycles balance available to the canister", // help
        ).unwrap()
    });

    pub static METRICS_REGISTRY: RefCell<Registry> = RefCell::new({
        let r = Registry::new();

        COUNTER_REGISTER_USER_TOTAL.with(|c| {
            let c = Box::new(c.borrow().to_owned());
            r.register(c).unwrap();
        });

        COUNTER_MANAGE_USER_TOTAL.with(|c| {
            let c = Box::new(c.borrow().to_owned());
            r.register(c).unwrap();
        });

        COUNTER_DELETE_USER_TOTAL.with(|c| {
            let c = Box::new(c.borrow().to_owned());
            r.register(c).unwrap();
        });

        COUNTER_DEPLOY_WALLET_TOTAL.with(|c| {
            let c = Box::new(c.borrow().to_owned());
            r.register(c).unwrap();
        });

        GAUGE_USERS_TOTAL.with(|g| {
            let g = Box::new(g.borrow().to_owned());
            r.register(g).unwrap();
        });

        GAUGE_CANISTER_CYCLES_BALANCE.with(|g| {
            let g = Box::new(g.borrow().to_owned());
            r.register(g).unwrap();
        });

        r
    });
}
