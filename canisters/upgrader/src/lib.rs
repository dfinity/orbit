use std::{cell::RefCell, thread::LocalKey, time::Duration};

use ic_cdk::{init, post_upgrade, update};
use ic_cdk_timers::set_timer_interval;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use lazy_static::lazy_static;

use crate::{
    hash::{Hash, Sha256Hasher},
    interface::{InitArg, QueueUpgradeError, QueueUpgradeResponse, UpgradeParams},
    queue::{Queue, QueueError, Queuer, WithAuthorization, WithHexDecode},
    upgrade::{Upgrade, Upgrader, WithCleanup},
};

mod hash;
mod interface;
mod queue;
mod upgrade;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type StableMap<K, V> = StableBTreeMap<K, V, Memory>;
type StableValue<T> = StableMap<(), T>;
type LocalRef<T> = &'static LocalKey<RefCell<T>>;

const SECOND: Duration = Duration::from_secs(1);

const MEMORY_ID_TARGET_CANISTER_ID: u8 = 0;
const MEMORY_ID_QUEUED_UPGRADE_PARAMS: u8 = 1;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

thread_local! {
    static TARGET_CANISTER_ID: RefCell<StableValue<String>> = RefCell::new(
        StableValue::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_TARGET_CANISTER_ID))),
        )
    );

    static QUEUED_UPGRADE_PARAMS: RefCell<StableValue<UpgradeParams>> = RefCell::new(
        StableValue::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_QUEUED_UPGRADE_PARAMS))),
        )
    );
}

pub struct VerifyChecksum<T, H>(T, H);

pub struct WithLogs<T>(T, String);

pub struct CheckController<T>(T, LocalRef<StableValue<String>>);

fn init_timers_fn() {
    set_timer_interval(10 * SECOND, || {
        if let Some(ps) = QUEUED_UPGRADE_PARAMS.with(|ps| ps.borrow().get(&())) {
            ic_cdk::spawn(async {
                let _ = UPGRADER.upgrade(ps).await;
            });
        }
    });
}

#[init]
fn init_fn(InitArg { target_canister }: InitArg) {
    TARGET_CANISTER_ID.with(|id| {
        let mut id = id.borrow_mut();
        id.insert((), target_canister.to_string());
    });

    init_timers_fn();
}

#[post_upgrade]
fn post_upgrade_fn() {
    init_timers_fn();
}

thread_local! {
    static HASHER: RefCell<Box<dyn Hash>> = RefCell::new({
        let h = Sha256Hasher;
        Box::new(h)
    });
}

lazy_static! {
    static ref QUEUER: Box<dyn Queue> = {
        let q = Queuer::new(&QUEUED_UPGRADE_PARAMS);
        let q = VerifyChecksum(q, &HASHER);
        let q = WithHexDecode(q);
        let q = CheckController(q, &TARGET_CANISTER_ID);
        let q = WithAuthorization(q, &TARGET_CANISTER_ID);
        let q = WithLogs(q, "queue".to_string());
        Box::new(q)
    };
}

lazy_static! {
    static ref UPGRADER: Box<dyn Upgrade> = {
        let u = Upgrader::new(&TARGET_CANISTER_ID);
        let u = VerifyChecksum(u, &HASHER);
        let u = CheckController(u, &TARGET_CANISTER_ID);
        let u = WithCleanup(u, &QUEUED_UPGRADE_PARAMS);
        let u = WithLogs(u, "upgrade".to_string());
        Box::new(u)
    };
}

#[update(name = "queueUpgrade")]
async fn queue_upgrade(params: UpgradeParams) -> QueueUpgradeResponse {
    match QUEUER.queue(params).await {
        Ok(()) => QueueUpgradeResponse::Ok,
        Err(err) => QueueUpgradeResponse::Err(match err {
            QueueError::ChecksumMismatch => QueueUpgradeError::ChecksumMismatch,
            QueueError::NotController => QueueUpgradeError::NotController,
            QueueError::Unauthorized => QueueUpgradeError::Unauthorized,
            QueueError::UnexpectedError(err) => QueueUpgradeError::UnexpectedError(err.to_string()),
        }),
    }
}
