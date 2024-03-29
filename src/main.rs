use email_newsletter_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    // Move the error up the call stack
    // otherwise await for the HttpServer
    run()?.await
}
