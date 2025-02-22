/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST EVM CORE
 * ----------------------------------------------------------------------------
 * This file provides the core EVM execution logic or an integration point
 * with an existing Rust EVM library.
 *
 * PRINCIPLES:
 * - SRP: evm_core.rs focuses on EVM execution and contract management.
 * - OCP: new features (e.g., forks, precompiles) can be added without modifying
 *        existing code.
 * - LSP: alternative EVM implementations can replace or extend this module
 *        if they conform to the EvmExecutor trait.
 * - ISP: only the relevant methods for contract execution and state transitions
 *        are exposed.
 * - DIP: high-level modules depend on the abstract EvmExecutor trait, not
 *        direct struct implementations.
 * - DRY & KISS: repeated logic is confined to helper methods; code remains readable.
 */

/// Trait that abstracts basic EVM operations.
pub trait EvmExecutor {
    fn execute_transaction(&mut self, from: &str, to: &str, data: &[u8]) -> Result<(), String>;
    fn get_balance(&self, address: &str) -> u64;
    fn set_balance(&mut self, address: &str, amount: u64);
}

/// BasicEvmExecutor is a demonstration EVM engine implementation.
/// In a real system, this might wrap a well-known EVM library (e.g., SputnikVM).
pub struct BasicEvmExecutor {
    /// A simplistic mapping of addresses to balances for demonstration.
    balances: std::collections::HashMap<String, u64>,
}

impl BasicEvmExecutor {
    /// Constructs a new `BasicEvmExecutor`.
    pub fn new() -> Self {
        Self {
            balances: std::collections::HashMap::new(),
        }
    }

    /// Internal helper to initialize a balance if the address does not exist.
    fn ensure_address(&mut self, addr: &str) {
        if !self.balances.contains_key(addr) {
            self.balances.insert(addr.to_string(), 0);
        }
    }
}

impl EvmExecutor for BasicEvmExecutor {
    /// Simulates a transaction execution (e.g., contract call, transfer).
    fn execute_transaction(&mut self, from: &str, to: &str, data: &[u8]) -> Result<(), String> {
        // In a real system, data would be compiled contract bytecode or call data.
        // Here, we simply demonstrate address checks and debug outputs.
        self.ensure_address(from);
        self.ensure_address(to);

        // Debug log to simulate contract execution
        println!(
            "Executing transaction from: {} to: {}, data: {:?}",
            from, to, data
        );
        Ok(())
    }

    /// Returns the balance of the given address.
    fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    /// Sets the balance for an address.
    fn set_balance(&mut self, address: &str, amount: u64) {
        self.ensure_address(address);
        if let Some(balance) = self.balances.get_mut(address) {
            *balance = amount;
        }
    }
}
