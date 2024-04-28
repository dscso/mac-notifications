#![cfg(target_os = "macos")]
#![allow(deprecated)]

pub use crate::notification::Notification;
pub use crate::notification_response::NotificationResponse;
pub use crate::provider::NotificationProvider;

mod delegate;
pub mod misc;
pub mod notification;
mod notification_response;
mod provider;