# Tests

## Notes

`tokio` spins up a new async runtime every time at the beginning of each test case and shutdown at the end of each test case the `spawn_app()` function therefore only survives as long as the runtime.
