use objc2_foundation::{NSUserNotification, NSUserNotificationActivationType};

/// Response from the Notification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationResponse {
    /// No interaction has occured
    None,
    /// User clicked on an action button with the given name
    ActionButton(String),
    /// User clicked on the close button with the given name
    CloseButton(String),
    /// User clicked the notification directly
    Click,
    /// User submitted text to the input text field
    Reply(String),
}

impl NotificationResponse {
    pub(crate) fn from_dictionary(notification: &NSUserNotification) -> Self {
        return unsafe {
            match notification.activationType() {
                NSUserNotificationActivationType::None => Self::None,
                NSUserNotificationActivationType::ActionButtonClicked => {
                    return Self::ActionButton("todo".to_string());
                }
                NSUserNotificationActivationType::ContentsClicked => Self::Click,
                NSUserNotificationActivationType::Replied => {
                    Self::Reply(notification.response().unwrap().string().to_string())
                }
                _ => unreachable!(
                    "Unknown notification response: {:?}",
                    notification.activationType()
                ),
            }
        };
    }
}
