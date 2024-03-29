use std::net::TcpListener;

use email_newsletter_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");

    // Move the error up the call stack
    // otherwise await for the HttpServer
    run(listener)?.await
}
