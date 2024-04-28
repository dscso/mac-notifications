//! # Miscellaneous
//! This module contains functions that are not directly related to notifications but are still useful building desktop applications.

use objc2_app_kit::NSApplication;
use objc2_foundation::NSString;
use objc2_foundation::{MainThreadMarker, NSDistributedNotificationCenter, NSNotificationName};
use std::ops::Deref;

/// Lets the Download Icon in Dock bounce. Nothing happens if file does not exist
/// # Example
/// ```rust
/// use std::fs;
/// use std::env::var;
/// use mac_notifications::misc::make_download_bounce;
///
/// // select a file from Downloads folder
/// let home = var("HOME").unwrap();
/// let folder = format!("{}/Downloads", home);
/// let file = fs::read_dir(folder).unwrap().next().unwrap().unwrap().path();
/// let file = file.to_str().unwrap();
///
/// // just takes a str with path as input
/// make_download_bounce(file);
/// ```
pub fn make_download_bounce(filename: &str) {
    let name = NSNotificationName::from_str("com.apple.DownloadFileFinished");
    let filename = NSString::from_str(filename);

    unsafe {
        let notification_center = NSDistributedNotificationCenter::defaultCenter();
        notification_center.postNotificationName_object(name.deref(), Some(filename.deref()));
    }
}
/// Sets red badge on Application Icon in Dock. Consider that application needs to be packaged of course
/// # Example
/// ```rust
/// use mac_notifications::misc::set_badge;
///
/// set_badge(Some("5"));
/// ```
pub fn set_badge(content: Option<&str>) {
    let content = content.map(|s| NSString::from_str(s));
    let mtm = MainThreadMarker::new().expect("set_badge() must be on the main thread");
    let app = NSApplication::sharedApplication(mtm);
    unsafe {
        match content {
            Some(s) => app.dockTile().setBadgeLabel(Some(s.deref())),
            None => app.dockTile().setBadgeLabel(None),
        }
    }
}
