// src/lib.rs

use rand::Rng;
use rand::thread_rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Represents errors that can occur during consensus operations.
///
/// This enum defines possible failure modes in the consensus process, such as invalid block proposals
/// or network-related issues.
///
/// # Examples
///
/// ```rust
/// use peo_consensus::ConsensusError;
///
/// let error = ConsensusError::InvalidBlock;
/// assert_eq!(format!("{:?}", error), "InvalidBlock");
/// ```
#[derive(Debug)]
pub enum ConsensusError {
    /// Indicates a block proposal was invalid (e.g., malicious or malformed).
    InvalidBlock,
    /// Indicates the block size is too large
    BlockSizeTooLarge,
    /// Indicates too many transactions in a block
    TooManyTransactions,
    /// Indicates a transaction is invalid
    InvalidTransaction,
    /// Represents a network-related failure, with a descriptive message.
    NetworkError(String),
}

/// Global memory usage tracking for consensus module
static TOTAL_CONSENSUS_MEMORY_USAGE: AtomicUsize = AtomicUsize::new(0);

/// Statistics reporting interval in seconds
const STATS_REPORTING_INTERVAL: u64 = 300; // 5 minutes

/// Defines the behavior required for a consensus engine in the PeoChain network.
///
/// Implementors of this trait must provide methods to propose blocks, validate them, update scores
/// based on consensus outcomes, and retrieve their synergy score.
///
/// # Examples
///
/// ```rust
/// use peo_consensus::{ConsensusEngine, ConsensusError, Block};
///
/// struct DummyEngine {
///     score: f64,
/// }
///
/// impl ConsensusEngine for DummyEngine {
///     fn propose_block(&self) -> Result<Block, ConsensusError> {
///         Ok(Block {
///             id: 1,
///             proposer: "dummy".to_string(),
///             transactions: vec![],
///         })
///     }
///     fn validate_block(&self, _block: &Block) -> Result<(), ConsensusError> {
///         Ok(())
///     }
///     fn update_scores(&mut self, block_accepted: bool, _violation: bool) {
///         if block_accepted {
///             self.score += 1.0;
///         }
///     }
///     fn get_synergy_score(&self) -> f64 {
///         self.score
///     }
/// }
///
/// let mut engine = DummyEngine { score: 0.0 };
/// let block = engine.propose_block().unwrap();
/// engine.update_scores(true, false);
/// assert_eq!(engine.get_synergy_score(), 1.0);
/// ```
pub trait ConsensusEngine {
    /// Proposes a new block for inclusion in the blockchain.
    fn propose_block(&self) -> Result<Block, ConsensusError>;

    /// Validates a proposed block according to consensus rules.
    fn validate_block(&self, block: &Block) -> Result<(), ConsensusError>;

    /// Updates the validator's synergy score based on the outcome of a block proposal.
    fn update_scores(&mut self, block_accepted: bool, violation_occurred: bool);

    /// Retrieves the current synergy score of the validator.
    fn get_synergy_score(&self) -> f64;
}

/// Represents a block in the PeoChain blockchain.
///
/// A block contains an identifier, the proposer's ID, and a list of transactions.
///
/// # Examples
///
/// ```rust
/// use peo_consensus::Block;
///
/// let block = Block {
///     id: 1,
///     proposer: "validator1".to_string(),
///     transactions: vec!["tx1".to_string()],
/// };
/// assert_eq!(block.id, 1);
/// assert_eq!(block.proposer, "validator1");
/// assert_eq!(block.transactions, vec!["tx1"]);
/// ```
#[derive(Clone)]
pub struct Block {
    /// Unique identifier for the block.
    pub id: u64,
    /// Identifier of the validator that proposed the block.
    pub proposer: String,
    /// List of transactions included in the block (bounded to prevent DoS attacks).
    pub transactions: Vec<String>,
}

/// Maximum number of transactions allowed per block to prevent memory exhaustion
const MAX_TRANSACTIONS_PER_BLOCK: usize = 10000;

/// Maximum size of a block in bytes
const MAX_BLOCK_SIZE: usize = 8 * 1024 * 1024; // 8MB

/// Maximum size of a transaction in bytes
const MAX_TRANSACTION_SIZE: usize = 32 * 1024; // 32KB 

