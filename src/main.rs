use std::net::TcpListener;

use email_newsletter_api::{configuration::get_configuration, startup};
use env_logger::Env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db_conn = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL");

    let port_number = configuration.application_port;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number))
        .unwrap_or_else(|_| panic!("Can't bind to port {} at localhost", port_number));

    // Move the error up the call stack
    // otherwise await for the HttpServer
    startup::run(listener, db_conn)?.await
}
