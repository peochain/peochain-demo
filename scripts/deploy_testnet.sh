#!/usr/bin/env bash
/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: DEPLOY TESTNET SCRIPT
 * ----------------------------------------------------------------------------
 * This script builds and deploys the PeoChain testnet containers via Docker
 * and Docker Compose. It is intended as a single-entry command for developers
 * or investors to see the network in action quickly.
 *
 * PRINCIPLES:
 * - SRP: this script focuses solely on deployment.
 * - OCP & LSP: updates to Docker services or new containers can be added
 *              with minimal script adjustments.
 * - DRY & KISS: re-uses docker-compose, minimal logic here.
 */

set -euo pipefail

echo "==> Building and deploying PeoChain testnet..."

# Optional: Pull latest images if you rely on remote images
# docker-compose pull

# Build and start in detached mode
docker-compose up --build -d

echo "==> Testnet is deployed. Use 'docker-compose logs' to view logs."
echo "==> Visit API service on http://localhost:8080 (default) for status checks."
