use std::time::Duration;
use obsidian_scheduler::callback::CallbackTimer;
use obsidian_scheduler::timer_trait::Timer;

#[tokio::main]
async fn main() {
	let timer = CallbackTimer::new(
		move || {
			Box::pin(async move {
				println!("Timer fired!");
				Ok(())
			})
		},
		Duration::from_secs(5),
	);

	timer.start();

	println!("This should print immediately, waiting for timer to fire ...");

	tokio::time::sleep(Duration::from_secs(6)).await;
	println!("Timer fired, exiting now.");
}
