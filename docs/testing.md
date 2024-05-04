# Testing Notes

## The Test Suite

- The OS will find an available port for the test suite to use.
- We use the same PostgreSQL database instance for both testing and production environment (might bite us in the ass later ?).

## Testing with `tokio`

- Each test has to be marked `#[tokio::test]`.
- This macro will build a new async runtime for that one test.
- By calling `spawn_app()`, we will get a API HttpServer running for the entire duration of the single test that the macro above belongs to.
