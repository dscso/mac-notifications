#![cfg(target_os = "macos")]
#![allow(deprecated)]

//! # macOS Notification Provider
//!
//! This crate provides a simple abstraction of the [NSUserNotification](https://developer.apple.com/documentation/foundation/nsusernotification) API on macOS.
//!
//! ## Limitations
//! the NSUserNotification API is deprecated, therefore in future MacOS versions there could be problems. However, Electron and a lot of other applications still use the same deprecated API therefore, it should be fine for now.
//!
//! ## Working principle
//! The crate provides a `NotificationProvider` struct which is the main struct for interacting with the notifications.
//! Notifications can be sent directly via the `Notification` struct.
//!
//! ## Example
//! ```rust
//! use mac_notifications::{Notification, NotificationProvider};
//!
//! let mut provider = NotificationProvider::new("Terminal");
//!
//! // callback for notification interaction
//! provider.set_callback(|id, resp| {
//!   println!("Notification {} clicked: {:?}", id, resp);
//! });
//!
//! // sends notification
//! Notification::new()
//!     .title("Hello")
//!     .subtitle("This is a notification")
//!     .reply(true)
//!     .send().unwrap();
//!
//! // to wait for user interactions you need to run the main loop (here for 5 seconds)
//! for _ in 0..50 {
//!     provider.run_main_loop_once();
//! }
//!```

mod delegate;
pub mod misc;
mod notification;
mod notification_response;
mod provider;

pub use notification::Notification;
pub use notification_response::NotificationResponse;
pub use provider::NotificationProvider;
