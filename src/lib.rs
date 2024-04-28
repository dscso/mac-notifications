#![cfg(target_os = "macos")]
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
    center: Id<NSUserNotificationCenter>,
}

impl NotificationProvider {
    pub fn new(app_name: &str) -> Self {
        MainThreadMarker::new().expect("init() must be on the main thread");
        let app_name = NSString::from_str(app_name);
        let app_name = app_name.deref();

        unsafe {
            sys::init(app_name);
        };
        let center = unsafe { NSUserNotificationCenter::defaultUserNotificationCenter() };
        Self {
            delegate: None,
            center,
        }
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

    pub fn get_all_notifications(&self) -> Vec<Notification> {
        MainThreadMarker::new().expect("get_all_notificaitons() must be on the main thread");
        let mut notifications = vec![];
        unsafe {
            let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
            let notifications_array = notification_center.deliveredNotifications();
            let count = notifications_array.count();
            for i in 0..count {
                let notification = notifications_array.objectAtIndex(i);
                notifications.push(Notification::from(notification.as_ref()));
            }
        }
        notifications
    }

    pub fn delete(&self, identifier: &str) {
        MainThreadMarker::new().expect("delete() must be on the main thread");
        unsafe {
            let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
            let notifications_array = notification_center.deliveredNotifications();
            let count = notifications_array.count();
            for i in 0..count {
                let notification = notifications_array.objectAtIndex(i);
                let notification = notification.as_ref();
                if let Some(id) = notification.identifier() {
                    if id.to_string() == identifier {
                        notification_center.removeDeliveredNotification(notification);
                    }
                }
            }
        }
    }

    pub fn delete_all(&self) {
        MainThreadMarker::new().expect("delete_all() must be on the main thread");
        unsafe {
            self.center.removeAllDeliveredNotifications();
        }
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
