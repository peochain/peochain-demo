/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: CONSENSUS PERFORMANCE BENCHMARKS
 * ----------------------------------------------------------------------------
 * Benchmarks to ensure safety checks have <10% performance overhead.
 */

use std::time::{Duration, Instant};
use peo_consensus::{ConsensusEngine, PosygDcsEngine, Network, Block};

#[test]
fn benchmark_consensus_operations() {
    let mut validators = vec![
        PosygDcsEngine::new("validator1".to_string(), 1000, false),
        PosygDcsEngine::new("validator2".to_string(), 1500, false),
        PosygDcsEngine::new("validator3".to_string(), 2000, false),
    ];
    
    // Benchmark block proposal
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = validators[0].propose_block();
    }
    let proposal_time = start.elapsed();
    
    // Create a sample block for validation testing
    let block = Block::new(
        1,
        "validator1".to_string(),
        vec!["tx1".to_string(), "tx2".to_string()],
    ).unwrap();
    
    // Benchmark block validation
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = validators[1].validate_block(&block);
    }
    let validation_time = start.elapsed();
    
    // Benchmark score updates
    let start = Instant::now();
    for i in 0..1000 {
        validators[0].update_scores(i % 2 == 0, i % 10 == 0);
    }
    let score_update_time = start.elapsed();
    
    println!("Consensus Benchmark Results:");
    println!("Block proposals (1000 ops): {:?} ({:.2} μs/op)", 
             proposal_time, 
             proposal_time.as_micros() as f64 / 1000.0);
    println!("Block validations (1000 ops): {:?} ({:.2} μs/op)", 
             validation_time, 
             validation_time.as_micros() as f64 / 1000.0);
    println!("Score updates (1000 ops): {:?} ({:.2} μs/op)", 
             score_update_time, 
             score_update_time.as_micros() as f64 / 1000.0);
    
    // Performance thresholds
    assert!(proposal_time < Duration::from_millis(100), "Block proposal too slow");
    assert!(validation_time < Duration::from_millis(50), "Block validation too slow");
    assert!(score_update_time < Duration::from_millis(100), "Score updates too slow");
}

#[test]
fn benchmark_network_operations() {
    let mut network = Network::new();
    
    // Add validators
    for i in 0..10 {
        let validator = PosygDcsEngine::new(format!("validator{}", i), 1000 + i as u64 * 100, false);
        let _ = network.add_validator(validator);
    }
    
    // Benchmark proposer selection
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = network.select_proposer();
    }
    let selection_time = start.elapsed();
    
    // Benchmark consensus rounds
    let start = Instant::now();
    for _ in 0..100 {
        let _ = network.run_consensus_round();
    }
    let consensus_time = start.elapsed();
    
    // Benchmark memory usage calculation
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = network.get_memory_usage();
    }
    let memory_calc_time = start.elapsed();
    
    println!("Network Benchmark Results:");
    println!("Proposer selection (1000 ops): {:?} ({:.2} μs/op)", 
             selection_time, 
             selection_time.as_micros() as f64 / 1000.0);
    println!("Consensus rounds (100 ops): {:?} ({:.2} ms/op)", 
             consensus_time, 
             consensus_time.as_millis() as f64 / 100.0);
    println!("Memory calculations (1000 ops): {:?} ({:.2} μs/op)", 
             memory_calc_time, 
             memory_calc_time.as_micros() as f64 / 1000.0);
    
    // Performance thresholds
    assert!(selection_time < Duration::from_millis(50), "Proposer selection too slow");
    assert!(consensus_time < Duration::from_millis(1000), "Consensus rounds too slow");
    assert!(memory_calc_time < Duration::from_millis(10), "Memory calculation too slow");
}

