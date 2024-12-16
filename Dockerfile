# syntax = docker/dockerfile:1
FROM rust:stable-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:slim
WORKDIR /app
COPY --from=builder /app/target/release/destruct-server ./app

EXPOSE 3000
CMD ["./app"]
