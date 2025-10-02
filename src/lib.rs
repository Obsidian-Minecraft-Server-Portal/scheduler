#[cfg(feature = "event-timers")]
pub mod event;
#[cfg(feature = "callback-timers")]
pub mod callback;
pub mod timer_trait;
pub mod error;

#[cfg(all(not(feature = "callback-timers"), not(feature = "event-timers")))]
compile_error!("At least one of the features 'callback-timers' or 'event-timers' must be enabled.");