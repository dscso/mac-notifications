use objc2::msg_send_id;
use objc2::mutability::MainThreadOnly;
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

            unsafe {
                match notification.response() {
                    Some(str) => {
                        println!("Notification activated with response: {:?}", str.as_ref());
                    }
                    None => {
                        println!("Notification activated: {:?}", notification);
                    }
                }
            }
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
    pub fn get(mtm: MainThreadMarker) -> Id<Self> {
        let app = unsafe { NSUserNotificationCenter::defaultUserNotificationCenter() };
        let delegate =
            unsafe { app.delegate() }.expect("a delegate was not configured on the application");
        if delegate.is_kind_of::<Self>() {
            // SAFETY: Just checked that the delegate is an instance of `ApplicationDelegate`
            unsafe { Id::cast(delegate) }
        } else {
            panic!("tried to get a delegate that was not the one Winit has registered")
        }
    }
}

impl Drop for RustNotificationDelegate {
    fn drop(&mut self) {
        println!("Dropping RustNotificationDelegate");
    }
}
