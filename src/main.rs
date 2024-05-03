use std::net::TcpListener;

use email_newsletter_api::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let port_number = configuration.application_port;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number))
        .unwrap_or_else(|_| panic!("Can't bind to port {} at localhost", port_number));

    // Move the error up the call stack
    // otherwise await for the HttpServer
    startup::run(listener)?.await
}
