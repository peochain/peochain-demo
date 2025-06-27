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
    fn set_balance(&mut self, address: &str, amount: u64) -> Result<(), String>;
}

/// Maximum allowed length for Ethereum addresses to prevent unbounded allocations
const MAX_ADDRESS_LENGTH: usize = 42; // Standard Ethereum address length (0x + 40 hex chars)
/// Maximum number of accounts to prevent memory exhaustion
const MAX_ACCOUNTS: usize = 1_000_000;
/// Maximum transaction data size (in bytes)
const MAX_TRANSACTION_DATA_SIZE: usize = 32768; // 32KB

/// Validates Ethereum address format and length
fn validate_address(address: &str) -> Result<(), String> {
    if address.is_empty() {
        return Err("Address cannot be empty".to_string());
    }
    if address.len() > MAX_ADDRESS_LENGTH {
        return Err(format!("Address too long. Maximum {} characters allowed", MAX_ADDRESS_LENGTH));
    }
    // Basic validation for hex format (should start with 0x for Ethereum addresses)
    if !address.starts_with("0x") && !address.chars().all(|c| c.is_alphanumeric()) {
        return Err("Invalid address format".to_string());
    }
    Ok(())
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
    fn ensure_address(&mut self, addr: &str) -> Result<(), String> {
        validate_address(addr)?;
        
        if !self.balances.contains_key(addr) {
            // Check if we're approaching the maximum number of accounts
            if self.balances.len() >= MAX_ACCOUNTS {
                return Err("Maximum number of accounts reached".to_string());
            }
            self.balances.insert(addr.to_string(), 0);
        }
        Ok(())
    }
}

impl EvmExecutor for BasicEvmExecutor {
    /// Simulates a transaction execution (e.g., contract call, transfer).
    fn execute_transaction(&mut self, from: &str, to: &str, data: &[u8]) -> Result<(), String> {
        // Validate transaction data size to prevent DoS attacks
        if data.len() > MAX_TRANSACTION_DATA_SIZE {
            return Err(format!("Transaction data too large. Maximum {} bytes allowed", MAX_TRANSACTION_DATA_SIZE));
        }
        
        // In a real system, data would be compiled contract bytecode or call data.
        // Here, we simply demonstrate address checks and debug outputs.
        self.ensure_address(from)?;
        self.ensure_address(to)?;

        // Debug log to simulate contract execution
        println!(
            "Executing transaction from: {} to: {}, data: {:?}",
            from, to, data
        );
        Ok(())
    }

    /// Returns the balance of the given address.
    fn get_balance(&self, address: &str) -> u64 {
        // Validate address before lookup
        if validate_address(address).is_err() {
            return 0;
        }
        *self.balances.get(address).unwrap_or(&0)
    }

    /// Sets the balance for an address.
    fn set_balance(&mut self, address: &str, amount: u64) -> Result<(), String> {
        self.ensure_address(address)?;
        if let Some(balance) = self.balances.get_mut(address) {
            *balance = amount;
        }
        Ok(())
    }
}
