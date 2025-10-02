use obsidian_scheduler::event::EventTimer;
use obsidian_scheduler::timer_trait::Timer;

#[tokio::main]
async fn main() {
	let timer = EventTimer::new("my_timer", tokio::time::Duration::from_secs(3));
	
	// Subscribe multiple receivers to the same timer (broadcast pattern)
	let mut receiver1 = timer.subscribe();
	let mut receiver2 = timer.subscribe();
	let mut receiver3 = timer.subscribe();
	
	// Start the timer
	timer.start();
	
	println!("Timer started with 3 subscribers, waiting for events...");
	
	// Spawn tasks for each receiver
	let task1 = tokio::spawn(async move {
		for i in 1..=3 {
			match receiver1.recv().await {
				Ok(event_name) => {
					println!("Receiver 1 got event #{}: {}", i, event_name);
				}
				Err(e) => {
					eprintln!("Receiver 1 error: {}", e);
					break;
				}
			}
		}
	});
	
	let task2 = tokio::spawn(async move {
		for i in 1..=3 {
			match receiver2.recv().await {
				Ok(event_name) => {
					println!("Receiver 2 got event #{}: {}", i, event_name);
				}
				Err(e) => {
					eprintln!("Receiver 2 error: {}", e);
					break;
				}
			}
		}
	});
	
	let task3 = tokio::spawn(async move {
		for i in 1..=3 {
			match receiver3.recv().await {
				Ok(event_name) => {
					println!("Receiver 3 got event #{}: {}", i, event_name);
				}
				Err(e) => {
					eprintln!("Receiver 3 error: {}", e);
					break;
				}
			}
		}
	});
	
	// Wait for all receivers to complete
	let _ = tokio::join!(task1, task2, task3);
	
	// Stop the timer
	timer.stop();
	println!("Timer stopped. All receivers completed.");
}
