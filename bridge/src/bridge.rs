/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST BRIDGE MODULE
 * ----------------------------------------------------------------------------
 * This module defines the basic cross-chain bridge logic to synchronize
 * state and transfer assets between PeoChain and external networks.
 *
 * PRINCIPLES:
 * - SRP: bridge.rs focuses on deposit and withdrawal logic.
 * - OCP: extended bridging functionalities (e.g., multi-chain support) can
 *        be added without altering existing core code.
 * - LSP: any future bridging mechanism can replace or extend the current
 *        logic if it implements BridgeEngine trait.
 * - ISP: only the essential methods (deposit, withdraw, verify) are exposed.
 * - DIP: high-level modules depend on the abstract BridgeEngine trait,
 *        not directly on BridgeService.
 * - DRY: repeated code for deposit/withdraw is centralized in helper methods.
 * - KISS: keep bridging logic straightforward to facilitate auditing.
 */

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use bytes::BytesMut;
use crate::structured_types::{StructuredTransaction, TransactionType};

/// Maximum allowed length for user identifiers to prevent unbounded allocations
const MAX_USER_ID_LENGTH: usize = 42; // Reduced to 42 for address + prefix

/// Maximum size for proof data (64KB)
const MAX_PROOF_SIZE: usize = 65536; // 64KB

/// Maximum size for serialized messages (1KB)
const MAX_SERIALIZED_MSG_SIZE: usize = 1024;

/// Statistics reporting interval in seconds
const STATS_REPORTING_INTERVAL: u64 = 300; // 5 minutes

/// Tracks overall memory usage for HashMap structures
static TOTAL_BRIDGE_MEMORY_USAGE: AtomicUsize = AtomicUsize::new(0);

/// Validates user input to prevent unbounded string allocations
fn validate_user_id(user: &str) -> Result<(), String> {
    if user.is_empty() {
        return Err("User ID cannot be empty".to_string());
    }
    if user.len() > MAX_USER_ID_LENGTH {
        return Err(format!("User ID too long. Maximum {} characters allowed", MAX_USER_ID_LENGTH));
    }
    // Basic validation for allowed characters (alphanumeric + some special chars)
    if !user.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err("User ID contains invalid characters".to_string());
    }
    Ok(())
}

/// Operation type for transactions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OperationType {
    Deposit,
    Withdraw,
}

/// Structured transaction with bounded fields
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    #[serde(with = "crate::bounded_string")]
    pub user: String,
    pub amount: u64,
    pub op_type: OperationType,
}

impl Transaction {
    /// Validates transaction fields
    pub fn validate(&self) -> Result<(), String> {
        validate_user_id(&self.user)?;
        if self.amount == 0 {
            return Err("Transaction amount cannot be zero".to_string());
        }
        Ok(())
    }
    
    /// Serialization with size limits
    pub fn to_bytes(&self) -> Result<BytesMut, String> {
        self.validate()?;
        let json = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        if json.len() > MAX_SERIALIZED_MSG_SIZE {
            return Err(format!("Serialized transaction too large: {} bytes", json.len()));
        }
        Ok(BytesMut::from(&json[..]))
    }
    
    /// Deserialization with size limits
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() > MAX_SERIALIZED_MSG_SIZE {
            return Err("Input buffer too large".to_string());
        }
        let tx: Transaction = serde_json::from_slice(buf).map_err(|e| e.to_string())?;
        tx.validate()?;
        Ok(tx)
    }
}

/// Custom error for proof verification
#[derive(Debug)]
pub enum ProofError {
    EmptyProof,
    OversizedProof,
    InvalidFormat,
}

impl std::fmt::Display for ProofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofError::EmptyProof => write!(f, "Empty proof data provided"),
            ProofError::OversizedProof => write!(f, "Proof data too large. Maximum size is {} bytes", MAX_PROOF_SIZE),
            ProofError::InvalidFormat => write!(f, "Invalid proof format"),
        }
    }
}

