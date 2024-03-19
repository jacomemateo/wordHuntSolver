use tokio;
use dbus_tokio::connection;
use dbus_crossroads::Crossroads;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, conn) = connection::new_system_sync().expect("D-Bus connection failed");

    let _handle = tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    conn.request_name("com.example.dbustest", false, true, false).await?;

    let mut cr = Crossroads::new();

    Ok(())
}
