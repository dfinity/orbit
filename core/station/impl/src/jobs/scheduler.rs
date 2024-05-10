use std::time::Duration;

use ic_cdk_timers::{clear_timer, set_timer, TimerId};
use orbit_essentials::cdk::{api::time, spawn};

use super::ScheduledJob;

pub enum TimerAction {
    AddedNew(u64),
    UsedExisting(u64),
}

pub trait ScheduleStrategy {
    const TOLERANCE_SEC: u64 = 3;

    /// If the call results in a new timer, return the timer value.
    fn add_schedule(&self, at_ns: u64) -> TimerAction;
    /// If the call results in a timer getting cancelled, return the timer value.
    fn remove_schedule(&self, at_ns: u64) -> Option<TimerId>;
    /// Return whether the job is currently running.
    fn is_running(&self) -> bool;
    /// Set whether the job is currently running.
    fn set_running(&self, running: bool);
    /// Saves the timer id and the time at which it was scheduled.
    fn save_timer_id(&self, timer_id: TimerId, at_ns: u64);
}

pub struct Scheduler;

impl Scheduler {
    pub fn cancel_scheduled_timer<Strategy: ScheduleStrategy>(
        strategy: Strategy,
        timer_at_ns: u64,
    ) {
        if let Some(timer_id) = strategy.remove_schedule(timer_at_ns) {
            clear_timer(timer_id)
        }
    }

    pub fn schedule<Strategy: ScheduleStrategy + Clone + 'static, Job: ScheduledJob>(
        strategy: Strategy,
        delay_ns: u64,
    ) {
        let now = time();
        let future_time_ns = now + delay_ns;

        match strategy.add_schedule(future_time_ns) {
            TimerAction::AddedNew(timer_at_ns) => {
                let cloned_strategy = strategy.clone();

                let timer_id = set_timer(
                    Duration::from_nanos(timer_at_ns.saturating_sub(time())),
                    move || {
                        spawn(async move {
                            if !strategy.is_running() {
                                strategy.set_running(true);
                                let job_complete = Job::run().await;
                                strategy.set_running(false);
                                strategy.remove_schedule(timer_at_ns);

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
                    },
                );

                cloned_strategy.save_timer_id(timer_id, timer_at_ns);
            }
            TimerAction::UsedExisting(_) => {
                // Do nothing
            }
        }
    }
}
