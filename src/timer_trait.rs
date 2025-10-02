use crate::error::SchedulerError;

pub trait Timer {
    /// Starts the timer, the timer will run continuously until stopped
    fn start(&self) -> impl Future<Output = anyhow::Result<(), SchedulerError>>;
    /// Aborts the timer early
    fn stop(&self) -> impl Future<Output = anyhow::Result<(), SchedulerError>>;
    /// Resets the elapsed time to zero without stopping the timer
    fn reset(&self) -> impl Future<Output = anyhow::Result<(), SchedulerError>>;
    /// Returns true if the timer is currently running
    fn is_running(&self) -> impl Future<Output = bool> + Send;
}
