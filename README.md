# Email Newsletter API

- An API to enable blog visitors subcribe to the newsletter so that they can receive updates when new content is published on the blog.

## Development setup

- Install [cargo-watch](https://crates.io/crates/cargo-watch).
- Open your favorite text editor (better be neovim or emacs).
- Run `cargo watch -x check -x test -x run` to lint, test and run the binary as soon as you make a change to the file.
- Bonus: install and use `mold`, a very fast linker that can link your Rust binary _blazingly fast_.

## Notable Dependencies

- `actix-web`: Most popular Rust web framework
- `serde`: Data structure serialization/deserialization
- `tokio`: Async Runtime
- `tracing`: Alternative to traditional logging
- `sqlx`: SQL toolkit for Rust. Offers compile-time SQL checked queries

## [Technical Write Up](./docs/technical_write_up.md)
