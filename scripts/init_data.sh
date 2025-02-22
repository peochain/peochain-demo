#!/usr/bin/env bash
# ----------------------------------------------------------------------------
# PEOCHAIN-DEMO: INIT DATA SCRIPT
# ----------------------------------------------------------------------------
# This script demonstrates initial data setup for the PeoChain testnet:
# creating accounts, deploying example smart contracts, setting validators, etc.
#
# PRINCIPLES:
# - SRP: focuses on bootstrap operations only.
# - OCP: new initialization routines can be added without changing the script’s core.
# - DRY & KISS: re-usable function calls to the Go API or direct container exec.
# ----------------------------------------------------------------------------

set -euo pipefail

echo "==> Initializing PeoChain testnet data..."

# Example: use the API container to create initial validator accounts
# This block simulates calls to the Go API endpoints
# or directly calls Docker container commands.
# (Adjust container name or endpoints as needed.)
curl -X POST -H "Content-Type: application/json" \
    -d '{"node_type":"consensus","validator_id":"validator_001"}' \
    http://localhost:8080/start-node || true

# Example: deploy an ERC-20 contract (stub)
curl -X POST -H "Content-Type: application/json" \
    -d '{"contract_type":"ERC20","parameters":"{\"name\":\"DemoToken\",\"symbol\":\"DMT\"}"}' \
    http://localhost:8080/deploy-contract || true

echo "==> Data initialization complete."
