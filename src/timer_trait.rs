pub trait Timer {
    /// Starts the timer
    fn start(&self);
    /// Aborts the timer early
    fn stop(&self);
    /// Resets the elapsed time to zero without stopping the timer
    fn reset(&self);
    /// Returns true if the timer is currently running
    async fn is_running(&self) -> bool;
}