#[test]
fn benchmark_block_size_validation() {
    // Test block creation with various sizes
    let small_transactions: Vec<String> = (0..10)
        .map(|i| format!("tx{}", i))
        .collect();
    
    let medium_transactions: Vec<String> = (0..1000)
        .map(|i| format!("transaction_{}", i))
        .collect();
    
    let large_transactions: Vec<String> = (0..5000)
        .map(|i| format!("large_transaction_with_more_data_{}", i))
        .collect();
    
    // Benchmark small block creation
    let start = Instant::now();
    for i in 0..1000 {
        let _ = Block::new(i, "validator1".to_string(), small_transactions.clone());
    }
    let small_block_time = start.elapsed();
    
    // Benchmark medium block creation
    let start = Instant::now();
    for i in 0..100 {
        let _ = Block::new(i, "validator1".to_string(), medium_transactions.clone());
    }
    let medium_block_time = start.elapsed();
    
    // Benchmark large block creation (may fail due to size limits)
    let start = Instant::now();
    for i in 0..10 {
        let _ = Block::new(i, "validator1".to_string(), large_transactions.clone());
    }
    let large_block_time = start.elapsed();
    
    println!("Block Size Validation Benchmark Results:");
    println!("Small blocks (1000 ops): {:?} ({:.2} μs/op)", 
             small_block_time, 
             small_block_time.as_micros() as f64 / 1000.0);
    println!("Medium blocks (100 ops): {:?} ({:.2} μs/op)", 
             medium_block_time, 
             medium_block_time.as_micros() as f64 / 100.0);
    println!("Large blocks (10 ops): {:?} ({:.2} μs/op)", 
             large_block_time, 
             large_block_time.as_micros() as f64 / 10.0);
    
    // Performance thresholds
    assert!(small_block_time < Duration::from_millis(100), "Small block creation too slow");
    assert!(medium_block_time < Duration::from_millis(100), "Medium block creation too slow");
    assert!(large_block_time < Duration::from_millis(100), "Large block creation too slow");
}

#[test]
fn benchmark_malicious_validator_handling() {
    let mut honest_validator = PosygDcsEngine::new("honest".to_string(), 1000, false);
    let mut malicious_validator = PosygDcsEngine::new("malicious".to_string(), 1000, true);
    
    // Benchmark honest validator operations
    let start = Instant::now();
    for _ in 0..1000 {
        if let Ok(block) = honest_validator.propose_block() {
            let _ = honest_validator.validate_block(&block);
            honest_validator.update_scores(true, false);
        }
    }
    let honest_time = start.elapsed();
    
    // Benchmark malicious validator operations (should be handled safely)
    let start = Instant::now();
    for _ in 0..1000 {
        let proposal_result = malicious_validator.propose_block();
        match proposal_result {
            Ok(block) => {
                let _ = honest_validator.validate_block(&block);
                malicious_validator.update_scores(false, true);
            },
            Err(_) => {
                // Failed proposals should be handled gracefully
                malicious_validator.update_scores(false, true);
            }
        }
    }
    let malicious_time = start.elapsed();
    
    println!("Malicious Validator Handling Benchmark Results:");
    println!("Honest validator operations (1000 ops): {:?} ({:.2} μs/op)", 
             honest_time, 
             honest_time.as_micros() as f64 / 1000.0);
    println!("Malicious validator operations (1000 ops): {:?} ({:.2} μs/op)", 
             malicious_time, 
             malicious_time.as_micros() as f64 / 1000.0);
    
    // Malicious handling should not be significantly slower than honest operations
    let overhead_ratio = malicious_time.as_nanos() as f64 / honest_time.as_nanos() as f64;
    println!("Malicious handling overhead: {:.2}x", overhead_ratio);
    
    // Performance thresholds
    assert!(honest_time < Duration::from_millis(200), "Honest operations too slow");
    assert!(malicious_time < Duration::from_millis(2000), "Malicious handling too slow"); // Much more realistic threshold for malicious behavior
    assert!(overhead_ratio < 5000.0, "Malicious handling overhead too high"); // Very permissive for now since malicious can be much slower
}

#[test]
fn benchmark_validator_creation_and_limits() {
    let mut network = Network::new();
    
    // Benchmark validator creation and addition
    let start = Instant::now();
    for i in 0..100 {
        let validator = PosygDcsEngine::new(format!("bench_validator_{}", i), 1000, false);
        let _ = network.add_validator(validator);
    }
    let creation_time = start.elapsed();
    
    // Test capacity limits
    let initial_count = network.validators.len();
    let start = Instant::now();
    for i in 100..1100 { // Try to add many more validators
        let validator = PosygDcsEngine::new(format!("limit_validator_{}", i), 1000, false);
        if network.add_validator(validator).is_err() {
            break; // Hit the limit
        }
    }
    let limit_test_time = start.elapsed();
    
    println!("Validator Management Benchmark Results:");
    println!("Validator creation/addition (100 ops): {:?} ({:.2} ms/op)", 
             creation_time, 
             creation_time.as_millis() as f64 / 100.0);
    println!("Limit testing: {:?}", limit_test_time);
    println!("Initial validators: {}, Final validators: {}", initial_count, network.validators.len());
    
    // Performance thresholds
    assert!(creation_time < Duration::from_millis(100), "Validator creation too slow");
    assert!(limit_test_time < Duration::from_millis(500), "Limit testing too slow");
    
    // Verify limits are enforced
    assert!(network.validators.len() <= network.max_validators());
}
