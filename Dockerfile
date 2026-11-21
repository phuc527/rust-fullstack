
FROM rust:1.89-bookworm AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust_axum_observability /usr/local/bin/api

USER 10001:10001

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/api"]
