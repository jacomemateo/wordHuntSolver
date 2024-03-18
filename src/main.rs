use std::time::Duration;
use dbus::blocking::Connection;
use dbus::Message;
use dbus::message::MatchRule;

// This programs implements the equivalent of running the "dbus-monitor" tool
fn main() {
    // First open up a connection to the session bus.
    let conn = Connection::new_system().expect("D-Bus connection failed");

    // Match rule, it left empty it will get every single message and act as dbus-monitor
    let rule = MatchRule::new().with_interface("com.example.greeting");

    conn.add_match(rule, |_: (), _, msg| {
        handle_message(&msg);
        true
    }).expect("add_match failed");

    loop {
        conn.process(Duration::from_millis(1000)).unwrap();
    };
}

fn handle_message(msg: &Message) {
    println!("{}", msg.get1::<String>().unwrap() );
}