/// Maximum length of proposer ID
const MAX_PROPOSER_ID_LENGTH: usize = 256;

impl Block {
    /// Creates a new block with validation for transaction limits
    pub fn new(id: u64, proposer: String, transactions: Vec<String>) -> Result<Self, String> {
        if transactions.len() > MAX_TRANSACTIONS_PER_BLOCK {
            return Err(format!(
                "Too many transactions in block. Maximum {} allowed, got {}",
                MAX_TRANSACTIONS_PER_BLOCK,
                transactions.len()
            ));
        }
        
        // Validate proposer ID length
        if proposer.len() > MAX_PROPOSER_ID_LENGTH {
            return Err(format!(
                "Proposer ID too long. Maximum {} characters allowed, got {}",
                MAX_PROPOSER_ID_LENGTH,
                proposer.len()
            ));
        }
        
        // Calculate total block size
        let mut total_size = proposer.len();
        for tx in &transactions {
            // Check individual transaction size
            if tx.len() > MAX_TRANSACTION_SIZE {
                return Err(format!(
                    "Transaction too large. Maximum {} bytes allowed, got {}",
                    MAX_TRANSACTION_SIZE,
                    tx.len()
                ));
            }
            total_size += tx.len();
        }
        
        // Check total block size
        if total_size > MAX_BLOCK_SIZE {
            return Err(format!(
                "Block too large. Maximum {} bytes allowed, current size {}",
                MAX_BLOCK_SIZE,
                total_size
            ));
        }
        
        Ok(Block {
            id,
            proposer,
            transactions,
        })
    }
    
    /// Adds a transaction to the block with bounds checking
    pub fn add_transaction(&mut self, transaction: String) -> Result<(), String> {
        // Check transaction count
        if self.transactions.len() >= MAX_TRANSACTIONS_PER_BLOCK {
            return Err("Block transaction limit reached".to_string());
        }
        
        // Validate transaction size to prevent large string allocations
        if transaction.len() > MAX_TRANSACTION_SIZE {
            return Err(format!(
                "Transaction too large. Maximum {} bytes allowed, got {}",
                MAX_TRANSACTION_SIZE,
                transaction.len()
            ));
        }
        
        // Calculate current block size
        let current_size = self.estimate_block_size();
        
        // Check if adding this transaction would exceed block size limit
        if current_size + transaction.len() > MAX_BLOCK_SIZE {
            return Err(format!(
                "Adding transaction would exceed block size limit. Current: {}, Max: {}",
                current_size,
                MAX_BLOCK_SIZE
            ));
        }
        
        self.transactions.push(transaction);
        Ok(())
    }
    
    /// Estimates the total size of the block in bytes
    pub fn estimate_block_size(&self) -> usize {
        let mut total_size = 8; // u64 id
        total_size += self.proposer.len();
        
        // Add transaction sizes
        for tx in &self.transactions {
            total_size += tx.len();
        }
        
        total_size
    }
}

/// Implements the PoSyg + DCS consensus engine for a single validator.
///
/// This struct encapsulates the state and behavior of a validator in the PeoChain network,
/// including its synergy score, stake, and proposal history.
///
/// # Examples
///
/// ```rust
/// use peo_consensus::{PosygDcsEngine, ConsensusEngine, ConsensusError, Block};
///
/// let mut validator = PosygDcsEngine::new("v1".to_string(), 1000, false);
/// let block = validator.propose_block().unwrap();
/// validator.update_scores(true, false);
/// assert_eq!(validator.get_synergy_score(), 3.4);
/// assert_eq!(validator.proposed_blocks(), 0); // Not incremented yet
/// validator.increment_proposed_blocks();
/// assert_eq!(validator.proposed_blocks(), 1);
/// ```
pub struct PosygDcsEngine {
    validator_id: String,
    synergy_score: f64,
    stake: u64,
    proposed_blocks: u64,
    accepted_blocks: u64,
    violations: u64,
    is_malicious: bool,
    last_stats_report: Instant,
    operations_count: usize,
}

