# Technical Write Up

## Routes

- `health_check`: returns a HTTP code 200 if the server is up and running. Response body is empty.

## The `tokio` Async Runtime

`tokio` will be the engine in charge of driving Futures to completion

- `#[tokio::main]` macro is just a builder to help building the runtime. It is basically saying, we want this async function to be run.
- `tokio::spawn(/*async task*/)` will spawn an async task to be run. We can continue executing other code concurrently while `task` runs in the background. If we need to wait for `task` to complete before proceeding, we can use `task.await` (which `#[tokio::test]` will take care for us in the mean time).

### Testing with `tokio`

- Each test has to be marked `#[tokio::test]`.
- This macro will build a new async runtime for that one test.
- By calling `spawn_app()`, we will get a API HttpServer running for the entire duration of the single test that the macro above belongs to.

## The Test Suite

- The OS will find an available port for the test suite to use.
