use objc2::msg_send_id;
use objc2::mutability::InteriorMutable;
use objc2::rc::Id;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{declare_class, ClassType, DeclaredClass};
use objc2_foundation::{
    MainThreadMarker, NSUserNotification, NSUserNotificationCenter,
    NSUserNotificationCenterDelegate,
};

#[derive(Debug, Default)]
pub(super) struct State {}

declare_class! {
    pub(super) struct RustNotificationDelegate;

    unsafe impl ClassType for RustNotificationDelegate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "RustNotificationDelegate";
    }

    impl DeclaredClass for RustNotificationDelegate {
        type Ivars = State;
    }

    unsafe impl NSObjectProtocol for RustNotificationDelegate {}

    unsafe impl NSUserNotificationCenterDelegate for RustNotificationDelegate {
        #[method(userNotificationCenter:didActivateNotification:)]
        fn user_notification_center_did_activate_notification(
            &self,
            _center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) {
            println!("Notification activated: {:?}", notification);
        }
    }
}

impl RustNotificationDelegate {
    pub fn new() -> Id<Self> {
        let this = MainThreadMarker::new().unwrap().alloc().set_ivars(State {
            ..Default::default()
        });
        unsafe { msg_send_id![super(this), init] }
    }
}
