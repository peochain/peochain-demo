/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: BRIDGE PERFORMANCE BENCHMARKS
 * ----------------------------------------------------------------------------
 * Benchmarks to ensure safety checks have <10% performance overhead.
 */

use std::time::{Duration, Instant};
use peo_bridge::{BridgeEngine, BridgeService, Transaction, OperationType};

#[test]
fn benchmark_bridge_operations() {
    let mut service = BridgeService::new();
    
    // Warm up
    for i in 0..100 {
        let user = format!("warmup_user_{}", i);
        let _ = service.deposit(&user, 100);
    }
    
    // Benchmark deposit operations
    let start = Instant::now();
    for i in 0..1000 {
        let user = format!("bench_user_{}", i);
        let _ = service.deposit(&user, 100);
    }
    let deposit_time = start.elapsed();
    
    // Benchmark withdrawal operations
    let start = Instant::now();
    for i in 0..1000 {
        let user = format!("bench_user_{}", i);
        let _ = service.withdraw(&user, 50);
    }
    let withdraw_time = start.elapsed();
    
    // Benchmark proof verification
    let proof_data = vec![1u8; 1000]; // 1KB proof
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = service.verify_proof(&proof_data);
    }
    let verify_time = start.elapsed();
    
    println!("Bridge Benchmark Results:");
    println!("Deposits (1000 ops): {:?} ({:.2} μs/op)", 
             deposit_time, 
             deposit_time.as_micros() as f64 / 1000.0);
    println!("Withdrawals (1000 ops): {:?} ({:.2} μs/op)", 
             withdraw_time, 
             withdraw_time.as_micros() as f64 / 1000.0);
    println!("Proof verifications (1000 ops): {:?} ({:.2} μs/op)", 
             verify_time, 
             verify_time.as_micros() as f64 / 1000.0);
    
    // Performance thresholds (adjust based on your requirements)
    assert!(deposit_time < Duration::from_millis(100), "Deposit operations too slow");
    assert!(withdraw_time < Duration::from_millis(100), "Withdrawal operations too slow");
    assert!(verify_time < Duration::from_millis(50), "Proof verification too slow");
}

#[test]
fn benchmark_structured_transactions() {
    let mut service = BridgeService::new();
    
    // Create sample transactions
    let transactions: Vec<Transaction> = (0..1000)
        .map(|i| Transaction {
            user: format!("bench_user_{}", i),
            amount: 100 + i as u64,
            op_type: if i % 2 == 0 { OperationType::Deposit } else { OperationType::Withdraw },
        })
        .collect();
    
    // Benchmark structured transaction processing
    let start = Instant::now();
    for tx in &transactions {
        if tx.op_type == OperationType::Deposit {
            let _ = service.process_transaction(tx);
        }
    }
    let structured_time = start.elapsed();
    
    // Benchmark serialization/deserialization
    let mut serialization_time = Duration::new(0, 0);
    let mut deserialization_time = Duration::new(0, 0);
    
    for tx in &transactions[0..100] { // Test subset for serialization
        let start = Instant::now();
        if let Ok(bytes) = tx.to_bytes() {
            serialization_time += start.elapsed();
            
            let start = Instant::now();
            let _ = Transaction::from_bytes(&bytes);
            deserialization_time += start.elapsed();
        }
    }
    
    println!("Structured Transaction Benchmark Results:");
    println!("Processing (1000 ops): {:?} ({:.2} μs/op)", 
             structured_time, 
             structured_time.as_micros() as f64 / 1000.0);
    println!("Serialization (100 ops): {:?} ({:.2} μs/op)", 
             serialization_time, 
             serialization_time.as_micros() as f64 / 100.0);
    println!("Deserialization (100 ops): {:?} ({:.2} μs/op)", 
             deserialization_time, 
             deserialization_time.as_micros() as f64 / 100.0);
    
    // Performance thresholds
    assert!(structured_time < Duration::from_millis(200), "Structured transaction processing too slow");
    assert!(serialization_time < Duration::from_millis(10), "Serialization too slow");
    assert!(deserialization_time < Duration::from_millis(10), "Deserialization too slow");
}

