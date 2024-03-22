use std::cell::RefCell;
use std::cmp::Ord;
use std::collections::BTreeSet;
use std::rc::Rc;

// The following code implementing canister locks is adapted from
// https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#recommendation-10

pub struct State<T: Ord> {
    pending_requests: BTreeSet<T>,
}

impl<T: Ord> Default for State<T> {
    fn default() -> Self {
        Self {
            pending_requests: BTreeSet::new(),
        }
    }
}

pub struct CallerGuard<T: Ord> {
    state: Rc<RefCell<State<T>>>,
    t: T,
}

impl<T: Clone + Ord> CallerGuard<T> {
    pub fn new(state: Rc<RefCell<State<T>>>, t: T) -> Option<Self> {
        {
            let pending_requests = &mut state.borrow_mut().pending_requests;
            if pending_requests.contains(&t) {
                return None;
            }
            pending_requests.insert(t.clone());
        }
        Some(Self { state, t })
    }
}

impl<T: Ord> Drop for CallerGuard<T> {
    fn drop(&mut self) {
        self.state.borrow_mut().pending_requests.remove(&self.t);
    }
}
