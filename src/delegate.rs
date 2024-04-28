use crate::NotificationResponse;
use objc2::msg_send_id;
use objc2::mutability::MainThreadOnly;
use objc2::rc::Id;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{declare_class, ClassType, DeclaredClass};
use objc2_foundation::{
    MainThreadMarker, NSUserNotification, NSUserNotificationCenter,
    NSUserNotificationCenterDelegate,
};

type Callback = dyn Fn(String, NotificationResponse);
pub(super) struct State {
    callback: Box<Callback>,
}

declare_class! {
    pub(super) struct RustNotificationDelegate;

    unsafe impl ClassType for RustNotificationDelegate {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
        const NAME: &'static str = "RustNotificationDelegate";
    }

    impl DeclaredClass for RustNotificationDelegate {
        type Ivars = State;
    }

    unsafe impl NSObjectProtocol for RustNotificationDelegate {}

    unsafe impl NSUserNotificationCenterDelegate for RustNotificationDelegate {
        #[method(userNotificationCenter:didActivateNotification:)]
        fn did_activate(
            &self,
            _center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) {
            let response = NotificationResponse::from_dictionary(notification);

            let id = unsafe { notification.identifier() };
            match id {
                Some(id) => self.ivars().callback.as_ref()(id.as_ref().to_string(), response),
                None => eprintln!("Notification has no identifier! This should never happen! Do you have another application providing notifications for the same app name?")
            }
        }
    }
}

impl RustNotificationDelegate {
    pub fn new<F>(callback: F) -> Id<Self>
    where
        F: Fn(String, NotificationResponse) + 'static,
    {
        let this = MainThreadMarker::new().unwrap().alloc().set_ivars(State {
            callback: Box::new(callback),
        });

        let delegate: Id<Self> = unsafe { msg_send_id![super(this), init] };
        unsafe {
            let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
            notification_center.setDelegate(Some(ProtocolObject::from_ref(delegate.as_ref())));
        }
        delegate
    }
}

impl Drop for RustNotificationDelegate {
    fn drop(&mut self) {
        unsafe {
            let notification_center = NSUserNotificationCenter::defaultUserNotificationCenter();
            notification_center.setDelegate(None);
        }
    }
}
