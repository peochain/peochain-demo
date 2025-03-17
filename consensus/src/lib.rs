// src/lib.rs

use rand::Rng;
use rand::thread_rng;

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
    /// Represents a network-related failure, with a descriptive message.
    NetworkError(String),
}

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
    /// List of transactions included in the block.
    pub transactions: Vec<String>,
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
        Self {
            validator_id,
            synergy_score: 0.0,
            stake,
            proposed_blocks: 0,
            accepted_blocks: 0,
            violations: 0,
            is_malicious,
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

    /// Increments the count of proposed blocks by one.
    pub fn increment_proposed_blocks(&mut self) {
        self.proposed_blocks += 1;
    }

    /// Increments the count of accepted blocks by one.
    pub fn increment_accepted_blocks(&mut self) {
        self.accepted_blocks += 1;
    }
}

impl ConsensusEngine for PosygDcsEngine {
    fn propose_block(&self) -> Result<Block, ConsensusError> {
        let block = if self.is_malicious {
            Block {
                id: 0,
                proposer: self.validator_id.clone(),
                transactions: vec!["invalid_tx".to_string()],
            }
        } else {
            Block {
                id: self.proposed_blocks + 1,
                proposer: self.validator_id.clone(),
                transactions: vec![],
            }
        };
        Ok(block)
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
            self.violations += 1;
            BASE_PENALTY * MULTIPLIER.powi(self.violations as i32 - 1)
        } else {
            0.0
        };

        self.synergy_score += ALPHA * h + BETA * e + GAMMA * v - DELTA * p;
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
/// };
/// network.run_consensus_round();
/// let v1 = &network.validators[0];
/// assert!(v1.proposed_blocks() <= 1);
/// ```
pub struct Network {
    pub validators: Vec<PosygDcsEngine>,
}

impl Network {
    /// Selects a validator to propose the next block based on a weighted random selection.
    ///
    /// The weight is calculated as the sum of the validator's synergy score and a fraction of its stake.
    pub fn select_proposer(&self) -> usize {
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
    pub fn run_consensus_round(&mut self) {
        let proposer_index = self.select_proposer();
        let (block, is_malicious);
        {
            let proposer = &mut self.validators[proposer_index];
            block = proposer.propose_block().unwrap();
            is_malicious = proposer.is_malicious();
        }

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
    }
}