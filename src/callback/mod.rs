use crate::error::SchedulerError;
use crate::timer_trait::Timer;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Handle for controlling a timer from within a callback
/// This can be cloned and moved into async blocks
#[derive(Clone)]
pub struct TimerHandle {
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl TimerHandle {
    /// Stops the timer
    pub fn stop(&self) {
        let handle = Arc::clone(&self.handle);
        tokio::spawn(async move {
            let mut h = handle.lock().await;
            if let Some(task) = h.take() {
                task.abort();
            }
        });
    }
}

type BoxedCallback = Box<
    dyn Fn(TimerHandle) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> + Send + Sync,
>;

pub struct CallbackTimer {
    callback: Arc<BoxedCallback>,
    interval: tokio::time::Duration,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl CallbackTimer {
    pub fn new<F, Fut>(callback: F, interval: tokio::time::Duration) -> Arc<Self>
    where
        F: Fn(TimerHandle) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let boxed: BoxedCallback = Box::new(move |handle| Box::pin(callback(handle)));

        Arc::new(CallbackTimer {
            callback: Arc::new(boxed),
            interval,
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

impl Timer for CallbackTimer {
    async fn start(&self) -> Result<(), SchedulerError> {
        if self.is_running().await {
            return Err(SchedulerError::TimerAlreadyExists(
                "CallbackTimer".to_string(),
            ));
        }

        let callback = Arc::clone(&self.callback);
        let interval = self.interval;
        let handle = Arc::clone(&self.handle);

        // Create a TimerHandle that can be passed to the callback
        let timer_handle = TimerHandle {
            handle: Arc::clone(&self.handle),
        };

        let task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                let future = callback(timer_handle.clone());
                if let Err(e) = future.await {
                    #[cfg(feature = "log")]
                    log::error!("Callback execution failed: {:?}", e);
                    #[cfg(not(feature = "log"))]
                    eprintln!("Callback execution failed: {}", e);
                }
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
            return Err(SchedulerError::TimerNotRunning("CallbackTimer".to_string()));
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
