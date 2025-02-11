# Neuro Worker

Neuro Worker is a proxy server used to access the Yandex Neuro API. This can be used with [neurojs](https://github.com/FOSWLY/neurojs)

## Supports

This worker supports:

- Official summarize API (get sharing-url)

  prefix: `/th`

  `POST /th/api/sharing-url`

- Browser summarize API (create session and summarize video)

  prefix: `/browser`

  `POST /browser/session/create`

  `POST /browser/video-summary/generation`

- Web summarize API (300) with YaHMAC or Cookies

  prefix: `/th`

  With YaHMAC:

  `POST /th/api/neuro/generation`

  With Cookies:

  `POST /th/api/generation`

- Self health. Check if it's working

  `GET /health`

## Why?

Worker is needed if:

1. You have blocked Yandex servers
2. You need to bypass CORS

## How to run

To run your own instance:

1. Install [Rust 1.75+](https://www.rust-lang.org/learn/get-started)

2. (Optional) Run for developing:

   2.1. Install `cargo watch`:

   ```bash
   cargo install cargo-watch
   ```

   2.2. Run live server:

   ```bash
   cargo watch -x run
   ```

3. Run for Production:

   3.1. Build:

   ```bash
   cargo build --release
   ```

   3.2. Run a `neuro-worker` file:

   ```bash
   ./target/release/neuro-worker
   ```
