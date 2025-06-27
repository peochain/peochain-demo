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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Trait that abstracts basic EVM operations.
pub trait EvmExecutor {
    fn execute_transaction(&mut self, from: &str, to: &str, data: &[u8]) -> Result<(), String>;
    fn get_balance(&self, address: &str) -> u64;
    fn set_balance(&mut self, address: &str, amount: u64) -> Result<(), String>;
    fn get_memory_usage(&self) -> usize;
    fn get_account_count(&self) -> usize;
}

/// Maximum allowed length for Ethereum addresses to prevent unbounded allocations
const MAX_ADDRESS_LENGTH: usize = 42; // Standard Ethereum address length (0x + 40 hex chars)
/// Maximum number of accounts to prevent memory exhaustion
const MAX_ACCOUNTS: usize = 1_000_000;
/// Maximum transaction data size (in bytes)
const MAX_TRANSACTION_DATA_SIZE: usize = 32768; // 32KB
/// Statistics reporting interval in seconds
const STATS_REPORTING_INTERVAL: u64 = 300; // 5 minutes

/// Tracks overall memory usage for EVM module
static TOTAL_EVM_MEMORY_USAGE: AtomicUsize = AtomicUsize::new(0);

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
    /// Tracks when the last memory report was generated
    last_stats_report: Instant,
    /// Tracks operations count since last report
    operations_count: usize,
    /// Tracks current block number
    block_number: u64,
    /// Tracks blockchain memory usage over time (bytes)
    blockchain_size: usize,
}

impl BasicEvmExecutor {
    /// Constructs a new `BasicEvmExecutor`.
    pub fn new() -> Self {
        Self {
            balances: std::collections::HashMap::new(),
            last_stats_report: Instant::now(),
            operations_count: 0,
            block_number: 0,
            blockchain_size: 0,
        }
    }
    
    /// Estimates the current memory usage of the EVM
    fn estimate_memory_usage(&self) -> usize {
        let mut total_bytes = 0;
        
        // Base structure size
        total_bytes += std::mem::size_of::<BasicEvmExecutor>();
        
        // HashMap overhead estimate
        total_bytes += std::mem::size_of::<std::collections::HashMap<String, u64>>();
        
        // Calculate size of all accounts
        for (address, _) in &self.balances {
            // String memory: capacity (not just length) + pointer overhead
            let string_capacity = address.capacity();
            total_bytes += std::mem::size_of::<String>() + string_capacity;
            
            // u64 value
            total_bytes += std::mem::size_of::<u64>();
        }
        
        // Add blockchain storage size
        total_bytes += self.blockchain_size;
        
        total_bytes
    }
    
    /// Updates memory usage statistics and logs if needed
    fn update_memory_stats(&mut self) {
        self.operations_count += 1;
        
        // Only recalculate periodically to reduce overhead
        if self.last_stats_report.elapsed() > Duration::from_secs(STATS_REPORTING_INTERVAL) {
            let memory_usage = self.estimate_memory_usage();
            
            // Update atomic counter for global monitoring
            TOTAL_EVM_MEMORY_USAGE.store(memory_usage, Ordering::Relaxed);
            
            // Log memory usage statistics
            println!(
                "[MEMORY STATS] EVM module using ~{} KB, {} accounts, {} operations since last report",
                memory_usage / 1024,
                self.balances.len(),
                self.operations_count
            );
            
            self.last_stats_report = Instant::now();
            self.operations_count = 0;
        }
    }
    
    /// Increments the block number and updates blockchain size
    pub fn increment_block_number(&mut self, block_size: usize) {
        self.block_number += 1;
        self.blockchain_size += block_size;
        
        // Log when blockchain size grows significantly
        if self.block_number % 100 == 0 {
            println!(
                "[BLOCKCHAIN] Size: {} MB, Blocks: {}",
                self.blockchain_size / (1024 * 1024),
                self.block_number
            );
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
            
            // Update memory stats when adding new account
            self.update_memory_stats();
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

        // Update blockchain size for memory tracking
        self.blockchain_size += data.len() + from.len() + to.len() + 16; // 16 bytes for tx metadata
        
        // Update memory usage statistics
        self.update_memory_stats();

        // Debug log to simulate contract execution
        println!(
            "Executing transaction from: {} to: {}, data size: {} bytes",
            from, to, data.len()
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
        
        // Update memory stats
        self.update_memory_stats();
        
        Ok(())
    }
    
    /// Returns the estimated memory usage of the EVM
    fn get_memory_usage(&self) -> usize {
        self.estimate_memory_usage()
    }
    
    /// Returns the current number of accounts in the EVM
    fn get_account_count(&self) -> usize {
        self.balances.len()
    }
}
