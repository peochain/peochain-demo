# ---------------------
# PEOCHAIN-DEMO MAKEFILE
# ---------------------
# This Makefile provides high-level commands to build, test,
# and deploy the PeoChain demo project.

.PHONY: build test deploy

build:
	@echo "==> Building Rust components..."
	cd consensus && cargo build
	cd evm && cargo build
	cd bridge && cargo build
	@echo "==> Building Go API..."
	cd api && go build -o peochain-api .

test:
	@echo "==> Testing Rust components..."
	cd consensus && cargo test
	cd evm && cargo test
	cd bridge && cargo test
	@echo "==> Testing Go API..."
	cd api && go test ./...

deploy:
	@echo "==> Deploying with Docker Compose..."
	docker-compose up --build -d