impl std::error::Error for ProofError {}

/// Trait that defines the essential operations any bridge engine must provide.
pub trait BridgeEngine {
    fn deposit(&mut self, user: &str, amount: u64) -> Result<(), String>;
    fn withdraw(&mut self, user: &str, amount: u64) -> Result<(), String>;
    fn verify_proof(&self, proof_data: &[u8]) -> Result<bool, ProofError>;
    fn get_balance(&self, user: &str) -> u64;
    fn get_memory_usage(&self) -> usize;
    
    // New methods for structured transactions
    fn process_transaction(&mut self, tx: &Transaction) -> Result<(), String>;
    fn process_transaction_from_bytes(&mut self, data: &[u8]) -> Result<(), String>;
    fn process_structured_transaction(&mut self, tx: &StructuredTransaction) -> Result<(), String>;
    fn process_structured_transaction_from_bytes(&mut self, data: &[u8]) -> Result<(), String>;
}

/// BridgeService is a basic implementation of a cross-chain bridge engine.
/// It simulates user balances and includes a simple proof verification stub.
pub struct BridgeService {
    /// Track user balances (in a real-world scenario, these might
    /// be tracked on-chain and verified via cryptographic proofs).
    balances: HashMap<String, u64>,
    /// Monitor memory usage statistics
    last_stats_report: Instant,
    /// Total number of operations processed
    operations_count: usize,
    /// Configurable max users
    max_users: usize,
}

impl BridgeService {
    /// Creates a new BridgeService instance with empty balances.
    pub fn new() -> Self {
        BridgeService {
            balances: HashMap::new(),
            last_stats_report: Instant::now(),
            operations_count: 0,
            max_users: 100_000, // Default value
        }
    }
    
    /// Estimates memory usage of current HashMap structures
    fn estimate_memory_usage(&self) -> usize {
        // Approximate memory calculation for HashMap entries
        // Each entry has a key (String) and a value (u64)
        let mut total_bytes = 0;
        
        // HashMap overhead (very rough estimate)
        total_bytes += std::mem::size_of::<HashMap<String, u64>>();
        
        // Estimate for each entry
        for (key, _) in &self.balances {
            // String memory: capacity (not just length) + pointer overhead
            let string_capacity = key.capacity();
            total_bytes += std::mem::size_of::<String>() + string_capacity;
            
            // u64 value
            total_bytes += std::mem::size_of::<u64>();
        }
        
        total_bytes
    }
    
    /// Updates memory usage statistics and logs if needed
    fn update_memory_stats(&mut self) {
        self.operations_count += 1;
        
        // Only recalculate periodically to reduce overhead
        if self.last_stats_report.elapsed() > Duration::from_secs(STATS_REPORTING_INTERVAL) {
            let memory_usage = self.estimate_memory_usage();
            
            // Update atomic counter for global monitoring
            TOTAL_BRIDGE_MEMORY_USAGE.store(memory_usage, Ordering::Relaxed);
            
            // Log memory usage statistics
            println!(
                "[MEMORY STATS] Bridge module using ~{} KB, {} users, {} operations since last report",
                memory_usage / 1024,
                self.balances.len(),
                self.operations_count
            );
            
            self.last_stats_report = Instant::now();
            self.operations_count = 0;
        }
    }

    /// Initialize the user's balance if not present, with input validation.
    fn ensure_user(&mut self, user: &str) -> Result<(), String> {
        validate_user_id(user)?;
        if !self.balances.contains_key(user) {
            // Check if maximum capacity is reached
            if self.balances.len() >= self.max_users {
                return Err("Maximum user capacity reached".to_string());
            }
            
            self.balances.insert(user.to_string(), 0);
            
            // Update memory statistics after adding a new user
            self.update_memory_stats();
        }
        Ok(())
    }
}

