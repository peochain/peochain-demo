/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: STRUCTURED TRANSACTION TESTS
 * ----------------------------------------------------------------------------
 * Tests for structured transaction types with size constraints and validation
 */

use peo_bridge::{StructuredTransaction, StructuredBlock, TransactionType};

#[test]
fn test_structured_transaction_creation() {
    let tx = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        1000,
        1,
        21000,
        20,
        TransactionType::Transfer,
        vec![],
    ).expect("Valid transaction should be created");
    
    assert_eq!(tx.amount, 1000);
    assert_eq!(tx.tx_type, TransactionType::Transfer);
}

#[test]
fn test_structured_transaction_validation() {
    // Test invalid from address (too long)
    let result = StructuredTransaction::new(
        "x".repeat(50), // Too long
        "0x1234567890123456789012345678901234567890".to_string(),
        1000,
        1,
        21000,
        20,
        TransactionType::Transfer,
        vec![],
    );
    assert!(result.is_err(), "Transaction with long address should fail");
    
    // Test zero gas limit
    let result = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        1000,
        1,
        0, // Invalid gas limit
        20,
        TransactionType::Transfer,
        vec![],
    );
    assert!(result.is_err(), "Transaction with zero gas limit should fail");
}

#[test]
fn test_structured_transaction_serialization() {
    let tx = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        1000,
        1,
        21000,
        20,
        TransactionType::Transfer,
        vec![1, 2, 3, 4],
    ).expect("Valid transaction should be created");
    
    let bytes = tx.to_bytes().expect("Serialization should succeed");
    let tx2 = StructuredTransaction::from_bytes(&bytes).expect("Deserialization should succeed");
    
    assert_eq!(tx.from, tx2.from);
    assert_eq!(tx.to, tx2.to);
    assert_eq!(tx.amount, tx2.amount);
    assert_eq!(tx.tx_type, tx2.tx_type);
    assert_eq!(tx.data, tx2.data);
}

#[test]
fn test_structured_transaction_size_limits() {
    // Test transaction with large data
    let large_data = vec![0u8; 70_000]; // 70KB
    let result = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        1000,
        1,
        21000,
        20,
        TransactionType::SmartContract,
        large_data,
    );
    assert!(result.is_err(), "Transaction with large data should fail");
}

#[test]
fn test_structured_block_creation() {
    let tx1 = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        500,
        1,
        21000,
        20,
        TransactionType::Transfer,
        vec![],
    ).expect("Valid transaction should be created");
    
    let tx2 = StructuredTransaction::new(
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        "0x1234567890123456789012345678901234567890".to_string(),
        300,
        2,
        21000,
        25,
        TransactionType::Transfer,
        vec![],
    ).expect("Valid transaction should be created");
    
    let block = StructuredBlock::new(
        1,
        1234567890,
        [0u8; 32],
        "0xvalidator123456789012345678901234567890".to_string(),
        vec![tx1, tx2],
    ).expect("Valid block should be created");
    
    assert_eq!(block.block_number, 1);
    assert_eq!(block.transactions.len(), 2);
}

#[test]
fn test_structured_block_serialization() {
    let tx = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        1000,
        1,
        21000,
        20,
        TransactionType::Transfer,
        vec![],
    ).expect("Valid transaction should be created");
    
    let block = StructuredBlock::new(
        1,
        1234567890,
        [0u8; 32],
        "0xvalidator123456789012345678901234567890".to_string(),
        vec![tx],
    ).expect("Valid block should be created");
    
    let bytes = block.to_bytes().expect("Block serialization should succeed");
    let block2 = StructuredBlock::from_bytes(&bytes).expect("Block deserialization should succeed");
    
    assert_eq!(block.block_number, block2.block_number);
    assert_eq!(block.proposer, block2.proposer);
    assert_eq!(block.transactions.len(), block2.transactions.len());
}

#[test]
fn test_structured_block_transaction_limits() {
    // Create too many transactions
    let mut transactions = Vec::new();
    for i in 0..1500 { // Exceeds MAX_TRANSACTIONS (1000)
        let tx = StructuredTransaction::new(
            format!("0x{:040x}", i),
            format!("0x{:040x}", i + 1),
            100,
            i as u64,
            21000,
            20,
            TransactionType::Transfer,
            vec![],
        ).expect("Valid transaction should be created");
        transactions.push(tx);
    }
    
    let result = StructuredBlock::new(
        1,
        1234567890,
        [0u8; 32],
        "0xvalidator123456789012345678901234567890".to_string(),
        transactions,
    );
    
    assert!(result.is_err(), "Block with too many transactions should fail");
}

#[test]
fn test_transaction_hash_consistency() {
    let tx = StructuredTransaction::new(
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        1000,
        1,
        21000,
        20,
        TransactionType::Transfer,
        vec![1, 2, 3],
    ).expect("Valid transaction should be created");
    
    let hash1 = tx.hash().expect("Hash calculation should succeed");
    let hash2 = tx.hash().expect("Hash calculation should succeed");
    
    assert_eq!(hash1, hash2, "Transaction hash should be consistent");
}
