# Technical Write Up

## Routes

- `health_check`: GET - returns a HTTP code 200 if the server is up and running. Response body is empty.
- `subscriptions` POST - returns a HTTP code 200 if the user successfully subscribed to our email newsletter service. 400 if data is missing or invalid.

## Other topics

- [Tracing](./tracing.md)
- [Database (PostgreSQL)](./database.md)
- [Testing](./technical_write_up.md)
- [Actic-web](./actix_web.md)
- [tokio](./tokio.md)
