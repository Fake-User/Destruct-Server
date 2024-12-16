# syntax = docker/dockerfile:1
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/destruct-server ./app

EXPOSE 3000
CMD ["./app"]
