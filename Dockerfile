FROM rust:1.85.0 as builder
WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/ src/
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/bt_discord_bot /usr/local/bin/bt_discord_bot
CMD ["bt_discord_bot"]