impl PosygDcsEngine {
    /// Creates a new validator with the specified ID, stake, and malicious behavior flag.
    ///
    /// # Arguments
    ///
    /// * `validator_id` - A unique identifier for the validator.
    /// * `stake` - The amount of stake the validator has committed.
    /// * `is_malicious` - If true, the validator will propose invalid blocks.
    pub fn new(validator_id: String, stake: u64, is_malicious: bool) -> Self {
        // Validate validator ID length
        let validated_id = if validator_id.len() > MAX_PROPOSER_ID_LENGTH {
            validator_id[0..MAX_PROPOSER_ID_LENGTH].to_string()
        } else {
            validator_id
        };
        
        Self {
            validator_id: validated_id,
            synergy_score: 0.0,
            stake,
            proposed_blocks: 0,
            accepted_blocks: 0,
            violations: 0,
            is_malicious,
            last_stats_report: Instant::now(),
            operations_count: 0,
        }
    }
    
    /// Estimates memory usage of this validator
    fn estimate_memory_usage(&self) -> usize {
        let mut total_bytes = 0;
        
        // Base struct size
        total_bytes += std::mem::size_of::<PosygDcsEngine>();
        
        // Dynamic String memory for validator_id
        total_bytes += self.validator_id.capacity();
        
        total_bytes
    }
    
    /// Updates memory usage statistics and logs if needed
    fn update_memory_stats(&mut self) {
        self.operations_count += 1;
        
        // Only recalculate periodically to reduce overhead
        if self.last_stats_report.elapsed() > Duration::from_secs(STATS_REPORTING_INTERVAL) {
            let memory_usage = self.estimate_memory_usage();
            
            // Update atomic counter for global monitoring
            let current = TOTAL_CONSENSUS_MEMORY_USAGE.load(Ordering::Relaxed);
            TOTAL_CONSENSUS_MEMORY_USAGE.store(current + memory_usage, Ordering::Relaxed);
            
            // Log memory usage statistics
            println!(
                "[MEMORY STATS] Validator {} using ~{} bytes, {} operations since last report",
                self.validator_id,
                memory_usage,
                self.operations_count
            );
            
            self.last_stats_report = Instant::now();
            self.operations_count = 0;
        }
    }

    /// Returns the validator's unique identifier.
    pub fn validator_id(&self) -> &str {
        &self.validator_id
    }

    /// Returns the number of violations committed by the validator.
    pub fn violations(&self) -> u64 {
        self.violations
    }

    /// Returns the number of blocks proposed by the validator.
    pub fn proposed_blocks(&self) -> u64 {
        self.proposed_blocks
    }

    /// Returns the number of blocks accepted from this validator.
    pub fn accepted_blocks(&self) -> u64 {
        self.accepted_blocks
    }

    /// Returns the validator's stake amount.
    pub fn stake(&self) -> u64 {
        self.stake
    }

    /// Returns whether the validator is configured to behave maliciously.
    pub fn is_malicious(&self) -> bool {
        self.is_malicious
    }

    /// Sets the validator's synergy score to a specific value.
    ///
    /// # Arguments
    ///
    /// * `score` - The new synergy score to set.
    pub fn set_synergy_score(&mut self, score: f64) {
        self.synergy_score = score;
    }

    /// Increments the count of proposed blocks by one with overflow protection.
    pub fn increment_proposed_blocks(&mut self) {
        self.proposed_blocks = self.proposed_blocks.saturating_add(1);
    }

    /// Increments the count of accepted blocks by one with overflow protection.
    pub fn increment_accepted_blocks(&mut self) {
        self.accepted_blocks = self.accepted_blocks.saturating_add(1);
    }
}

