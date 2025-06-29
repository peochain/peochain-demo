# PeoChain Demo

PeoChain Demo is a high-performance, security-focused blockchain test network implemented in Rust. It serves as a reference implementation for the PeoChain architecture, which combines a novel **PoSyg + DCS** consensus mechanism with a compatible **EVM** module. The design prioritizes memory safety, execution speed, and deterministic execution, making it a robust platform for decentralized applications.

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.14908526.svg)](https://doi.org/10.5281/zenodo.14908526)
[![Bitcointalk Thread](https://img.shields.io/badge/Bitcointalk-Thread-blue?style=for-the-badge)](https://bitcointalk.org/index.php?topic=5532958.msg65092666#msg65092666)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Key Highlights

- **Rust-Based for Safety and Performance**: Core modules are written in idiomatic Rust, leveraging its ownership model and type system to guarantee memory safety. The codebase contains **zero `unsafe` blocks** in all critical network message paths.
- **Advanced Consensus (PoSyg + DCS)**: A unique consensus algorithm designed for high-throughput and low-latency transactions, featuring a Dynamic Contribution Scoring (DCS) system to incentivize validators.
- **EVM Compatibility**: An integrated EVM module allows for the deployment and execution of existing Ethereum smart contracts without modification.
- **Rigorous Security Audits**: The system has undergone extensive memory safety and message validation audits. Key improvements include:
    - **Bounded Inputs**: Strict size limits on all network inputs, including proofs (64KB), blocks (8MB), and transactions (32KB), to mitigate DoS attacks.
    - **Property-Based Testing**: `proptest` is used for fuzzing critical components like proof verification and block validation to ensure resilience against malformed inputs.
    - **Integer Overflow Protection**: All arithmetic operations in critical financial paths are checked to prevent overflow vulnerabilities.
- **Mobile-First Bridge**: The architecture includes a cross-chain bridge designed for seamless crypto-to-mobile money conversions.

For a detailed overview of the security enhancements, see the [Memory Safety Implementation Report](MEMORY_SAFETY_REPORT.md).

---

## Repository Structure

```plaintext
peochain-demo/
├── consensus/            # Rust code for the PoSyg + DCS consensus algorithm
├── evm/                  # EVM module for smart contract execution
├── bridge/               # Cross-chain bridge logic
├── api/                  # Go-based REST API for node management and user endpoints
├── scripts/              # Deployment and network management scripts
├── docs/                 # Architectural documentation and project roadmap
├── ci/                   # CI/CD configurations (e.g., GitHub Actions)
├── docker-compose.yml    # Docker orchestration for a complete test network
└── Makefile              # High-level commands for build, test, and deploy
```

---

## Getting Started

### Prerequisites
- **Rust (1.68+ recommended)**
- **Go (1.19+ recommended)**
- **Docker & Docker Compose**
- A compatible Linux distribution (e.g., Fedora 41, Ubuntu 22.04)

### Building and Testing

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/peochain/peochain-demo.git
    cd peochain-demo
    ```

2.  **Build All Components**:
    This command compiles the Rust modules (consensus, evm, bridge) and the Go API.
    ```bash
    make build
    ```

3.  **Run All Tests**:
    This executes unit and integration tests for all modules, including property-based fuzzing tests.
    ```bash
    make test
    ```

### Local Deployment

1.  **Deploy the Test Network**:
    This script builds and starts all services (consensus-node, evm-node, bridge-service, api-service) in detached mode using Docker Compose.
    ```bash
    ./scripts/deploy_testnet.sh
    ```

2.  **Initialize Network Data**:
    This script sets up initial accounts, deploys example smart contracts, and configures the network validators.
    ```bash
    ./scripts/init_data.sh
    ```

3.  **Verify Network Status**:
    Check the health of the API service, which indicates the status of the underlying nodes.
    ```bash
    curl http://localhost:8080/health
    ```

---

## Contributing

We welcome contributions that improve PeoChain. This includes feature enhancements, bug fixes, performance optimizations, and documentation.

1.  **Fork** the repository.
2.  Create a new branch for your feature (`feature/my-new-feature`).
3.  Commit your changes and push them to your fork.
4.  Open a **Pull Request** with a detailed description of your changes.

## Citing This Work

If you use this software in your research, please cite it using the information in `CITATION.cff`.

---

## Author & License

This project is authored by **PEOCHAIN GmBH** and is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
