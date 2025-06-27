/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: EVM MEMORY MONITORING TESTS
 * ----------------------------------------------------------------------------
 * This file contains tests for memory monitoring in the EVM module.
 */

use peo_evm::{BasicEvmExecutor, EvmExecutor};

#[test]
fn test_evm_memory_monitoring() {
    let mut executor = BasicEvmExecutor::new();
    
    // Get initial memory usage
    let initial_usage = executor.get_memory_usage();
    let initial_account_count = executor.get_account_count();
    
    // Create a number of accounts to trigger memory usage
    for i in 0..50 {
        let address = format!("0xAccount{}", i);
        let _ = executor.set_balance(&address, i * 100);
    }
    
    // Get updated memory usage
    let updated_usage = executor.get_memory_usage();
    let updated_account_count = executor.get_account_count();
    
    // Memory usage should increase
    assert!(updated_usage > initial_usage, "Memory usage should increase after adding accounts");
    assert_eq!(updated_account_count, 50, "Should have 50 accounts");
    
    println!("Initial memory usage: {} bytes", initial_usage);
    println!("Updated memory usage: {} bytes", updated_usage);
    println!("Memory per account: ~{} bytes", (updated_usage - initial_usage) / 50);
}

#[test]
fn test_transaction_size_limits() {
    let mut executor = BasicEvmExecutor::new();
    
    // Set up accounts
    let _ = executor.set_balance("0xSender", 1000);
    let _ = executor.set_balance("0xReceiver", 0);
    
    // Create a transaction with data that is too large
    let large_data = vec![0u8; 40000]; // 40KB (over 32KB limit)
    
    // Should be rejected
    let result = executor.execute_transaction("0xSender", "0xReceiver", &large_data);
    assert!(result.is_err(), "Oversized transaction should be rejected");
    
    if let Err(err) = result {
        assert!(err.contains("too large"), "Error should mention size");
    }
    
    // Create a valid sized transaction
    let valid_data = vec![1u8; 16000]; // 16KB (under 32KB limit)
    
    // Should be accepted
    let result = executor.execute_transaction("0xSender", "0xReceiver", &valid_data);
    assert!(result.is_ok(), "Valid sized transaction should be accepted");
}

#[test]
fn test_blockchain_size_tracking() {
    let mut executor = BasicEvmExecutor::new();
    
    // Set up accounts
    let _ = executor.set_balance("0xSender", 1000);
    let _ = executor.set_balance("0xReceiver", 0);
    
    // Initial blockchain size
    let initial_block_number = 0;
    
    // Execute multiple transactions to trigger blockchain growth
    for i in 0..10 {
        let data = vec![i as u8; 1000]; // 1KB per transaction
        let block_size = 1000 + 100; // data + overhead
        
        let _ = executor.execute_transaction("0xSender", "0xReceiver", &data);
        executor.increment_block_number(block_size);
    }
    
    // Block number should have increased
    assert!(executor.block_number > initial_block_number, "Block number should increase");
    
    // Memory usage should include blockchain size
    let memory_usage = executor.get_memory_usage();
    assert!(memory_usage > 10000, "Memory usage should include blockchain size");
}
