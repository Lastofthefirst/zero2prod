# We use the latest Rust stable release as base image
FROM rust:1.58.1

WORKDIR /app

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]