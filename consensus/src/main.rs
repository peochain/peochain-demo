// src/main.rs

use peo_consensus::{ConsensusEngine, Network, PosygDcsEngine};

// Main function to simulate the consensus mechanism over 5 rounds.
fn main() {
    let mut network = Network::new();
    
    // Add validators to the network
    let validators = vec![
        PosygDcsEngine::new("validator1".to_string(), 1000, false),
        PosygDcsEngine::new("validator2".to_string(), 1500, false),
        PosygDcsEngine::new("validator3".to_string(), 800, true),
    ];
    
    for validator in validators {
        if let Err(e) = network.add_validator(validator) {
            eprintln!("Failed to add validator: {:?}", e);
            return;
        }
    }

    for round in 1..=5 {
        println!("Starting consensus round {}", round);
        if let Err(e) = network.run_consensus_round() {
            eprintln!("Consensus round {} failed: {:?}", round, e);
            continue;
        }

        for (i, validator) in network.validators.iter().enumerate() {
            println!(
                "Validator {} ({}): Synergy Score = {:.2}, Violations = {}, Proposed = {}, Accepted = {}",
                i + 1,
                validator.validator_id(),
                validator.get_synergy_score(),
                validator.violations(),
                validator.proposed_blocks(),
                validator.accepted_blocks()
            );
        }
        println!();
    }
}