impl BridgeEngine for BridgeService {
    /// Simulates a deposit from an external chain into PeoChain.
    fn deposit(&mut self, user: &str, amount: u64) -> Result<(), String> {
        // In a real environment, this would require verifying external chain
        // proofs and updating on-chain state. Here we just increment the
        // user's balance.
        self.ensure_user(user)?;
        if let Some(bal) = self.balances.get_mut(user) {
            let old_balance = *bal;
            match bal.checked_add(amount) {
                Some(new_balance) => {
                    *bal = new_balance;
                    
                    // Periodically update memory statistics
                    self.update_memory_stats();
                    
                    println!("Deposit successful: user={}, amount={}", user, amount);
                    Ok(())
                },
                None => {
                    Err(format!("Deposit overflow. user={}, old_balance={}, amount={}", user, old_balance, amount))
                }
            }
        } else {
            Err("User not found after ensure_user".to_string())
        }
    }

    /// Simulates a withdrawal from PeoChain to an external chain.
    fn withdraw(&mut self, user: &str, amount: u64) -> Result<(), String> {
        self.ensure_user(user)?;
        let balance = self.balances.get_mut(user).unwrap();
        if *balance < amount {
            return Err("Insufficient balance for withdrawal".to_string());
        }
        *balance -= amount;
        
        // Periodically update memory statistics
        self.update_memory_stats();
        
        println!("Withdrawal successful: user={}, amount={}", user, amount);
        Ok(())
    }

    /// Minimal proof verification. In reality, this would involve
    /// cryptographic checks of Merkle proofs or signature-based validation.
    fn verify_proof(&self, proof_data: &[u8]) -> Result<bool, ProofError> {
        if proof_data.is_empty() {
            return Err(ProofError::EmptyProof);
        }
        if proof_data.len() > MAX_PROOF_SIZE {
            return Err(ProofError::OversizedProof);
        }
        if proof_data[0] == 0 {
            return Err(ProofError::InvalidFormat);
        }
        Ok(true)
    }

    /// Returns the current balance of the specified user.
    fn get_balance(&self, user: &str) -> u64 {
        // Validate user ID before lookup to prevent potential issues
        if validate_user_id(user).is_err() {
            return 0;
        }
        *self.balances.get(user).unwrap_or(&0)
    }
    
    /// Returns the estimated memory usage of this bridge service
    fn get_memory_usage(&self) -> usize {
        self.estimate_memory_usage()
    }
    
    /// Process transaction through structured type
    fn process_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        match tx.op_type {
            OperationType::Deposit => self.deposit(&tx.user, tx.amount),
            OperationType::Withdraw => self.withdraw(&tx.user, tx.amount),
        }
    }
    
    /// Process transaction from serialized bytes
    fn process_transaction_from_bytes(&mut self, data: &[u8]) -> Result<(), String> {
        let tx = Transaction::from_bytes(data)?;
        self.process_transaction(&tx)
    }
    
    /// Process structured transaction (more efficient than string-based)
    fn process_structured_transaction(&mut self, tx: &StructuredTransaction) -> Result<(), String> {
        match tx.tx_type {
            TransactionType::Transfer => {
                // For bridge operations, we treat transfers as deposits or withdrawals
                // based on the to/from addresses
                if tx.to == "bridge_deposit" {
                    self.deposit(&tx.from, tx.amount)
                } else if tx.from == "bridge_withdraw" {
                    self.withdraw(&tx.to, tx.amount)
                } else {
                    Err("Invalid bridge transaction type".to_string())
                }
            },
            TransactionType::Mint => self.deposit(&tx.to, tx.amount),
            TransactionType::Burn => self.withdraw(&tx.from, tx.amount),
            TransactionType::SmartContract => {
                // For demo purposes, reject smart contract transactions
                Err("Smart contract transactions not supported in bridge".to_string())
            }
        }
    }
    
    /// Process structured transaction from bytes
    fn process_structured_transaction_from_bytes(&mut self, data: &[u8]) -> Result<(), String> {
        let tx = StructuredTransaction::from_bytes(data)?;
        self.process_structured_transaction(&tx)
    }
}
