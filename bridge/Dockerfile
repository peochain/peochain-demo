# ------------------------------------------------------------------
# PEOCHAIN-DEMO: BRIDGE DOCKERFILE
# ------------------------------------------------------------------
# This Dockerfile creates a container for the cross-chain bridge module.

FROM rust:1.68 as builder

WORKDIR /app

COPY Cargo.toml .
# If using src/lib.rs
COPY src ./src
COPY tests ./tests

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app

COPY --from=builder /app/target/release/bridge /usr/local/bin/bridge
# If there's no binary named 'bridge' (pure library), you can build a small CLI or remove this step.

EXPOSE 30305

ENTRYPOINT ["bridge"]
