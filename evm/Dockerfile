# ------------------------------------------------------------------
# PEOCHAIN-DEMO: EVM MODULE DOCKERFILE
# ------------------------------------------------------------------
# This Dockerfile creates a container for the EVM compatibility node.
# It uses the official Rust image for a consistent build environment.

FROM rust:1.68 as builder

WORKDIR /app

COPY Cargo.toml .
COPY src/lib.rs ./src/lib.rs
COPY src ./src
COPY tests ./tests
COPY contracts ./contracts

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app

COPY --from=builder /app/target/release/evm_node /usr/local/bin/evm_node

EXPOSE 30304

ENTRYPOINT ["evm_node"]
