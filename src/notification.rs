use objc2::rc::Id;
use objc2::{msg_send, ClassType};
use objc2_app_kit::NSImage;
use objc2_foundation::{
    MainThreadMarker, NSDate, NSError, NSString, NSUserNotification, NSUserNotificationCenter,
    NSURL,
};
use std::fmt::Debug;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Notification {
    /// The unique identifier for the notification.
    identifier: String,
    /// The localized text that provides the notification’s primary description.
    title: Option<String>,
    /// The localized text that provides the notification’s secondary description.
    subtitle: Option<String>,
    /// launches because of the notification.
    image: Option<String>,
    /// The sound that plays when the system delivers the notification.
    sound: Option<String>,
    /// Delivery date
    delivery_date: Option<SystemTime>,
    /// Has reply button
    reply: bool,
}

impl From<&NSUserNotification> for Notification {
    fn from(notification: &NSUserNotification) -> Self {
        unsafe {
            Self {
                identifier: notification.identifier().unwrap().to_string(),
                title: notification.title().map(|s| s.to_string()),
                subtitle: notification.subtitle().map(|s| s.to_string()),
                sound: notification.soundName().map(|s| s.to_string()),
                reply: notification.hasReplyButton(),
                delivery_date: None,
                image: None,
            }
        }
    }
}
impl From<&Notification> for Id<NSUserNotification> {
    fn from(value: &Notification) -> Self {
        unsafe {
            let notification = NSUserNotification::new();

            let ns_str = NSString::from_str(value.identifier.as_str());
            notification.setIdentifier(Some(&ns_str));

            if let Some(title) = value.title.as_ref() {
                let ns_str = NSString::from_str(&title);
                notification.setTitle(Some(&ns_str));
            }

            if let Some(subtitle) = value.subtitle.as_ref() {
                let ns_str = NSString::from_str(&subtitle);
                notification.setSubtitle(Some(&ns_str));
            }
            if let Some(sound) = value.sound.as_ref() {
                let ns_str = NSString::from_str(&sound);
                notification.setSoundName(Some(&ns_str));
            }
            if let Some(delivery_date) = value.delivery_date {
                let timestamp = delivery_date
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let date = NSDate::dateWithTimeIntervalSince1970(timestamp as f64);
                notification.setDeliveryDate(Some(date.as_ref()));
            }
            if let Some(image) = value.image.as_ref() {
                let ns_str = NSString::from_str(&image);
                let ns_url = NSURL::URLWithString(&ns_str).unwrap();
                let ns_image = NSImage::initWithContentsOfURL(NSImage::alloc(), &ns_url).unwrap();
                let _: () = msg_send![notification.as_ref(), setContentImage:ns_image.as_ref()];
            }

            if value.reply {
                notification.setHasReplyButton(value.reply);
            }
            notification
        }
    }
}

impl Notification {
    pub fn send(self) -> Result<String, NotificationError> {
        MainThreadMarker::new().expect("send() must be on the main thread");
        let identifier = self.identifier.clone();
        unsafe {
            let notification = Id::<NSUserNotification>::from(&self);
            let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
            match self.delivery_date {
                Some(_) => notification_center.scheduleNotification(notification.as_ref()),
                None => notification_center.deliverNotification(notification.as_ref()),
            }
        }
        Ok(identifier)
    }
}
impl Notification {
    pub fn new() -> Self {
        Self {
            identifier: Uuid::new_v4().to_string(),
            ..Default::default()
        }
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn image(mut self, image: &str) -> Self {
        self.image = Some(image.to_string());
        self
    }

    pub fn sound(mut self, sound: &str) -> Self {
        self.sound = Some(sound.to_string());
        self
    }

    pub fn reply(mut self, reply: bool) -> Self {
        self.reply = reply;
        self
    }
    pub fn delivery_date(mut self, delivery_date: SystemTime) -> Self {
        self.delivery_date = Some(delivery_date);
        self
    }
}

#[derive(Clone)]
pub enum NotificationError {
    /// Error from the Objective C User Notifications framework
    NSError(Id<NSError>),
    /// Not supported for the current OS version
    NotSupported,
}

impl From<Id<NSError>> for NotificationError {
    fn from(value: Id<NSError>) -> Self {
        Self::NSError(value)
    }
}

impl Debug for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationError::NSError(ns_error) => f
                .debug_struct("NSError")
                .field("code", &ns_error.code())
                .field("domain", &ns_error.domain().to_string())
                .field("message", &ns_error.localizedDescription().to_string())
                .finish(),
            NotificationError::NotSupported => f.write_str("NotSupported"),
        }
    }
}
