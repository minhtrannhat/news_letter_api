# tokio Notes

## The `tokio` Async Runtime

`tokio` will be the engine in charge of driving Futures to completion

- `#[tokio::main]` macro is just a builder to help building the runtime. It is basically saying, we want this async function to be run.
- `tokio::spawn(/*async task*/)` will spawn an async task to be run. We can continue executing other code concurrently while `task` runs in the background. If we need to wait for `task` to complete before proceeding, we can use `task.await` (which `#[tokio::test]` will take care for us in the mean time).
