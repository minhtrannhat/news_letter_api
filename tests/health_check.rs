mod test_utils;

use test_utils::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let server_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &server_address))
        .send()
        .await
        .expect("Failed to execute health_check request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
