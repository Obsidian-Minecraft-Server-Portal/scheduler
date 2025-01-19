# Obsidian Scheduler

Obsidian Scheduler is a lightweight asynchronous scheduling system for the Minecraft server panel, [Obsidian Server Portal](https://github.com/Obsidian-Minecraft-Server-Portal/). It provides a robust API for managing and executing recurring or one-time tasks.

This library is available as a Git dependency and is designed to be integrated into the Obsidian Server Portal to schedule actions like maintenance, backups, or gameplay-related events.

---

## Features

- **Custom Time Durations:** Use the flexible `Duration` struct to represent time in days, hours, minutes, and seconds.
- **Schedules and Recurrence:** Create schedules that run once or periodically with an easy-to-use API.
- **Asynchronous Tick Management:** Automatically tick and execute schedules in a background thread.
- **Thread Safe & Singleton-based Control:** Utilize the globally available `ScheduleManager` for managing tasks.
- **Macros for Easy Usage:** Predefined macros to simplify adding, starting, or stopping tasks.

---

## Installation

Since this library is only available as a Git dependency, add the following to your `Cargo.toml`:

```toml
[dependencies]
obsidian-scheduler = { git = "https://github.com/Obsidian-Minecraft-Server-Portal/scheduler.git" }
```

---

## Usage

### How to Add a Schedule

You can use the `add_schedule!` macro to add tasks:

```rust
use obsidian_scheduler::{add_schedule, start_ticking_schedules};

fn main() {
    // Start ticking schedules in a background thread
    start_ticking_schedules!();

    // Add a one-time schedule that executes after 10 seconds
    let duration = obsidian_scheduler::duration::Duration::from_seconds(10);
    add_schedule!(duration, false, false, |schedule| {
        println!("Schedule with ID {} executed!", schedule.id);
    });
}
```

### Remove a Schedule

Use the `remove_schedule!` macro to remove schedules. Provide a closure to determine which schedules to remove:

```rust
use obsidian_scheduler::remove_schedule;

fn remove_example() {
    remove_schedule!(|schedule| schedule.id == 1);
}
```

### Ticking Events

The `start_ticking_schedules!` macro runs schedules asynchronously in the background:

```rust
use obsidian_scheduler::{start_ticking_schedules, stop_ticking_schedules};

fn toggle_ticking() {
    // Start ticking
    start_ticking_schedules!();

    // After some time, stop ticking
    std::thread::sleep(std::time::Duration::from_secs(30));
    stop_ticking_schedules!();
}
```

---

## Provided Modules

### **[`Duration`](src/duration.rs)**
A custom time struct with extensive methods for creating and manipulating durations.

Example:

```rust
use obsidian_scheduler::duration::Duration;

// Create durations
let duration = Duration::from_seconds(3600); // 1 hour
let another = Duration::from_minutes(30);

// Add durations together
let combined = duration + another;
println!("Combined Duration: {}", combined);
```

### **[`Schedule`](src/schedule.rs)**
Defines a schedule, its properties, and behavior for execution.

- **Custom Actions:** Assign a custom action using closures.
- **Control:** Activate/deactivate your schedules or check their end times.

Example:

```rust
use obsidian_scheduler::schedule::Schedule;
use obsidian_scheduler::duration::Duration;

let duration = Duration::from_seconds(60);
let schedule = Schedule::new(1, duration, false, |schedule| {
    println!("Schedule {} triggered!", schedule.id);
});

println!("Schedule will execute at: {:?}", schedule.get_end_time());
```

### **[`ScheduleManager`](src/schedule_manager.rs)**
Singleton-based manager for handling all schedules in a thread-safe manner.

- **Add & Remove Schedules:** Programmatically manage tasks.
- **Ticking:** Iterate through tasks and trigger actions when necessary.

Example:

```rust
use obsidian_scheduler::schedule_manager::{ScheduleManager, SCHEDULE_MANAGER_SINGLETON};
use obsidian_scheduler::duration::Duration;

let mut manager = SCHEDULE_MANAGER_SINGLETON.lock().unwrap();
let id = manager.add_schedule(Duration::from_seconds(30), false, true, |schedule| {
    println!("Task executed with ID: {}", schedule.id);
});
manager.remove_schedule(|schedule| schedule.id == id);
```

---

## Macros Reference

- **`add_schedule!(duration, reoccurring, execute_immediately, action)`**
    - Adds a new schedule to the manager.

- **`remove_schedule!(filter)`**
    - Removes schedules based on the provided filter closure.

- **`start_ticking_schedules!()`**
    - Starts ticking schedules in an asynchronous loop.

- **`stop_ticking_schedules!()`**
    - Stops the ticking of schedules.

---

## Contributing

To contribute to this project, open a pull request or file an issue in the [GitHub repository](https://github.com/Obsidian-Minecraft-Server-Portal/scheduler.git). Contributions are welcome!

---

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Obsidian-Minecraft-Server-Portal/scheduler/blob/main/LICENSE) file for more details.