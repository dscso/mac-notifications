// This file is part of prose-app-web
//
// Copyright 2024, Prose Foundation

#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

/**************************************************************************
 * IMPORTS
 * ************************************************************************* */

use objc2::runtime::ProtocolObject;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::Once;

use objc2::rc::Id;
use objc2::{ClassType, DeclaredClass, ProtocolType};
use objc2_foundation::{
    MainThreadMarker, NSCopying, NSUserNotificationCenter, NSUserNotificationCenterDelegate,
};

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
    use objc2_foundation::NSString;

    #[link(name = "notification")]
    extern "C" {
        pub fn init(app_name: *const NSString); // -> *const NSUserNotificationCenterDelegate;
    }
}
unsafe fn get_delegate() -> &'static Id<RustNotificationDelegate> {
    static mut DELEGATE: MaybeUninit<Id<RustNotificationDelegate>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        DELEGATE.write(RustNotificationDelegate::new());
    });

    DELEGATE.assume_init_ref()
}

/// Initialize the notification system
/// This function should be called once in the application
pub fn init(app_name: &str) {
    let app_name = NSString::from_str(app_name);
    let app_name = app_name.deref();

    unsafe {
        // check if is main thread
        MainThreadMarker::new().expect("init() must be on the main thread");

        sys::init(app_name);

        let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
        let delegate = get_delegate();
        notification_center.setDelegate(Some(ProtocolObject::from_ref(delegate.as_ref())));
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
