use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::{PgConnection, Connection};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
   

    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind(address).expect("Failed to listen port...");
    run(listener, connection)?.await
}
