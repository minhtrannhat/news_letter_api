use std::net::TcpListener;

use email_newsletter_api::telemetry::{get_subscriber, init_subscriber};
use email_newsletter_api::{configuration::get_configuration, startup};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let subscriber = get_subscriber(
        "email_newsletter_api".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let db_conn = PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to PostgreSQL");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))
    .unwrap_or_else(|_| {
        panic!(
            "Can't bind to port {} at localhost",
            configuration.application.port
        )
    });

    // Move the error up the call stack
    // otherwise await for the HttpServer
    startup::run(listener, db_conn)?.await
}
