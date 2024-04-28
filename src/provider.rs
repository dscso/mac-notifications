use std::ops::Deref;
use objc2::ClassType;
use objc2::rc::Id;
use objc2::runtime::NSObjectProtocol;
use objc2_foundation::{MainThreadMarker, NSDate, NSDefaultRunLoopMode, NSRunLoop, NSString, NSUserNotificationCenter};
use crate::delegate::RustNotificationDelegate;
use crate::{Notification, NotificationResponse};

mod sys {
    use objc2_foundation::NSString;

    #[link(name = "notification")]
    extern "C" {
        pub fn init(app_name: *const NSString);
    }
}

/// the main struct for the notification provider
/// # Example
/// ```rust
/// use notifications::{Notification, NotificationProvider};
///
/// let mut provider = NotificationProvider::new("Terminal");
/// // callback for notification interaction
/// provider.set_callback(|id, resp| {
///    println!("Notification {} clicked: {:?}", id, resp);
/// });
/// // sends notification
/// Notification::new()
///     .title("Hello")
///     .reply(true)
///     .send().unwrap();
/// // run the main loop. If some notification gets interacted with, the callback will be called
/// for _ in 0..50 {
///     provider.run_main_loop_once();
/// }
/// ```
pub struct NotificationProvider {
    delegate: Option<Id<RustNotificationDelegate>>,
    center: Id<NSUserNotificationCenter>,
}

impl NotificationProvider {
    /// Creates a new NotificationProvider with the name of the application e.g. "Terminal"
    /// # Panics
    //  Panics if the callback is not called on the main thread
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

    /// This callback gets called when a notification was interacted with
    pub fn set_callback<F>(&mut self, callback: F)
        where
            F: Fn(String, NotificationResponse) + 'static,
    {
        let delegate = RustNotificationDelegate::new(callback);
        self.delegate = Some(delegate);
    }
    /// Runs the main loop for .1 seconds
    pub fn run_main_loop_once(&self) {
        run_main_loop_once();
    }
    /// Returns a vector of all notifications
    /// # Example
    /// ```rust
    /// use notifications::{Notification, NotificationProvider};
    ///
    /// let mut provider = NotificationProvider::new("Terminal");
    /// Notification::new().title("Hello").send().unwrap();
    ///
    /// let notifications = provider.get_all_notifications();
    /// for notification in notifications {
    ///    println!("{:?}", notification);
    /// }
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
    /// Deletes a notification by its identifier
    /// # Example
    /// ```rust
    /// use notifications::{Notification, NotificationProvider};
    ///
    /// let mut provider = NotificationProvider::new("Terminal");
    ///
    /// let id = Notification::new().title("Hello").send().unwrap();
    /// // wait 1 sec
    /// for _ in 0..10 {
    ///    provider.run_main_loop_once();
    /// }
    /// provider.delete(&id);
    /// let notifications = provider.get_all_notifications();
    /// ```
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
    /// Deletes all notifications
    /// # Example
    /// ```rust
    /// use notifications::{Notification, NotificationProvider};
    ///
    /// let mut provider = NotificationProvider::new("Terminal");
    ///
    /// Notification::new().title("Hello").send().unwrap();
    /// Notification::new().title("Hello2").send().unwrap();
    ///
    /// provider.delete_all();
    /// let notifications = provider.get_all_notifications();
    /// assert_eq!(notifications.len(), 0);
    /// ```
    pub fn delete_all(&self) {
        MainThreadMarker::new().expect("delete_all() must be on the main thread");
        unsafe {
            self.center.removeAllDeliveredNotifications();
        }
    }
}
/// Runs the main loop for .1 seconds
/// # Panics
/// Panics if the function is not called on the main thread
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
