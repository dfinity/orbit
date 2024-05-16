#[cfg(not(test))]
pub use ic_cdk_timers::*;

#[cfg(test)]
pub use mocks::*;

pub mod mocks {

    use std::time::Duration;

    type TimerEntry = (TimerId, Box<dyn FnOnce() + Send + 'static>);

    thread_local! {
        static TIMERS: std::cell::RefCell<Vec<TimerEntry>> = Default::default();
    }

    pub type TimerId = u64;

    pub fn set_timer(_duration: Duration, callback: impl FnOnce() + Send + 'static) -> TimerId {
        TIMERS.with(|timers| {
            let timer_id = timers.borrow().last().map(|(id, _)| id).unwrap_or(&0) + 1;
            timers.borrow_mut().push((timer_id, Box::new(callback)));
            timer_id
        })
    }

    pub fn clear_timer(timer_id: TimerId) {
        TIMERS.with(|timers| {
            let mut timers = timers.borrow_mut();
            if let Some(index) = timers.iter().position(|(id, _)| *id == timer_id) {
                let _ = timers.remove(index);
            }
        })
    }
}
