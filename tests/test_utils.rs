use std::net::TcpListener;

#[allow(dead_code)]
#[allow(clippy::let_underscore_future)]
pub fn spawn_app() -> String {
    /* Spawn a app server with a TcpListener bound to localhost:<random port>
     *
     *  Returns a valid IPv4 string (i.e localhost:8080)
     */
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter_api::run(listener).expect("Failed to bind address");

    /* `tokio::spawn(/*async task*/)` will spawn an async task to be run.
    We can continue executing other code concurrently while `task` runs in the background.
    If we need to wait for `task` to complete before proceeding,
    we can use `task.await`
    (which `#[tokio::test]` will take care for us in the mean time).*/
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
