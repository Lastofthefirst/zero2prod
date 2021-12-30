use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to listen port...");
    run(listener)?.await
}
