# Tracing

Logs only record events, but traces record all that plus the start and end events. This makes following traces much more logical.

## The flow

- Create a new span, attach some values to it. These values are key-value pairs.
- We explicitly step into the span with `.enter()`.
- `.enter()` returns an instance of Entered, a guard: as long the guard variable is not dropped all downstream spans and log events will be registered as children of the entered span. And then the compiler will drop these for us.

## Notations
- enter the span (->);
- We exit the span (<-);
- We finally close the span (--).

## Instrumenting

When we think about an async task, the async executor (in our case, the `tokio` async runtime) will have to poll the futures multiple times to drive that future to completion. And while that future is idle, we will do work on other futures.

We then need to think about how to not mix the spans of the futures. This is where instrument comes in. It is an extension trait for futures. `Instrument::instrument` does exactly what we want: enters the span we pass as argument every time self, the future, is polled; it exits the span every time the future is parked.

## Notes

- We can enter and exit the span multiple times. But can only close once. This is good for async tasks as we will enter and resume async tasks.
