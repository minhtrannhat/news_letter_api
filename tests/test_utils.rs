use email_newsletter_api::{
    configuration::{get_configuration, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[allow(clippy::let_underscore_future)]
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    /* Spawn a app server with a TcpListener bound to localhost:<random port>
     *
     *  Returns a valid IPv4 string (i.e localhost:8080)
     */
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    let mut configuration = get_configuration().expect("Failed to read configuration");

    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_conn_pool = configure_test_database(&configuration.database).await;

    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter_api::startup::run(listener, db_conn_pool.clone())
        .expect("Failed to bind address");

    /* `tokio::spawn(/*async task*/)` will spawn an async task to be run.
    We can continue executing other code concurrently while `task` runs in the background.
    If we need to wait for `task` to complete before proceeding,
    we can use `task.await`
    (which `#[tokio::test]` will take care for us in the mean time).*/
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: db_conn_pool,
    }
}

pub async fn configure_test_database(db_config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&db_config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let conn_pool = PgPool::connect(&db_config.connection_string())
        .await
        .expect("Failed to connect to PostgreSQL pool");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database");

    conn_pool
}
