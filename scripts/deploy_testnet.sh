#!/usr/bin/env bash
# ----------------------------------------------------------------------------
# PEOCHAIN-DEMO: DEPLOY TESTNET SCRIPT
# ----------------------------------------------------------------------------
# This script builds and deploys the PeoChain testnet using Docker Compose.
# (SRP, OCP, LSP, DIP, DRY, KISS) # If you want to mention those principles, do so in a comment
# ----------------------------------------------------------------------------

set -euo pipefail

echo "==> Building and deploying PeoChain testnet..."

docker-compose up --build -d

echo "==> Testnet deployed. Use 'docker-compose logs' to view logs."
