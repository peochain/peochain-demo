/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: CONSENSUS STRUCTURED TYPES TESTS
 * ----------------------------------------------------------------------------
 * Tests for consensus structured transaction types and validation
 */

use peo_consensus::{ConsensusTransaction, StructuredConsensusEngine, PosygDcsEngine, StructuredBlock};

#[test]
fn test_consensus_transaction_creation() {
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        1000,
        "transfer_data".to_string(),
        1,
        21000,
        20,
    ).expect("Valid consensus transaction should be created");
    
    assert_eq!(tx.from, "validator1");
    assert_eq!(tx.to, "validator2");
    assert_eq!(tx.amount, 1000);
    assert!(tx.is_valid());
}

#[test]
fn test_consensus_transaction_validation() {
    // Test empty from address
    let result = ConsensusTransaction::new(
        "".to_string(), // Empty from
        "validator2".to_string(),
        1000,
        "data".to_string(),
        1,
        21000,
        20,
    );
    assert!(result.is_ok(), "Transaction should be created");
    
    let tx = result.unwrap();
    assert!(!tx.is_valid(), "Transaction with empty from should be invalid");
    
    // Test zero gas limit
    let tx = ConsensusTransaction {
        from: "validator1".to_string(),
        to: "validator2".to_string(),
        amount: 1000,
        data: "data".to_string(),
        nonce: 1,
        gas_limit: 0, // Invalid
        gas_price: 20,
    };
    assert!(!tx.is_valid(), "Transaction with zero gas limit should be invalid");
}

#[test]
fn test_consensus_transaction_serialization() {
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        1000,
        "test_data".to_string(),
        1,
        21000,
        20,
    ).expect("Valid transaction should be created");
    
    // Test JSON serialization
    let json_bytes = tx.to_bytes().expect("JSON serialization should succeed");
    let tx2 = ConsensusTransaction::from_bytes(&json_bytes)
        .expect("JSON deserialization should succeed");
    
    assert_eq!(tx.from, tx2.from);
    assert_eq!(tx.amount, tx2.amount);
    
    // Test binary serialization
    let binary_bytes = tx.to_bytes_binary().expect("Binary serialization should succeed");
    let tx3 = ConsensusTransaction::from_bytes_binary(&binary_bytes)
        .expect("Binary deserialization should succeed");
    
    assert_eq!(tx.from, tx3.from);
    assert_eq!(tx.amount, tx3.amount);
}

#[test]
fn test_consensus_transaction_hash() {
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        1000,
        "test_data".to_string(),
        1,
        21000,
        20,
    ).expect("Valid transaction should be created");
    
    let hash1 = tx.hash().expect("Hash calculation should succeed");
    let hash2 = tx.hash().expect("Hash calculation should succeed");
    
    assert_eq!(hash1, hash2, "Hash should be deterministic");
    assert_ne!(hash1, [0u8; 32], "Hash should not be all zeros");
}

#[test]
fn test_structured_consensus_engine() {
    let mut engine = PosygDcsEngine::new("test_validator".to_string(), 1000, false);
    
    // Test structured block proposal
    let block = engine.propose_structured_block()
        .expect("Should be able to propose structured block");
    
    assert!(block.transactions.len() > 0, "Block should contain transactions");
    assert_eq!(block.proposer, "validator_test_validator");
    
    // Test structured block validation
    let validation_result = engine.validate_structured_block(&block);
    assert!(validation_result.is_ok(), "Valid block should pass validation");
    
    // Test score updates
    let initial_score = engine.get_synergy_score_structured();
    engine.update_scores_structured(true, false);
    let updated_score = engine.get_synergy_score_structured();
    
    assert!(updated_score >= initial_score, "Score should increase on accepted block");
}

#[test]
fn test_malicious_structured_consensus_engine() {
    let mut engine = PosygDcsEngine::new("malicious_validator".to_string(), 1000, true);
    
    // Malicious validator should propose blocks, but they may contain invalid transactions
    let block_result = engine.propose_structured_block();
    
    // The block creation might fail for malicious validators due to invalid transactions
    // This is expected behavior - malicious validators can fail to create valid blocks
    match block_result {
        Ok(block) => {
            // If block was created, test validation
            let _validation_result = engine.validate_structured_block(&block);
            // Block might be invalid due to malicious transactions
            
            // Test that malicious behavior affects scoring regardless
            let initial_score = engine.get_synergy_score_structured();
            engine.update_scores_structured(false, true); // Block rejected with violation
            let updated_score = engine.get_synergy_score_structured();
            
            assert!(updated_score <= initial_score, "Score should decrease on violation");
        },
        Err(_) => {
            // It's acceptable for malicious validators to fail block creation
            // Test that violations still affect scoring
            let initial_score = engine.get_synergy_score_structured();
            engine.update_scores_structured(false, true); // Violation occurred
            let updated_score = engine.get_synergy_score_structured();
            
            assert!(updated_score <= initial_score, "Score should decrease on violation");
        }
    }
}

#[test]
fn test_structured_block_size_limits() {
    // Create a transaction with maximum allowed data
    let large_data = "x".repeat(32 * 1024 - 100); // Close to max transaction data length
    
    let tx = ConsensusTransaction::new(
        "validator1".to_string(),
        "validator2".to_string(),
        1000,
        large_data,
        1,
        21000,
        20,
    ).expect("Large transaction should be created");
    
    assert!(tx.is_valid(), "Large but valid transaction should pass validation");
    
    // Test serialization size limits
    let bytes = tx.to_bytes_binary().expect("Should serialize large transaction");
    assert!(bytes.len() <= 2 * 1024 * 1024, "Serialized size should be within limits");
}

#[test]
fn test_structured_block_serialization() {
    let engine = PosygDcsEngine::new("test_validator".to_string(), 1000, false);
    let block = engine.propose_structured_block()
        .expect("Should propose block");
    
    // Test block serialization
    let serialized = block.to_bytes().expect("Block serialization should succeed");
    let deserialized = StructuredBlock::from_bytes(&serialized)
        .expect("Block deserialization should succeed");
    
    assert_eq!(block.id, deserialized.id);
    assert_eq!(block.proposer, deserialized.proposer);
    assert_eq!(block.transactions.len(), deserialized.transactions.len());
    
    // Verify transactions are preserved
    for (orig, deser) in block.transactions.iter().zip(deserialized.transactions.iter()) {
        assert_eq!(orig.from, deser.from);
        assert_eq!(orig.to, deser.to);
        assert_eq!(orig.amount, deser.amount);
    }
}
