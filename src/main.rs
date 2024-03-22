use dbus::{blocking::Connection, Error};
use std::time::Duration;
mod bluez_constants;

fn discover_device(connection: &dbus::blocking::Connection) {
    let adapter_path = format!("{}{}", bluez_constants::BLUEZ_NAMESPACE, bluez_constants::ADAPTER_NAME);

    println!("{}", adapter_path);

    let adapter_object = connection.with_proxy(bluez_constants::BLUEZ_SERVICE_NAME, adapter_path, Duration::from_millis(5000));

    
}

fn main() {
    let connection = match Connection::new_system() {
        Ok(conn) => conn,
        Err(err) => panic!("Error: {}", err)   
    };

    discover_device(&connection);
}
