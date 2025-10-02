use crate::timer_trait::Timer;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::task::JoinHandle;

pub struct EventTimer {
	pub event_name: String,
	pub interval: tokio::time::Duration,
	sender: broadcast::Sender<String>,
	handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl EventTimer {
	pub fn new(event_name: impl Into<String>, interval: tokio::time::Duration) -> Self {
		let (sender, _) = broadcast::channel(100);
		EventTimer {
			event_name: event_name.into(),
			interval,
			sender,
			handle: Arc::new(Mutex::new(None)),
		}
	}

	/// Subscribe to receive timer events. Returns a Receiver that will get the event_name
	/// each time the timer fires.
	pub fn subscribe(&self) -> broadcast::Receiver<String> {
		self.sender.subscribe()
	}
}

impl Timer for EventTimer {
	fn start(&self) {
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

	async fn is_running(&self) -> bool {
		let handle = Arc::clone(&self.handle);
		let h = handle.lock().await;
		h.is_some()
	}
}