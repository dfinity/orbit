use std::cell::RefCell;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::cdk::api::time;
use std::fmt::Debug;

// The following code implementing canister locks with optional expiration is adapted from
// https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#recommendation-1

pub struct State<T: Ord> {
    pending_requests: BTreeMap<T, Option<u64>>,
}

impl<T: Ord> Default for State<T> {
    fn default() -> Self {
        Self {
            pending_requests: BTreeMap::new(),
        }
    }
}

pub struct CallerGuard<T: Ord> {
    state: Rc<RefCell<State<T>>>,
    lock: T,
}

pub struct CallerGuardParams {
    pub max_concurrency: Option<usize>,
    pub expires_at_ns: Option<u64>,
}

impl<T: Clone + Ord + Debug> CallerGuard<T> {
    pub fn new(state: Rc<RefCell<State<T>>>, lock: T, params: CallerGuardParams) -> Option<Self> {
        {
            let pending_requests = &mut state.borrow_mut().pending_requests;
            if let Some(existing_request) = pending_requests.get(&lock) {
                if let Some(expires_at_ns) = existing_request {
                    if expires_at_ns > &time() {
                        // Lock is already held by another caller and has not expired.
                        return None;
                    } else {
                        // Lock has expired, fall through to update the lock.
                        crate::cdk::api::print(format!("Lock has expired for {:?}", lock));
                        pending_requests.remove(&lock);
                    }
                } else {
                    // Lock is held indefinitely.
                    return None;
                }
            }
            if let Some(max_concurrency) = params.max_concurrency {
                if pending_requests.len() >= max_concurrency {
                    return None;
                }
            }
            pending_requests.insert(lock.clone(), params.expires_at_ns);
        }

        Some(Self { state, lock })
    }
}

impl<T: Ord> Drop for CallerGuard<T> {
    fn drop(&mut self) {
        self.state.borrow_mut().pending_requests.remove(&self.lock);
    }
}
