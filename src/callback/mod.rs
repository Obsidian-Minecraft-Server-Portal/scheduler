use crate::timer_trait::Timer;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct CallbackTimer<F>
where
    F: FnMut() -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> + Send + 'static,
{
    callback: Arc<Mutex<F>>,
    interval: tokio::time::Duration,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl<F> CallbackTimer<F>
where
    F: FnMut() -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> + Send + 'static,
{
    pub fn new(callback: F, interval: tokio::time::Duration) -> Self {
        CallbackTimer {
            callback: Arc::new(Mutex::new(callback)),
            interval,
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

impl<F> Timer for CallbackTimer<F>
where
    F: FnMut() -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> + Send + 'static,
{
    fn start(&self) {
        let callback = Arc::clone(&self.callback);
        let interval = self.interval;
        let handle = Arc::clone(&self.handle);

        let task = tokio::spawn(async move {
            tokio::time::sleep(interval).await;
            let mut cb = callback.lock().await;
            let future = cb();
            drop(cb); // Release the lock before awaiting
            if let Err(e) = future.await {
                #[cfg(feature = "log")]
                log::error!("Callback execution failed: {:?}", e);
                #[cfg(not(feature = "log"))]
                eprintln!("Callback execution failed: {}", e);
            }
        });

        // Store the handle so we can stop it later
        let handle_clone = Arc::clone(&handle);
        tokio::spawn(async move {
            let mut h = handle_clone.lock().await;
            *h = Some(task);
        });
    }

    fn stop(&self) {
        let handle = Arc::clone(&self.handle);
        tokio::spawn(async move {
            let mut h = handle.lock().await;
            if let Some(task) = h.take() {
                task.abort();
            }
        });
    }

    fn reset(&self) {
        self.stop();
        self.start();
    }
}
