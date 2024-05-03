mod test_utils;

use test_utils::spawn_app;

use email_newsletter_api::configuration::{self, get_configuration};
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let server_address = spawn_app();

    let configuration = get_configuration().expect("Failed to read configuration");

    let postgres_connection_string = configuration.database.connection_string();

    let connection = PgConnection::connect(&postgres_connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    let body = "name=le%20test&email=le_test%40gmail.com";

    let response = client
        .post(&format!("{}/subscribe", &server_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute subscribe request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let server_address = spawn_app();

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &server_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute subscribe request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API failed with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
