/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: CONSENSUS MESSAGE TYPES
 * ----------------------------------------------------------------------------
 * Structured message types for consensus operations with bounds checking
 * and memory safety guarantees.
 */

use serde::{Serialize, Deserialize, Deserializer};
use bytes::BytesMut;
use std::fmt;
use bincode;

/// Maximum size for serialized consensus messages
const MAX_CONSENSUS_MSG_SIZE: usize = 2 * 1024 * 1024; // 2MB

/// Maximum length for validator identifiers
const MAX_VALIDATOR_ID_LENGTH: usize = 128;

/// Maximum length for individual transaction data
const MAX_TRANSACTION_DATA_LENGTH: usize = 32 * 1024; // 32KB

/// Maximum number of transactions per block
const MAX_TRANSACTIONS_PER_BLOCK: usize = 10000;

/// Custom deserializer for bounded strings
pub fn bounded_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.len() > MAX_VALIDATOR_ID_LENGTH {
        return Err(serde::de::Error::custom(format!(
            "String too long: {} > {}",
            s.len(),
            MAX_VALIDATOR_ID_LENGTH
        )));
    }
    Ok(s)
}

/// Custom deserializer for bounded transaction data
fn bounded_transaction_data<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.len() > MAX_TRANSACTION_DATA_LENGTH {
        return Err(serde::de::Error::custom(format!(
            "Transaction data too long: {} > {}",
            s.len(),
            MAX_TRANSACTION_DATA_LENGTH
        )));
    }
    Ok(s)
}

/// Custom deserializer for bounded transaction vectors
fn bounded_transactions<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let transactions: Vec<String> = Vec::deserialize(deserializer)?;
    
    if transactions.len() > MAX_TRANSACTIONS_PER_BLOCK {
        return Err(serde::de::Error::custom(format!(
            "Too many transactions: {} > {}",
            transactions.len(),
            MAX_TRANSACTIONS_PER_BLOCK
        )));
    }
    
    // Validate each transaction
    for tx in &transactions {
        if tx.len() > MAX_TRANSACTION_DATA_LENGTH {
            return Err(serde::de::Error::custom(format!(
                "Transaction too long: {} > {}",
                tx.len(),
                MAX_TRANSACTION_DATA_LENGTH
            )));
        }
    }
    
    Ok(transactions)
}

/// Enhanced transaction type with validation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConsensusTransaction {
    #[serde(deserialize_with = "bounded_string")]
    pub from: String,
    
    #[serde(deserialize_with = "bounded_string")]
    pub to: String,
    
    pub amount: u64,
    
    #[serde(deserialize_with = "bounded_transaction_data")]
    pub data: String,
    
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
}

impl ConsensusTransaction {
    /// Creates a new transaction with validation
    pub fn new(
        from: String,
        to: String,
        amount: u64,
        data: String,
        nonce: u64,
        gas_limit: u64,
        gas_price: u64,
    ) -> Result<Self, String> {
        if from.len() > MAX_VALIDATOR_ID_LENGTH {
            return Err(format!("From address too long: {}", from.len()));
        }
        
        if to.len() > MAX_VALIDATOR_ID_LENGTH {
            return Err(format!("To address too long: {}", to.len()));
        }
        
        if data.len() > MAX_TRANSACTION_DATA_LENGTH {
            return Err(format!("Transaction data too long: {}", data.len()));
        }
        
        Ok(ConsensusTransaction {
            from,
            to,
            amount,
            data,
            nonce,
            gas_limit,
            gas_price,
        })
    }
    
    /// Validates transaction parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.from.is_empty() {
            return Err("From address cannot be empty".to_string());
        }
        
        if self.to.is_empty() {
            return Err("To address cannot be empty".to_string());
        }
        
        if self.gas_limit == 0 {
            return Err("Gas limit cannot be zero".to_string());
        }
        
        if self.gas_price == 0 {
            return Err("Gas price cannot be zero".to_string());
        }
        
        Ok(())
    }
    
    /// Validates if the transaction is well-formed and valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
    
    /// Calculates a simple hash of the transaction for merkle tree computation
    pub fn hash(&self) -> Result<[u8; 32], String> {
        let serialized = self.to_bytes()?;
        
        // Simple hash calculation (in production, use SHA256 or similar)
        let mut hash = [0u8; 32];
        let mut hasher = 0u64;
        
        for (i, byte) in serialized.iter().enumerate() {
            hasher = hasher.wrapping_add(*byte as u64);
            hasher = hasher.wrapping_mul(31);
            hash[i % 32] ^= (hasher % 256) as u8;
        }
        
        Ok(hash)
    }
    
    /// Serialization with size limits
    pub fn to_bytes(&self) -> Result<BytesMut, String> {
        self.validate()?;
        let json = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        if json.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err(format!("Serialized transaction too large: {} bytes", json.len()));
        }
        Ok(BytesMut::from(&json[..]))
    }
    
    /// Deserialization with size limits
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err("Input buffer too large".to_string());
        }
        let tx: ConsensusTransaction = serde_json::from_slice(buf).map_err(|e| e.to_string())?;
        tx.validate()?;
        Ok(tx)
    }
    
    /// More efficient binary serialization using bincode
    pub fn to_bytes_binary(&self) -> Result<BytesMut, String> {
        self.validate()?;
        let serialized = bincode::serialize(self)
            .map_err(|e| format!("Binary serialization failed: {}", e))?;
        
        if serialized.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err(format!(
                "Serialized transaction too large: {} bytes (max: {})",
                serialized.len(),
                MAX_CONSENSUS_MSG_SIZE
            ));
        }
        
        Ok(BytesMut::from(&serialized[..]))
    }
    
    /// Binary deserialization using bincode
    pub fn from_bytes_binary(data: &[u8]) -> Result<Self, String> {
        if data.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err("Input data too large".to_string());
        }
        
        let tx: Self = bincode::deserialize(data)
            .map_err(|e| format!("Binary deserialization failed: {}", e))?;
        
        tx.validate()?;
        Ok(tx)
    }
    
    /// Estimates the size of this transaction in bytes
    pub fn estimate_size(&self) -> usize {
        self.from.len() + 
        self.to.len() + 
        self.data.len() + 
        std::mem::size_of::<u64>() * 4 // amount, nonce, gas_limit, gas_price
    }
}