impl ConsensusEngine for PosygDcsEngine {
    fn propose_block(&self) -> Result<Block, ConsensusError> {
        // Create transactions for the block
        let transactions = if self.is_malicious {
            // Malicious validator attempts to create oversized transactions or invalid blocks
            match rand::thread_rng().gen_range(0..3) {
                0 => vec!["invalid_tx".to_string()],
                1 => {
                    // Try to create an oversized transaction (will be rejected)
                    vec!["x".repeat(MAX_TRANSACTION_SIZE + 1000)]
                },
                _ => {
                    // Try to create too many transactions (will be rejected)
                    let tx_count = MAX_TRANSACTIONS_PER_BLOCK + 10;
                    (0..tx_count).map(|i| format!("tx{}", i)).collect()
                }
            }
        } else {
            // Honest validator creates valid blocks
            Vec::new() // Start with empty transactions
        };
    
        // Attempt to create a new block
        let block = Block::new(
            self.proposed_blocks + 1,
            self.validator_id.clone(),
            transactions,
        );
        
        // Convert string errors to ConsensusError type
        match block {
            Ok(b) => Ok(b),
            Err(e) => {
                if e.contains("too large") {
                    Err(ConsensusError::BlockSizeTooLarge)
                } else if e.contains("Too many transactions") {
                    Err(ConsensusError::TooManyTransactions)
                } else {
                    Err(ConsensusError::InvalidBlock)
                }
            }
        }
    }

    fn validate_block(&self, block: &Block) -> Result<(), ConsensusError> {
        if block.transactions.contains(&"invalid_tx".to_string()) {
            Err(ConsensusError::InvalidBlock)
        } else {
            Ok(())
        }
    }

    fn update_scores(&mut self, block_accepted: bool, violation_occurred: bool) {
        const ALPHA: f64 = 0.4;
        const BETA: f64 = 0.3;
        const GAMMA: f64 = 0.2;
        const DELTA: f64 = 0.5;

        let h = if block_accepted { 1.0 } else { 0.0 };
        let e = self.stake as f64 * 0.01;
        let v = 0.0;
        let p = if violation_occurred {
            const BASE_PENALTY: f64 = 10.0;
            const MULTIPLIER: f64 = 2.0;
            const MAX_VIOLATION_EXPONENT: i32 = 10; // Limit to prevent overflow
            
            self.violations = self.violations.saturating_add(1);
            
            // Bound the exponent to prevent overflow
            let bounded_exponent = (self.violations as i32 - 1).min(MAX_VIOLATION_EXPONENT);
            
            BASE_PENALTY * MULTIPLIER.powi(bounded_exponent)
        } else {
            0.0
        };

        self.synergy_score += ALPHA * h + BETA * e + GAMMA * v - DELTA * p;
        
        // Update memory statistics
        self.update_memory_stats();
    }

    fn get_synergy_score(&self) -> f64 {
        self.synergy_score
    }
}

/// Manages a network of validators participating in the consensus process.
///
/// This struct simulates a network where validators take turns proposing blocks based on a weighted
/// random selection, reflecting their synergy scores and stakes.
///
/// # Examples
///
/// ```rust
/// use peo_consensus::{Network, PosygDcsEngine, ConsensusEngine};
///
/// let mut network = Network {
///     validators: vec![
///         PosygDcsEngine::new("v1".to_string(), 1000, false),
///         PosygDcsEngine::new("v2".to_string(), 1500, false),
///     ],
///     last_memory_report: std::time::Instant::now(),
///     blocks_processed: 0,
///     max_validators: 1000,
/// };
/// network.run_consensus_round();
/// let v1 = &network.validators[0];
/// assert!(v1.proposed_blocks() <= 1);
/// ```
pub struct Network {
    pub validators: Vec<PosygDcsEngine>,
    last_memory_report: Instant,
    blocks_processed: usize,
    max_validators: usize,
}

impl Network {
    /// Creates a new Network with default settings
    pub fn new() -> Self {
        Network {
            validators: Vec::new(),
            last_memory_report: Instant::now(),
            blocks_processed: 0,
            max_validators: 1000, // Default maximum number of validators
        }
    }

    /// Adds a validator to the network with bounds checking
    pub fn add_validator(&mut self, validator: PosygDcsEngine) -> Result<(), ConsensusError> {
        if self.validators.len() >= self.max_validators {
            return Err(ConsensusError::NetworkError(
                "Maximum validator capacity reached".to_string()
            ));
        }
        
        self.validators.push(validator);
        
        // Update memory usage statistics after adding a validator
        self.update_memory_stats();
        
        Ok(())
    }
    
