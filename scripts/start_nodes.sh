#!/usr/bin/env bash
# ----------------------------------------------------------------------------
# PEOCHAIN-DEMO: START NODES SCRIPT
# ----------------------------------------------------------------------------
# This script is used to sequentially or concurrently start multiple containers,
# if you prefer not to rely on the single 'docker-compose up' approach.
# It can also handle scaling the number of nodes (e.g., multiple validators).
#
# PRINCIPLES:
# - SRP: focuses on starting containers or services in an orchestrated manner.
# - DRY & KISS: keeps logic minimal, delegates scaling or advanced logic to Docker Compose.
# ----------------------------------------------------------------------------

set -euo pipefail

echo "==> Starting PeoChain nodes..."

# If you'd like to start each container individually:
docker-compose up -d consensus-node
docker-compose up -d evm-node
docker-compose up -d bridge-service
docker-compose up -d api-service

echo "==> All nodes started. Use 'docker-compose ps' to check status."
