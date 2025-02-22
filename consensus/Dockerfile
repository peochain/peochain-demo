# ------------------------------------------------------------------
# PEOCHAIN-DEMO: CONSENSUS DOCKERFILE
# ------------------------------------------------------------------
# This Dockerfile creates a container for the PoSyg + DCS consensus node.
# It uses the official Rust image to ensure consistent builds.
#
# PRINCIPLES:
# - SRP: This Dockerfile only handles the consensus component.
# - DRY & KISS: Minimally installs dependencies.

FROM rust:1.68 as builder

# Create a new directory for building the consensus node
WORKDIR /app

# Copy Cargo.toml and source code
COPY Cargo.toml .
COPY src ./src
COPY tests ./tests

# Build the project, output a release binary
RUN cargo build --release

# Production container
FROM debian:stable-slim
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/consensus_node /usr/local/bin/consensus_node

# Expose typical P2P port (customize as needed)
EXPOSE 30303

# Command to run the consensus node
ENTRYPOINT ["consensus_node"]
