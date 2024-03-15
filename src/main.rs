use std::{error::Error, future::pending};
use std::fmt::DebugList;
use async_std::task;

use zbus::Connection;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bus = Connection::system().await.unwrap();
    
    let out = bus.call_method(
        Some("org.freedesktop.hostname1"),
        "/org/freedesktop/hostname1",
        Some("org.freedesktop.DBus.Properties"),
        "Get",
        &("org.freedesktop.hostname1", "Hostname")
    ).await?.body();
        
    let body: &str = out.().deserialize().unwrap();

    println!("{}", body);

    Ok(())
}

