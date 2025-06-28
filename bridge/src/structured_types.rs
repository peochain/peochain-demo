/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: STRUCTURED TRANSACTION TYPES
 * ----------------------------------------------------------------------------
 * Defines structured transaction types with proper validation and serialization
 */

use serde::{Serialize, Deserialize};
use bytes::BytesMut;
use crate::bounded_string;

/// Maximum size for serialized transaction data
const MAX_SERIALIZED_TX_SIZE: usize = 8 * 1024; // 8KB

/// Transaction type enumeration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TransactionType {
    Transfer,
    SmartContract,
    Mint,
    Burn,
}

/// Structured transaction data with bounds validation
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StructuredTransaction {
    #[serde(with = "bounded_string")]
    pub from: String,
    
    #[serde(with = "bounded_string")]
    pub to: String,
    
    pub amount: u64,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_type: TransactionType,
    
    #[serde(default)]
    pub data: Vec<u8>, // Contract data or metadata
}

impl StructuredTransaction {
    /// Creates a new structured transaction with validation
    pub fn new(
        from: String,
        to: String,
        amount: u64,
        nonce: u64,
        gas_limit: u64,
        gas_price: u64,
        tx_type: TransactionType,
        data: Vec<u8>,
    ) -> Result<Self, String> {
        let tx = Self {
            from,
            to,
            amount,
            nonce,
            gas_limit,
            gas_price,
            tx_type,
            data,
        };
        
        tx.validate()?;
        Ok(tx)
    }
    
    /// Validates transaction fields
    pub fn validate(&self) -> Result<(), String> {
        if self.from.len() > 42 {
            return Err("From address too long".to_string());
        }
        
        if self.to.len() > 42 {
            return Err("To address too long".to_string());
        }
        
        if self.amount == 0 && self.tx_type == TransactionType::Transfer {
            return Err("Transfer amount cannot be zero".to_string());
        }
        
        if self.data.len() > 64 * 1024 { // 64KB limit for data
            return Err("Transaction data too large".to_string());
        }
        
        if self.gas_limit == 0 {
            return Err("Gas limit cannot be zero".to_string());
        }
        
        Ok(())
    }
    
    /// Serializes transaction to bytes with size validation
    pub fn to_bytes(&self) -> Result<BytesMut, String> {
        self.validate()?;
        
        let serialized = bincode::serialize(self)
            .map_err(|e| format!("Serialization failed: {}", e))?;
        
        if serialized.len() > MAX_SERIALIZED_TX_SIZE {
            return Err(format!(
                "Serialized transaction too large: {} bytes (max: {})",
                serialized.len(),
                MAX_SERIALIZED_TX_SIZE
            ));
        }
        
        Ok(BytesMut::from(&serialized[..]))
    }
    
    /// Deserializes transaction from bytes with validation
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() > MAX_SERIALIZED_TX_SIZE {
            return Err("Input data too large".to_string());
        }
        
        let tx: Self = bincode::deserialize(data)
            .map_err(|e| format!("Deserialization failed: {}", e))?;
        
        tx.validate()?;
        Ok(tx)
    }
    
    /// Calculates transaction hash (simplified for demo)
    pub fn hash(&self) -> Result<[u8; 32], String> {
        let bytes = self.to_bytes()?;
        
        // Simple hash calculation (in production, use a proper cryptographic hash)
        let mut hash = [0u8; 32];
        let len = std::cmp::min(bytes.len(), 32);
        hash[..len].copy_from_slice(&bytes[..len]);
        
        Ok(hash)
    }
}

/// Block structure with structured transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StructuredBlock {
    pub block_number: u64,
    pub timestamp: u64,
    pub parent_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    
    #[serde(with = "bounded_string")]
    pub proposer: String,
    
    pub transactions: Vec<StructuredTransaction>,
}

impl StructuredBlock {
    /// Maximum number of transactions per block
    const MAX_TRANSACTIONS: usize = 1000;
    
    /// Maximum block size in bytes
    const MAX_BLOCK_SIZE: usize = 1024 * 1024; // 1MB
    
    /// Creates a new block with validation
    pub fn new(
        block_number: u64,
        timestamp: u64,
        parent_hash: [u8; 32],
        proposer: String,
        transactions: Vec<StructuredTransaction>,
    ) -> Result<Self, String> {
        if transactions.len() > Self::MAX_TRANSACTIONS {
            return Err(format!(
                "Too many transactions: {} (max: {})",
                transactions.len(),
                Self::MAX_TRANSACTIONS
            ));
        }
        
        // Calculate merkle root (simplified)
        let merkle_root = Self::calculate_merkle_root(&transactions)?;
        
        let block = Self {
            block_number,
            timestamp,
            parent_hash,
            merkle_root,
            proposer,
            transactions,
        };
        
        block.validate()?;
        Ok(block)
    }
    
    /// Validates block structure and contents
    pub fn validate(&self) -> Result<(), String> {
        if self.proposer.len() > 42 {
            return Err("Proposer address too long".to_string());
        }
        
        // Validate all transactions
        for tx in &self.transactions {
            tx.validate()?;
        }
        
        // Check total block size
        let serialized_size = bincode::serialize(self)
            .map_err(|e| format!("Block serialization failed: {}", e))?
            .len();
        
        if serialized_size > Self::MAX_BLOCK_SIZE {
            return Err(format!(
                "Block too large: {} bytes (max: {})",
                serialized_size,
                Self::MAX_BLOCK_SIZE
            ));
        }
        
        Ok(())
    }
    
    /// Serializes block to bytes
    pub fn to_bytes(&self) -> Result<BytesMut, String> {
        self.validate()?;
        
        let serialized = bincode::serialize(self)
            .map_err(|e| format!("Block serialization failed: {}", e))?;
        
        Ok(BytesMut::from(&serialized[..]))
    }
    
    /// Deserializes block from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() > Self::MAX_BLOCK_SIZE {
            return Err("Block data too large".to_string());
        }
        
        let block: Self = bincode::deserialize(data)
            .map_err(|e| format!("Block deserialization failed: {}", e))?;
        
        block.validate()?;
        Ok(block)
    }
    
    /// Calculates merkle root from transactions (simplified)
    fn calculate_merkle_root(transactions: &[StructuredTransaction]) -> Result<[u8; 32], String> {
        if transactions.is_empty() {
            return Ok([0u8; 32]);
        }
        
        // Simplified merkle root calculation
        let mut root = [0u8; 32];
        for (i, tx) in transactions.iter().enumerate() {
            let tx_hash = tx.hash()?;
            for j in 0..32 {
                root[j] ^= tx_hash[j].wrapping_add(i as u8);
            }
        }
        
        Ok(root)
    }
}
