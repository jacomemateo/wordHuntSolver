use std::sync::Arc;
use tokio::runtime::Runtime;
use zbus::{Connection, ProxyBuilder, Proxy};

#[tokio::main]
async fn main() -> Result<(), zbus::Error> {
    // Connect to the system bus
    let connection = Connection::system().await?;

    // Create a proxy builder for the signal interface
    let proxy_builder = ProxyBuilder::new(&connection)
        .destination("com.example.greeting")?
        .path("/")?
        .interface("com.example.greeting.GreetingSignal")?;

    // Create a proxy for receiving signals
    let proxy: Proxy<'_> = proxy_builder.build().await?;

    // Subscribe to signals
    //proxy.subscribe().await?;

    // Receive signals in a loop
    loop {
        // Wait for a signal
        let signal = proxy.receive_signal("GreetingSignal").await?;

        // Extract the string parameter from the signal
        let greeting: String = signal.body()?;

        // Print the received greeting
        println!("Received greeting: {}", greeting);
    }
}