    /// Estimates memory usage of the network
    fn estimate_memory_usage(&self) -> usize {
        let mut total_bytes = std::mem::size_of::<Network>();
        
        // Add size of validators vector
        total_bytes += self.validators.capacity() * std::mem::size_of::<PosygDcsEngine>();
        
        // Estimate validator memory usage
        for validator in &self.validators {
            // Using internal method, which in real code would be available
            // Here we use a rough estimate
            total_bytes += std::mem::size_of::<PosygDcsEngine>();
            total_bytes += validator.validator_id().len();
        }
        
        total_bytes
    }
    
    /// Updates memory usage statistics and logs if needed
    fn update_memory_stats(&mut self) {
        // Only recalculate periodically to reduce overhead
        if self.last_memory_report.elapsed() > Duration::from_secs(STATS_REPORTING_INTERVAL) {
            let memory_usage = self.estimate_memory_usage();
            
            // Log memory usage statistics
            println!(
                "[MEMORY STATS] Consensus network using ~{} KB, {} validators, {} blocks since last report",
                memory_usage / 1024,
                self.validators.len(),
                self.blocks_processed
            );
            
            self.last_memory_report = Instant::now();
            self.blocks_processed = 0;
            
            // Update global memory counter
            TOTAL_CONSENSUS_MEMORY_USAGE.store(memory_usage, Ordering::Relaxed);
        }
    }

    /// Selects a validator to propose the next block based on a weighted random selection.
    ///
    /// The weight is calculated as the sum of the validator's synergy score and a fraction of its stake.
    pub fn select_proposer(&self) -> usize {
        if self.validators.is_empty() {
            return 0;
        }
        
        let total_weight: f64 = self.validators
            .iter()
            .map(|v| v.get_synergy_score() + v.stake as f64 * 0.01)
            .sum();

        if total_weight == 0.0 {
            return thread_rng().gen_range(0..self.validators.len());
        }

        let mut rng = thread_rng();
        let random_weight = rng.gen_range(0.0..total_weight);

        let mut cumulative_weight = 0.0;
        for (i, validator) in self.validators.iter().enumerate() {
            cumulative_weight += validator.get_synergy_score() + validator.stake as f64 * 0.01;
            if cumulative_weight >= random_weight {
                return i;
            }
        }
        0
    }

    /// Executes a single round of the consensus protocol.
    ///
    /// A validator is selected to propose a block, which is then validated by others, and scores are updated.
    pub fn run_consensus_round(&mut self) -> Result<(), ConsensusError> {
        if self.validators.is_empty() {
            return Err(ConsensusError::NetworkError("No validators available".to_string()));
        }
        
        let proposer_index = self.select_proposer();
        
        // Attempt to create a block proposal
        let block_result = self.validators[proposer_index].propose_block();
        
        // If proposal fails, update scores and return early
        let block = match block_result {
            Ok(block) => block,
            Err(err) => {
                let proposer = &mut self.validators[proposer_index];
                let is_malicious = proposer.is_malicious();
                // Only count as violation if malicious
                proposer.update_scores(false, is_malicious);
                proposer.increment_proposed_blocks();
                
                // Count the block as processed even though it failed
                self.blocks_processed += 1;
                self.update_memory_stats();
                
                return Err(err);
            }
        };
        
        let is_malicious = self.validators[proposer_index].is_malicious();

        let mut is_valid = true;
        for (i, validator) in self.validators.iter().enumerate() {
            if i != proposer_index {
                if let Err(_) = validator.validate_block(&block) {
                    is_valid = false;
                    break;
                }
            }
        }

        let proposer = &mut self.validators[proposer_index];
        let violation_occurred = !is_valid && is_malicious;
        proposer.update_scores(is_valid, violation_occurred);
        if is_valid {
            proposer.increment_accepted_blocks();
        }
        proposer.increment_proposed_blocks();
        
        // Increment block counter and update memory stats
        self.blocks_processed += 1;
        self.update_memory_stats();
        
        Ok(())
    }
    
    /// Returns the maximum number of validators allowed in the network
    pub fn max_validators(&self) -> usize {
        self.max_validators
    }
    
    /// Sets the maximum number of validators allowed in the network
    pub fn set_max_validators(&mut self, max: usize) {
        self.max_validators = max;
    }
    
    /// Returns the estimated memory usage of this network
    pub fn get_memory_usage(&self) -> usize {
        self.estimate_memory_usage()
    }
}