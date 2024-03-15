use std::error::Error; 
use zbus::{Connection, zvariant::Value, Proxy};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::system().await.unwrap();
    
    let proxy= Proxy::new(
        &conn,
        "org.freedesktop.hostname1",
        "/org/freedesktop/hostname1",
        "org.freedesktop.DBus.Properties"
    ).await?;        


    let body= proxy.call_method("Get", &("org.freedesktop.hostname1", "Hostname") ).await?.body();
    let body: Value = body.deserialize()?;        

    println!("The hostname is {}", body);

    Ok(())
}
