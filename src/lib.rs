// This file is part of prose-app-web
//
// Copyright 2024, Prose Foundation

#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

/**************************************************************************
 * IMPORTS
 * ************************************************************************* */

use objc2::runtime::{ProtocolObject};
use std::ops::Deref;

use objc2::{ClassType, DeclaredClass, ProtocolType};
use objc2_foundation::{NSCopying, NSUserNotificationCenter, NSUserNotificationCenterDelegate};

use crate::delegate::RustNotificationDelegate;
use objc2_foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop, NSString};

pub use crate::notification::NotificationResponse;
pub use crate::notification_struct::Notification;

mod delegate;
pub mod misc;
mod notification;
pub mod notification_struct;

/**************************************************************************
 * MODULES
 * ************************************************************************* */
mod sys {
    use objc2_foundation::{NSString};

    #[link(name = "notification")]
    extern "C" {
        pub fn init(app_name: *const NSString); // -> *const NSUserNotificationCenterDelegate;
    }
}


/// Initialize the notification system
/// This function should be called once in the application
pub fn init(app_name: &str) {
    let app_name = NSString::from_str(app_name);
    let app_name = app_name.deref();

    unsafe {
        sys::init(app_name);
        let delegate = RustNotificationDelegate::new();
        let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
        notification_center.setDelegate(Some(ProtocolObject::from_ref(delegate.as_ref())));
    }
}

pub fn run_main_loop_once() {
    unsafe {
        let main_loop = NSRunLoop::mainRunLoop();
        let limit_date = NSDate::dateWithTimeIntervalSinceNow(0.1);
        main_loop.runMode_beforeDate(NSDefaultRunLoopMode, &limit_date);
    }
}
