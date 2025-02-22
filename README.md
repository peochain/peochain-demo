# PeoChain & PeoPay

> **Next-Generation “Layer Meta” Blockchain + Mobile Financial Ecosystem**

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.14908526.svg)](https://doi.org/10.5281/zenodo.14908526)

[![Bitcointalk Thread](https://img.shields.io/badge/Bitcointalk-Thread-blue?style=for-the-badge)](https://bitcointalk.org/index.php?topic=5532958.msg65092666#msg65092666)  

## Overview
PeoChain is an innovative blockchain designed with Rust for high performance, security, and sustainability. It features a unique **PoSyg + DCS** consensus mechanism and an **EVM** module (similar to Ethereum’s), making it easy for developers to deploy smart contracts. PeoPay complements PeoChain by offering a user-friendly, mobile-first platform targeting underbanked populations and anyone looking for fast, affordable finance solutions.

**Key Highlights**
- **Rust-Based Consensus**: High-speed, low-latency transactions with a synergy scoring system (DCS).
- **EVM Compatibility**: Deploy or integrate with existing Ethereum smart contracts.
- **Mobile Focus**: Seamless crypto-to-mobile money conversions, bridging the gap between decentralized blockchain technology and real-world usage.

---

## Repository Structure

```plaintext
peochain-demo/
├── consensus/
│   ├── src/
│   │   └── main.rs
│   ├── tests/
│   ├── Cargo.toml
│   └── Dockerfile
├── evm/
│   ├── src/
│   ├── contracts/
│   ├── tests/
│   ├── Cargo.toml
│   └── Dockerfile
├── bridge/
│   ├── src/
│   ├── tests/
│   ├── Cargo.toml
│   └── Dockerfile
├── api/
│   ├── src/
│   │   ├── main.go
│   │   └── handlers.go
│   ├── go.mod
│   └── Dockerfile
├── scripts/
│   ├── deploy_testnet.sh
│   ├── init_data.sh
│   └── start_nodes.sh
├── docs/
│   ├── architecture.md
│   ├── investor_pitch.md
│   └── roadmap.md
├── docker-compose.yml
├── Makefile
└── ci/
    └── github-actions.yml
```

- **consensus/**: Rust code implementing the PoSyg + DCS algorithm.
- **evm/**: EVM module for smart contract compatibility and execution.
- **bridge/**: Cross-chain bridge logic to connect external blockchains to PeoChain.
- **api/**: Go-based REST API providing services like node management, contract deployment, and user-facing endpoints.
- **scripts/**: Shell scripts for deploying and managing the test network.
- **docs/**: Contains architectural documentation, pitch decks, and project roadmaps.
- **docker-compose.yml**: Orchestration file to run all containers for a demo network.
- **Makefile**: Basic commands for build, test, and deploy.
- **ci/**: GitHub Actions or other CI/CD configuration.

---

## Getting Started

### Prerequisites
- **Rust (1.68+ recommended)**  
- **Go (1.19+ recommended)**  
- **Docker & Docker Compose**  
- **Fedora 41** (as a reference OS, though other platforms may work)

### Building & Testing

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/dkrizhanovskyi/peochain-demo.git
   cd peochain-demo
   ```

2. **Build All Services**:
   ```bash
   make build
   ```
   This compiles Rust modules (consensus, evm, bridge) and the Go API.

3. **Run Tests**:
   ```bash
   make test
   ```
   Runs integration and unit tests for all modules.

### Deployment

1. **Deploy with Docker Compose**:
   ```bash
   ./scripts/deploy_testnet.sh
   ```
   This script builds and starts all containers (consensus-node, evm-node, bridge-service, api-service) in detached mode.

2. **Initialize Data**:
   ```bash
   ./scripts/init_data.sh
   ```
   Sets up initial accounts, deploys example smart contracts, and configures validators.

3. **Check Status**:
   - Access the Go API at `http://localhost:8080/health` or `http://localhost:8080/status`.

---

## Contributing

We welcome contributions to improve PeoChain & PeoPay. Whether you’re adding new features, fixing bugs, or helping with documentation:

1. **Fork** the repository  
2. **Create** a new branch (`feature/my-new-feature`)  
3. **Commit** your changes and push  
4. **Open** a Pull Request, describing your modifications in detail.

---

## Community & Support

- **Discussions**: [GitHub Discussions](https://github.com/orgs/PeoPay/discussions)  
- **Bugs & Issues**: [GitHub Issues](https://github.com/orgs/PeoPay/issues)  
- **Official Support**: [support@peopay.io](mailto:support@peopay.io)

---

## Author

This project is initially authored and maintained by **Daniil Krizhanovskyi** (Blockchain Cryptographer, Smart Contracts Auditor). Feel free to connect on [LinkedIn](https://www.linkedin.com/in/dkrizhanovskyi-seceng/), [GitHub](https://github.com/dkrizhanovskyi), or via email at [dk.arecibo@proton.me](mailto:dk.arecibo@proton.me) for inquiries related to security, audits, or blockchain architecture.

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

### Tips
1. Adjust the **repository URL** to match your actual hosting location.
2. Update **community links** with real addresses (e.g., GitHub, Slack, Discord).
3. If you have a **license** file, mention it explicitly under a “License” heading.

## Linked Documents

- [SynLedger White Paper](linked/SynLedger_White_Paper.pdf): Outlines the technical architecture and financial model for the SynLedger project, including advanced PoSyg consensus details and tokenomics considerations.
- [PoSyg Consensus Mechanism](linked/PoSyg_Consensus_Mechanism.pdf): Explores the multi-dimensional Synergy Score, reward/penalty systems, and proof-of-stake integration used to incentivize honest participation and deter malicious activity.
- [Dynamic Contribution Scoring Paper](linked/ssrn-5045954.pdf): Presents a formal mathematical model for the Dynamic Contribution Score (DCS), covering linearity, penalty dominance, and advanced extensions like time-varying weights and stochastic user behavior.

---
© 2025 PeoChain Demo Project
