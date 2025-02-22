/*!
 * -------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST CONSENSUS TEST
 * -------------------------------------------------------------------------
 * This file contains integration tests for the PoSyg + DCS engine.
 */

 use peo_consensus::{ConsensusEngine, PosygDcsEngine};

 #[test]
 fn test_consensus_engine_run() {
     let validator_id = String::from("validator_123");
     let engine = PosygDcsEngine::new(validator_id.clone());
     let result = engine.run();
     assert!(result.is_ok(), "Engine failed to run.");
 }
 
 #[test]
 fn test_synergy_score_increment() {
     let validator_id = String::from("validator_456");
     let mut engine = PosygDcsEngine::new(validator_id);
 
     // The synergy_score is initially 0 (encapsulated).
     assert_eq!(engine.synergy_score(), 0, "Initial synergy score should be 0.");
 
     // Increment synergy score by 100.
     engine.increment_synergy_score(100);
 
     assert_eq!(
         engine.synergy_score(),
         100,
         "Synergy score did not increment correctly."
     );
 }
 