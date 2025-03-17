// tests/integration_test.rs

use peo_consensus::{ConsensusEngine, Network, PosygDcsEngine};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

// Integration test to verify consensus behavior over multiple rounds.
#[test]
fn test_consensus_rounds() {
    let mut network = Network {
        validators: vec![
            PosygDcsEngine::new("v1".to_string(), 1000, false),
            PosygDcsEngine::new("v2".to_string(), 1000, false),
            PosygDcsEngine::new("v3".to_string(), 1000, true),
        ],
    };
    network.validators[2].set_synergy_score(10.0);

    let seed = [42; 32];
    let mut rng = StdRng::from_seed(seed);
    for _ in 0..10 {
        let proposer_index = {
            let total_weight: f64 = network
                .validators
                .iter()
                .map(|v| v.get_synergy_score() + v.stake() as f64 * 0.01)
                .sum();
            if total_weight == 0.0 {
                rng.gen_range(0..network.validators.len())
            } else {
                let random_weight = rng.gen_range(0.0..total_weight);
                let mut cumulative_weight = 0.0;
                let mut i = 0;
                loop {
                    if i >= network.validators.len() {
                        break 0;
                    }
                    let validator = &network.validators[i];
                    cumulative_weight += validator.get_synergy_score() + validator.stake() as f64 * 0.01;
                    if cumulative_weight >= random_weight {
                        break i;
                    }
                    i += 1;
                }
            }
        };

        let (block, is_malicious);
        {
            let proposer = &mut network.validators[proposer_index];
            block = proposer.propose_block().unwrap();
            is_malicious = proposer.is_malicious();
        }

        let mut is_valid = true;
        for (i, validator) in network.validators.iter().enumerate() {
            if i != proposer_index {
                if let Err(_) = validator.validate_block(&block) {
                    is_valid = false;
                    break;
                }
            }
        }

        let proposer = &mut network.validators[proposer_index];
        let violation_occurred = !is_valid && is_malicious;
        proposer.update_scores(is_valid, violation_occurred);
        if is_valid {
            proposer.increment_accepted_blocks();
        }
        proposer.increment_proposed_blocks();
    }

    let v1 = &network.validators[0];
    let v2 = &network.validators[1];
    let v3 = &network.validators[2];

    assert!(v3.proposed_blocks() > 0, "Malicious validator should have proposed at least once");
    assert!(v3.violations() > 0, "Malicious validator should have violations");
    assert!(v3.get_synergy_score() < 10.0, "Malicious validator's score should decrease");
    assert!(v1.get_synergy_score() > v3.get_synergy_score(), "v1 should have higher score than v3");
    assert!(v2.get_synergy_score() > v3.get_synergy_score(), "v2 should have higher score than v3");
    assert_eq!(v1.violations(), 0, "v1 should have no violations");
    assert_eq!(v2.violations(), 0, "v2 should have no violations");
}