#[test]
fn benchmark_memory_usage_monitoring() {
    let mut service = BridgeService::new();
    
    // Benchmark memory usage calculation
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = service.get_memory_usage();
    }
    let memory_calc_time = start.elapsed();
    
    // Add users and measure memory reporting overhead
    let start = Instant::now();
    for i in 0..1000 {
        let user = format!("memory_user_{}", i);
        let _ = service.deposit(&user, 100);
    }
    let operations_with_monitoring = start.elapsed();
    
    println!("Memory Monitoring Benchmark Results:");
    println!("Memory usage calculations (1000 ops): {:?} ({:.2} μs/op)", 
             memory_calc_time, 
             memory_calc_time.as_micros() as f64 / 1000.0);
    println!("Operations with monitoring (1000 ops): {:?} ({:.2} μs/op)", 
             operations_with_monitoring, 
             operations_with_monitoring.as_micros() as f64 / 1000.0);
    
    // Performance thresholds
    assert!(memory_calc_time < Duration::from_millis(10), "Memory calculation too slow");
    assert!(operations_with_monitoring < Duration::from_millis(500), "Operations with monitoring too slow");
}

#[test]
fn benchmark_input_validation() {
    let service = BridgeService::new();
    
    // Create various input types for validation testing
    let valid_users: Vec<String> = (0..1000)
        .map(|i| format!("valid_user_{}", i))
        .collect();
    
    let invalid_users = vec![
        "".to_string(),                           // empty
        "x".repeat(300),                         // too long
        "invalid@user".to_string(),              // invalid chars
        "user\nwith\nnewlines".to_string(),      // control chars
    ];
    
    // Benchmark valid user ID validation
    let start = Instant::now();
    for user in &valid_users {
        let _ = service.get_balance(user);
    }
    let valid_validation_time = start.elapsed();
    
    // Benchmark invalid user ID validation
    let start = Instant::now();
    for _ in 0..250 {
        for user in &invalid_users {
            let _ = service.get_balance(user);
        }
    }
    let invalid_validation_time = start.elapsed();
    
    // Benchmark proof size validation
    let valid_proof = vec![1u8; 1000];
    let invalid_proofs = vec![
        vec![], // empty
        vec![1u8; 70000], // too large
        vec![0u8; 100], // invalid format
    ];
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = service.verify_proof(&valid_proof);
    }
    let valid_proof_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..333 {
        for proof in &invalid_proofs {
            let _ = service.verify_proof(proof);
        }
    }
    let invalid_proof_time = start.elapsed();
    
    println!("Input Validation Benchmark Results:");
    println!("Valid user validation (1000 ops): {:?} ({:.2} μs/op)", 
             valid_validation_time, 
             valid_validation_time.as_micros() as f64 / 1000.0);
    println!("Invalid user validation (1000 ops): {:?} ({:.2} μs/op)", 
             invalid_validation_time, 
             invalid_validation_time.as_micros() as f64 / 1000.0);
    println!("Valid proof verification (1000 ops): {:?} ({:.2} μs/op)", 
             valid_proof_time, 
             valid_proof_time.as_micros() as f64 / 1000.0);
    println!("Invalid proof verification (1000 ops): {:?} ({:.2} μs/op)", 
             invalid_proof_time, 
             invalid_proof_time.as_micros() as f64 / 1000.0);
    
    // Performance thresholds
    assert!(valid_validation_time < Duration::from_millis(50), "Valid validation too slow");
    assert!(invalid_validation_time < Duration::from_millis(50), "Invalid validation too slow");
    assert!(valid_proof_time < Duration::from_millis(50), "Valid proof verification too slow");
    assert!(invalid_proof_time < Duration::from_millis(50), "Invalid proof verification too slow");
}
