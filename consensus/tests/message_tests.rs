/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: CONSENSUS MESSAGE TESTING
 * ----------------------------------------------------------------------------
 * Tests for structured consensus message types with bounds checking.
 */

use peo_consensus::messages::*;

#[test]
fn test_consensus_transaction_creation() {
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        100,
        "transfer".to_string(),
        1,
        21000,
        20,
    );
    
    assert!(tx.is_ok());
    let tx = tx.unwrap();
    assert_eq!(tx.from, "validator1");
    assert_eq!(tx.to, "validator2");
    assert_eq!(tx.amount, 100);
}

#[test]
fn test_consensus_transaction_validation() {
    // Test empty from address
    let tx = ConsensusTransaction::new(
        "".to_string(),
        "validator2".to_string(),
        100,
        "transfer".to_string(),
        1,
        21000,
        20,
    );
    assert!(tx.is_ok()); // Creation succeeds
    assert!(tx.unwrap().validate().is_err()); // But validation fails
    
    // Test zero gas limit
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        100,
        "transfer".to_string(),
        1,
        0, // zero gas limit
        20,
    );
    assert!(tx.is_ok());
    assert!(tx.unwrap().validate().is_err());
}

#[test]
fn test_consensus_transaction_size_limits() {
    // Test oversized from address
    let long_address = "x".repeat(200);
    let tx = ConsensusTransaction::new(
        long_address,
        "validator2".to_string(),
        100,
        "transfer".to_string(),
        1,
        21000,
        20,
    );
    assert!(tx.is_err());
    
    // Test oversized data
    let large_data = "x".repeat(40000);
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        100,
        large_data,
        1,
        21000,
        20,
    );
    assert!(tx.is_err());
}

#[test]
fn test_consensus_transaction_serialization() {
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        100,
        "transfer".to_string(),
        1,
        21000,
        20,
    ).unwrap();
    
    // Test serialization
    let bytes = tx.to_bytes().unwrap();
    assert!(!bytes.is_empty());
    
    // Test deserialization
    let tx2 = ConsensusTransaction::from_bytes(&bytes).unwrap();
    assert_eq!(tx.from, tx2.from);
    assert_eq!(tx.to, tx2.to);
    assert_eq!(tx.amount, tx2.amount);
}

#[test]
fn test_consensus_block_creation() {
    let block = ConsensusBlock::new(
        1,
        "validator1".to_string(),
        vec!["tx1".to_string(), "tx2".to_string()],
        1640995200, // timestamp
        "prev_hash".to_string(),
        "merkle_root".to_string(),
        123456,
    );
    
    assert!(block.is_ok());
    let block = block.unwrap();
    assert_eq!(block.id, 1);
    assert_eq!(block.proposer, "validator1");
    assert_eq!(block.transactions.len(), 2);
}

#[test]
fn test_consensus_block_size_limits() {
    // Test too many transactions
    let many_transactions: Vec<String> = (0..15000)
        .map(|i| format!("tx{}", i))
        .collect();
    
    let block = ConsensusBlock::new(
        1,
        "validator1".to_string(),
        many_transactions,
        1640995200,
        "prev_hash".to_string(),
        "merkle_root".to_string(),
        123456,
    );
    assert!(block.is_err());
    
    // Test oversized transaction
    let large_tx = "x".repeat(40000);
    let block = ConsensusBlock::new(
        1,
        "validator1".to_string(),
        vec![large_tx],
        1640995200,
        "prev_hash".to_string(),
        "merkle_root".to_string(),
        123456,
    );
    assert!(block.is_err());
}

#[test]
fn test_consensus_block_serialization() {
    let block = ConsensusBlock::new(
        1,
        "validator1".to_string(),
        vec!["tx1".to_string(), "tx2".to_string()],
        1640995200,
        "prev_hash".to_string(),
        "merkle_root".to_string(),
        123456,
    ).unwrap();
    
    // Test serialization
    let bytes = block.to_bytes().unwrap();
    assert!(!bytes.is_empty());
    
    // Test deserialization
    let block2 = ConsensusBlock::from_bytes(&bytes).unwrap();
    assert_eq!(block.id, block2.id);
    assert_eq!(block.proposer, block2.proposer);
    assert_eq!(block.transactions, block2.transactions);
}

#[test]
fn test_consensus_message_types() {
    let block = ConsensusBlock::new(
        1,
        "validator1".to_string(),
        vec!["tx1".to_string()],
        1640995200,
        "prev_hash".to_string(),
        "merkle_root".to_string(),
        123456,
    ).unwrap();
    
    // Test BlockProposal message
    let msg = ConsensusMessage::BlockProposal {
        block,
        proposer_signature: "signature".to_string(),
    };
    
    let bytes = msg.to_bytes().unwrap();
    let msg2 = ConsensusMessage::from_bytes(&bytes).unwrap();
    
    match (msg, msg2) {
        (ConsensusMessage::BlockProposal { block: b1, .. }, 
         ConsensusMessage::BlockProposal { block: b2, .. }) => {
            assert_eq!(b1.id, b2.id);
            assert_eq!(b1.proposer, b2.proposer);
        },
        _ => panic!("Message type mismatch"),
    }
    
    // Test BlockVote message
    let vote_msg = ConsensusMessage::BlockVote {
        block_id: 1,
        validator_id: "validator1".to_string(),
        vote: true,
        signature: "vote_signature".to_string(),
    };
    
    let bytes = vote_msg.to_bytes().unwrap();
    let vote_msg2 = ConsensusMessage::from_bytes(&bytes).unwrap();
    
    match vote_msg2 {
        ConsensusMessage::BlockVote { block_id, validator_id, vote, .. } => {
            assert_eq!(block_id, 1);
            assert_eq!(validator_id, "validator1");
            assert_eq!(vote, true);
        },
        _ => panic!("Message type mismatch"),
    }
}

#[test]
fn test_consensus_message_size_limits() {
    // Test oversized message
    let large_block = ConsensusBlock::new(
        1,
        "validator1".to_string(),
        vec!["x".repeat(1000); 5000], // Large number of large transactions
        1640995200,
        "prev_hash".to_string(),
        "merkle_root".to_string(),
        123456,
    );
    
    // Should fail during creation due to size limits
    assert!(large_block.is_err());
}
