# Consensus Module - PeoChain Demo

The `consensus` module implements the Proof of Synergy (PoSyg) and Dynamic Contribution Scoring (DCS) consensus mechanism for the PeoChain blockchain. It is designed to manage a network of validators, enabling them to propose and validate blocks while dynamically adjusting their synergy scores based on honest participation or malicious behavior. This module is a core component of the PeoChain Demo project, showcasing a scalable and secure consensus protocol.

## Features

- **Proof of Synergy (PoSyg)**: Combines stake-based weighting with a synergy score system to select block proposers.
- **Dynamic Contribution Scoring (DCS)**: Rewards honest validators and penalizes malicious ones through a configurable scoring formula.
- **Validator Network**: Simulates a decentralized network with weighted random proposer selection.
- **Rust Implementation**: Built with Rust for performance, safety, and reliability.

## Prerequisites

- **Rust**: Version 1.68 or higher (recommended).
- **Cargo**: Rust’s package manager and build tool.
- **Operating System**: Tested on Fedora 41, but compatible with other platforms supporting Rust.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/peopay/peochain-demo.git
   cd peochain-demo/consensus
   ```

2. **Install Dependencies**:
   Ensure you have Rust installed, then let Cargo handle the rest:
   ```bash
   cargo build
   ```
   This will download and compile the required dependencies, including `rand` version 0.8.5.

## Usage

### Running the Simulation
The `consensus_node` binary simulates the consensus process over 5 rounds with three validators: two honest and one malicious.

```bash
cargo run
```

**Example Output**:
```
Starting consensus round 1
Validator 1 (validator1): Synergy Score = 3.40, Violations = 0, Proposed = 1, Accepted = 1
Validator 2 (validator2): Synergy Score = 0.00, Violations = 0, Proposed = 0, Accepted = 0
Validator 3 (validator3): Synergy Score = 0.00, Violations = 0, Proposed = 0, Accepted = 0

Starting consensus round 2
Validator 1 (validator1): Synergy Score = 6.80, Violations = 0, Proposed = 2, Accepted = 2
Validator 2 (validator2): Synergy Score = 0.00, Violations = 0, Proposed = 0, Accepted = 0
Validator 3 (validator3): Synergy Score = 0.00, Violations = 0, Proposed = 0, Accepted = 0
...
```

### Testing
Run unit tests and integration tests to verify the module’s functionality:

```bash
cargo test
```

To run only doctests (examples in the documentation):
```bash
cargo test --doc
```

## Structure

- **`src/lib.rs`**: Core library code defining the consensus mechanism, including `ConsensusEngine`, `PosygDcsEngine`, and `Network`.
- **`src/main.rs`**: Binary entry point for running a simulation of the consensus process.
- **`tests/integration_test.rs`**: Integration tests ensuring the consensus logic works as expected over multiple rounds.
- **`Cargo.toml`**: Project configuration and dependencies.

## Code Example

Here’s how to create and run a simple network programmatically:

```rust
use peo_consensus::{ConsensusEngine, Network, PosygDcsEngine};

fn main() {
    let mut network = Network {
        validators: vec![
            PosygDcsEngine::new("v1".to_string(), 1000, false),
            PosygDcsEngine::new("v2".to_string(), 1500, true),
        ],
    };
    network.run_consensus_round();
    println!("Validator 1 Score: {}", network.validators[0].get_synergy_score());
}
```

## Documentation

Detailed documentation is embedded in the source code using Rust’s doc comments. To generate and view it locally:

```bash
cargo doc --open
```

This opens a browser with HTML documentation for the `peo_consensus` library.

## Contributing

We welcome contributions to enhance the consensus module! To contribute:

1. **Fork the Repository**: Create your own fork on GitHub.
2. **Create a Branch**: Use a descriptive name, e.g., `feature/add-validator-sync`.
   ```bash
   git checkout -b feature/add-validator-sync
   ```
3. **Make Changes**: Implement your feature or fix, ensuring tests pass.
4. **Run Tests**: Verify with `cargo test`.
5. **Submit a Pull Request**: Open a PR on GitHub with a clear description of your changes.

Please follow Rust coding conventions and include tests for new functionality.

## License

This module is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For questions or support, contact Daniil Krizhanovskyi at [d.krizhanovskyi@peochain.xyz](mailto:d.krizhanovskyi@peochain.xyz) or open an issue on the GitHub repository.

---

© 2025 PeoChain Demo Project

