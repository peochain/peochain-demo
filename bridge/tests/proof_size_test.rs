/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: BRIDGE PROOF SIZE TEST
 * ----------------------------------------------------------------------------
 * This file contains tests that verify the proof size validation logic.
 */

use peo_bridge::{BridgeEngine, BridgeService};
use peo_bridge::bridge::ProofError;

/// Test to verify that proofs exceeding the maximum size are rejected
#[test]
fn test_oversized_proof() {
    let service = BridgeService::new();
    // Generate a large proof that exceeds the 64KB limit
    let large_proof = vec![1u8; 70000]; // 70KB
    
    let result = service.verify_proof(&large_proof);
    assert!(matches!(result, Err(ProofError::OversizedProof)), "Oversized proof should be rejected");
}

/// Test to verify that empty proofs are rejected
#[test]
fn test_empty_proof() {
    let service = BridgeService::new();
    let empty_proof: Vec<u8> = Vec::new();
    
    let result = service.verify_proof(&empty_proof);
    assert!(matches!(result, Err(ProofError::EmptyProof)), "Empty proof should be rejected");
}

/// Test to verify that valid-sized proofs are accepted
#[test]
fn test_valid_proof() {
    let service = BridgeService::new();
    
    // Generate a proof that is under the limit
    let valid_proof = vec![1u8; 1000]; // 1KB
    
    let result = service.verify_proof(&valid_proof);
    assert!(result.is_ok(), "Valid sized proof should be accepted");
}

/// Test to verify malformed proofs are rejected
#[test]
fn test_malformed_proof() {
    let service = BridgeService::new();
    
    // Generate a proof that has invalid format (starting with 0)
    let invalid_proof = vec![0u8, 1, 2, 3];
    
    let result = service.verify_proof(&invalid_proof);
    assert!(matches!(result, Err(ProofError::InvalidFormat)), "Malformed proof should be rejected");
}

/// Test memory usage monitoring
#[test]
fn test_memory_usage_reporting() {
    let mut service = BridgeService::new();
    
    // Get initial memory usage
    let initial_usage = service.get_memory_usage();
    
    // Add multiple users to trigger memory usage
    for i in 0..100 {
        let user = format!("user{}", i);
        let _ = service.deposit(&user, 100);
    }
    
    // Get updated memory usage
    let updated_usage = service.get_memory_usage();
    
    // Memory usage should increase
    assert!(updated_usage > initial_usage, "Memory usage should increase after adding users");
    
    println!("Initial memory usage: {} bytes", initial_usage);
    println!("Updated memory usage: {} bytes", updated_usage);
    println!("Difference: {} bytes", updated_usage - initial_usage);
}
