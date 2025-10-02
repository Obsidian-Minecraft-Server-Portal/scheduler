use crate::error::SchedulerError;
use crate::timer_trait::Timer;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, broadcast};
use tokio::task::JoinHandle;

static ACTIVE_TIMERS: OnceLock<HashMap<String, Arc<Mutex<EventTimer>>>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct EventTimer {
    pub event_name: String,
    pub interval: tokio::time::Duration,
    sender: broadcast::Sender<String>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl EventTimer {
    pub fn new(
        event_name: impl Into<String>,
        interval: tokio::time::Duration,
    ) -> Result<Self, SchedulerError> {
        let (sender, _) = broadcast::channel(100);
        let timer = EventTimer {
            event_name: event_name.into(),
            interval,
            sender,
            handle: Arc::new(Mutex::new(None)),
        };
        timer.register_timer()?;

        Ok(timer)
    }

    /// Subscribe to receive timer events. Returns a Receiver that will get the event_name
    /// each time the timer fires.
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }

    fn register_timer(&self) -> Result<(), SchedulerError> {
        let timers = ACTIVE_TIMERS.get_or_init(HashMap::new);
        let mut timers = timers.clone();
        if timers.contains_key(&self.event_name) {
            return Err(SchedulerError::TimerAlreadyExists(self.event_name.clone()));
        }
        timers.insert(self.event_name.clone(), Arc::new(Mutex::new(self.clone())));

        Ok(())
    }
    pub fn get_timer_by_name(name: impl AsRef<str>) -> Option<EventTimer> {
        let timers = ACTIVE_TIMERS.get_or_init(HashMap::new);
        timers.get(name.as_ref()).map(|t| t.blocking_lock().clone())
    }
}

impl Timer for EventTimer {
    async fn start(&self) -> Result<(), SchedulerError> {
        
        if self.is_running().await {
            return Err(SchedulerError::TimerAlreadyExists(self.event_name.clone()));
        }
        
        let sender = self.sender.clone();
        let interval = self.interval;
        let event_name = self.event_name.clone();
        let handle = Arc::clone(&self.handle);

        let task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                // Send the event to all subscribers
                // Ignore send errors (happens when no receivers are listening)
                let _ = sender.send(event_name.clone());
            }
        });

        // Store the handle so we can stop it later
        let handle_clone = Arc::clone(&handle);
        tokio::spawn(async move {
            let mut h = handle_clone.lock().await;
            *h = Some(task);
        });
        Ok(())
    }

    async fn stop(&self) -> Result<(), SchedulerError> {
        
        if !self.is_running().await {
            return Err(SchedulerError::TimerNotRunning(self.event_name.clone()));
        }
        
        let handle = Arc::clone(&self.handle);
        tokio::spawn(async move {
            let mut h = handle.lock().await;
            if let Some(task) = h.take() {
                task.abort();
            }
        });

        Ok(())
    }

    async fn reset(&self) -> Result<(), SchedulerError> {
        self.stop().await?;
        self.start().await?;
        Ok(())
    }

    async fn is_running(&self) -> bool {
        let handle = Arc::clone(&self.handle);
        let handle_guard = handle.lock().await;
        handle_guard.is_some()
    }
}
