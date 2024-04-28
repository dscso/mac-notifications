use mac_notifications::*;

fn main() {
    let mut provider = NotificationProvider::new("terminal");

    provider.set_callback(|id, response| {
        println!("notification activated {}: {:?}", id, response);
    });

    let image = String::from("https://avatars.githubusercontent.com/u/6866008?v=4");

    let id = Notification::new()
        .reply(true)
        .title("title")
        .subtitle("This notification will be deleted in ~5sec... Interact with it before that!")
        .image(image.as_ref())
        .delivery_date(std::time::SystemTime::now() - std::time::Duration::from_secs(100))
        .send()
        .expect("TODO: panic message");

    for _ in 0..50 {
        provider.run_main_loop_once();
    }
    println!("all notifications: {:?}", provider.get_all_notifications());
    println!("deleting old notification(s)...");
    provider.delete(id.as_str());

    println!("all notifications: {:?}", provider.get_all_notifications());
    for _ in 0..50 {
        provider.run_main_loop_once();
    }
}
