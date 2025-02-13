FROM rust:1.84.1-alpine3.21 as builder

WORKDIR /usr/src/app

RUN apk add libc-dev openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/release/neuro-worker /usr/local/bin/neuro-worker
ENV SERVICE_HOST=0.0.0.0

CMD ["neuro-worker"]