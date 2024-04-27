#![feature(let_chains)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]
#![allow(deprecated)]

use objc2::runtime::NSObjectProtocol;
use std::ops::Deref;

use objc2::rc::Id;
use objc2::ClassType;
use objc2_foundation::{MainThreadMarker, NSUserNotificationCenter};

use crate::delegate::RustNotificationDelegate;
use objc2_foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop, NSString};

pub use crate::notification::Notification;
pub use crate::notification_response::NotificationResponse;

mod delegate;
pub mod misc;
pub mod notification;
mod notification_response;

/**************************************************************************
 * MODULES
 * ************************************************************************* */
pub mod sys {
    use objc2_foundation::NSString;

    #[link(name = "notification")]
    extern "C" {
        pub fn init(app_name: *const NSString);
    }
}

pub struct NotificationProvider {
    delegate: Option<Id<RustNotificationDelegate>>,
}

impl NotificationProvider {
    pub fn new(app_name: &str) -> Self {
        MainThreadMarker::new().expect("init() must be on the main thread");
        let app_name = NSString::from_str(app_name);
        let app_name = app_name.deref();

        unsafe {
            sys::init(app_name);
        };

        Self { delegate: None }
    }

    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(String, NotificationResponse) + 'static,
    {
        let delegate = RustNotificationDelegate::new(callback);
        self.delegate = Some(delegate);
    }
    pub fn run_main_loop_once(&self) {
        run_main_loop_once();
    }
}
pub fn run_main_loop_once() {
    MainThreadMarker::new().expect("run_main_loop_once() must be on the main thread");

    unsafe {
        let main_loop = NSRunLoop::mainRunLoop();
        let limit_date = NSDate::dateWithTimeIntervalSinceNow(0.1);
        main_loop.runMode_beforeDate(NSDefaultRunLoopMode, &limit_date);
    }
}
impl Drop for NotificationProvider {
    fn drop(&mut self) {
        unsafe {
            let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
            if let Some(delegate) = notification_center.delegate() {
                if delegate
                    .as_ref()
                    .isKindOfClass(RustNotificationDelegate::class())
                {
                    notification_center.setDelegate(None);
                }
            }
        }
    }
}
