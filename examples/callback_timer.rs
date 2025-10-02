use std::sync::Arc;
use std::time::Duration;
use obsidian_scheduler::callback::CallbackTimer;
use obsidian_scheduler::timer_trait::Timer;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let is_okay_to_exit = Arc::new(Mutex::new(false));
    let is_okay_to_exit_clone = Arc::clone(&is_okay_to_exit);
    println!("Starting callback timer ...");
    
    let timer = CallbackTimer::new(
        move || {
            let is_okay_to_exit = Arc::clone(&is_okay_to_exit_clone);
            Box::pin(async move {
                println!("Timer fired!");
                let mut flag = is_okay_to_exit.lock().await;
                *flag = true;
                Ok(())
            })
        },
        Duration::from_secs(5),
    );
    
    timer.start();

    println!("This should print immediately, waiting for timer to fire ...");

    loop {
        let flag = is_okay_to_exit.lock().await;
        if *flag {
            break;
        }
        drop(flag); // Release the lock before sleeping
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("Timer fired, exiting now.");
}