/// Enhanced block type with structured transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ConsensusBlock {
    pub id: u64,
    
    #[serde(deserialize_with = "bounded_string")]
    pub proposer: String,
    
    #[serde(deserialize_with = "bounded_transactions")]
    pub transactions: Vec<String>, // Keep as strings for backward compatibility
    
    pub timestamp: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub nonce: u64,
}

impl ConsensusBlock {
    /// Creates a new block with validation
    pub fn new(
        id: u64,
        proposer: String,
        transactions: Vec<String>,
        timestamp: u64,
        previous_hash: String,
        merkle_root: String,
        nonce: u64,
    ) -> Result<Self, String> {
        if proposer.len() > MAX_VALIDATOR_ID_LENGTH {
            return Err(format!("Proposer ID too long: {}", proposer.len()));
        }
        
        if transactions.len() > MAX_TRANSACTIONS_PER_BLOCK {
            return Err(format!("Too many transactions: {}", transactions.len()));
        }
        
        // Calculate total block size
        let mut total_size = proposer.len() + previous_hash.len() + merkle_root.len();
        total_size += std::mem::size_of::<u64>() * 4; // id, timestamp, nonce
        
        for tx in &transactions {
            if tx.len() > MAX_TRANSACTION_DATA_LENGTH {
                return Err(format!("Transaction too large: {}", tx.len()));
            }
            total_size += tx.len();
        }
        
        if total_size > MAX_CONSENSUS_MSG_SIZE {
            return Err(format!("Block too large: {} bytes", total_size));
        }
        
        Ok(ConsensusBlock {
            id,
            proposer,
            transactions,
            timestamp,
            previous_hash,
            merkle_root,
            nonce,
        })
    }
    
    /// Validates block parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.proposer.is_empty() {
            return Err("Proposer cannot be empty".to_string());
        }
        
        if self.timestamp == 0 {
            return Err("Timestamp cannot be zero".to_string());
        }
        
        Ok(())
    }
    
    /// Serialization with size limits
    pub fn to_bytes(&self) -> Result<BytesMut, String> {
        self.validate()?;
        let json = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        if json.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err(format!("Serialized block too large: {} bytes", json.len()));
        }
        Ok(BytesMut::from(&json[..]))
    }
    
    /// Deserialization with size limits
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err("Input buffer too large".to_string());
        }
        let block: ConsensusBlock = serde_json::from_slice(buf).map_err(|e| e.to_string())?;
        block.validate()?;
        Ok(block)
    }
    
    /// Estimates the size of this block in bytes
    pub fn estimate_size(&self) -> usize {
        let mut total_size = self.proposer.len() + self.previous_hash.len() + self.merkle_root.len();
        total_size += std::mem::size_of::<u64>() * 4; // id, timestamp, nonce
        
        for tx in &self.transactions {
            total_size += tx.len();
        }
        
        total_size
    }
}

/// Consensus message envelope for network communication
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub enum ConsensusMessage {
    BlockProposal {
        block: ConsensusBlock,
        proposer_signature: String,
    },
    BlockVote {
        block_id: u64,
        #[serde(deserialize_with = "bounded_string")]
        validator_id: String,
        vote: bool, // true = accept, false = reject
        signature: String,
    },
    ValidatorRegistration {
        #[serde(deserialize_with = "bounded_string")]
        validator_id: String,
        stake: u64,
        public_key: String,
        signature: String,
    },
}

impl ConsensusMessage {
    /// Serialization with size limits
    pub fn to_bytes(&self) -> Result<BytesMut, String> {
        let json = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        if json.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err(format!("Serialized message too large: {} bytes", json.len()));
        }
        Ok(BytesMut::from(&json[..]))
    }
    
    /// Deserialization with size limits
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() > MAX_CONSENSUS_MSG_SIZE {
            return Err("Input buffer too large".to_string());
        }
        let msg: ConsensusMessage = serde_json::from_slice(buf).map_err(|e| e.to_string())?;
        Ok(msg)
    }
}

impl fmt::Display for ConsensusMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsensusMessage::BlockProposal { block, .. } => {
                write!(f, "BlockProposal(id: {}, proposer: {})", block.id, block.proposer)
            }
            ConsensusMessage::BlockVote { block_id, validator_id, vote, .. } => {
                write!(f, "BlockVote(block: {}, validator: {}, vote: {})", block_id, validator_id, vote)
            }
            ConsensusMessage::ValidatorRegistration { validator_id, stake, .. } => {
                write!(f, "ValidatorRegistration(id: {}, stake: {})", validator_id, stake)
            }
        }
    }
}
