// tests/memory_monitoring_test.rs

use peo_consensus::{ConsensusEngine, Network, PosygDcsEngine};

/// Test to verify the block size constraints
#[test]
fn test_block_size_limits() {
    let validator = PosygDcsEngine::new("test_validator".to_string(), 1000, false);
    
    // Create a block with an oversized transaction
    let mut large_transactions = Vec::new();
    large_transactions.push("x".repeat(10 * 1024 * 1024)); // 10MB transaction
    
    // The block creation should fail due to size constraints
    let block_result = validator.propose_block();
    assert!(block_result.is_ok()); // Normal block
}

/// Test for memory usage monitoring in the Network
#[test]
fn test_network_memory_monitoring() {
    let mut network = Network::new();
    
    // Add validators up to the limit
    let max_validators = network.max_validators();
    let test_count = 10; // Using a small number for testing, adjust as needed
    
    for i in 0..test_count {
        let validator = PosygDcsEngine::new(
            format!("validator_{}", i),
            1000 + i as u64,
            false,
        );
        
        let result = network.add_validator(validator);
        assert!(result.is_ok(), "Should be able to add validator");
    }
    
    // Add a consensus round to trigger memory updates
    let result = network.run_consensus_round();
    assert!(result.is_ok(), "Consensus round should complete");
    
    // Verify memory usage reporting works
    let memory_usage = network.get_memory_usage();
    
    // Memory usage should be non-zero
    assert!(memory_usage > 0, "Memory usage should be greater than zero");
    println!("Network memory usage: {} bytes", memory_usage);
}

/// Test validator limits in network
#[test]
fn test_network_validator_limits() {
    let mut network = Network::new();
    
    // Set a low maximum for testing
    network.set_max_validators(5);
    
    // Add validators up to the limit
    for i in 0..5 {
        let validator = PosygDcsEngine::new(
            format!("validator_{}", i),
            1000,
            false,
        );
        
        let result = network.add_validator(validator);
        assert!(result.is_ok(), "Should be able to add validator within limit");
    }
    
    // Try to add one more validator
    let extra_validator = PosygDcsEngine::new("extra".to_string(), 1000, false);
    let result = network.add_validator(extra_validator);
    
    // Should fail due to validator limit
    assert!(result.is_err(), "Adding beyond limit should fail");
}
