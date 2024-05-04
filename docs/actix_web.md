# Actix-web Notes

- Actix-web runtime model: Spins up a worker process per CPU core. Each worker runs its own copy of of the application provided in the closure that `HttpServer` takes as an argument. Therefore, everything in the closure (including `app_data` below) has to implement the `Clone` trait.

## `app_data`

When we set up our web app, we can attach a resource to the app with `app_data`. And then our route handlers can access this application data with `HttpRequest::app_data()`. It is similar to FastAPI's dependency injection.

In our app, we want to inject a `db_conn` to the route handlers, so that these routes can handle PostgreSQL read/write.

Since the database connection is a TCP connection that is NOT `Clone`-able. We use Rust's `Arc` (Atomic Reference Counter) as a wrapper around this connection. Each instance of our web app, instead of getting a raw TCP connection to the PostgreSQL database, will be given the pointer to the memory region of `db_conn`.

