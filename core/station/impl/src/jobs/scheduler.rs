use orbit_essentials::cdk::{api::time, spawn};

use super::ScheduledJob;

pub trait ScheduleStrategy {
    const TOLERANCE_SEC: u64 = 3;

    // If the call results in a new timer, return the timer value.
    fn add_timer(&self, at_ns: u64) -> Option<u64>;
    // If the call results in a timer being removed, return the timer value.
    fn remove_timer(&self, at_ns: u64) -> Option<u64>;
    // Return whether the job is currently running.
    fn is_running(&self) -> bool;
    // Set whether the job is currently running.
    fn set_running(&self, running: bool);
}

pub struct Scheduler;

impl Scheduler {
    pub fn cancel_scheduled_timer<'a, Strategy: ScheduleStrategy>(
        strategy: Strategy,
        timer_at_ns: u64,
    ) {
        strategy.remove_timer(timer_at_ns);
    }

    pub fn schedule<'a, Strategy: ScheduleStrategy + 'static, Job: ScheduledJob>(
        strategy: Strategy,
        delay_ns: u64,
    ) {
        let now = time();
        let future_time_ns = now + delay_ns;

        if let Some(timer_at_ns) = strategy.add_timer(future_time_ns) {
            spawn(async move {
                if !strategy.is_running() {
                    strategy.set_running(true);
                    let job_complete = Job::run().await;
                    strategy.set_running(false);
                    strategy.remove_timer(timer_at_ns);

                    if !job_complete {
                        Self::schedule::<Strategy, Job>(strategy, 0)
                    }
                } else {
                    Self::schedule::<Strategy, Job>(
                        strategy,
                        Strategy::TOLERANCE_SEC * 1_000_000_000,
                    )
                };
            });
        }
    }
}
