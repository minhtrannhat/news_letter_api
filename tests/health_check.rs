use std::{fmt::format, net::TcpListener};
// tokio spins up a new async runtime every time 
// at the beginning of each test case and shutdown at
// the end of each test case
// the spawn_app() function therefore only survives as long as the runtime
#[tokio::test]
async fn health_check_works(){
    let server_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client.get(&format!("{}/health_check", &server_address)).send().await.expect("Failed to execute health_check request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter_api::run(listener).expect("Failed to bind address");

    // run() returns an instance of HttpServer that will run forever.
    // We don't want this behavior
    // Therefore we want to spawn our server, run our test logic
    // and then tear down the entire test suite
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
