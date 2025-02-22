/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: BRIDGE MODULE TEST
 * ----------------------------------------------------------------------------
 * This file contains integration tests for the BridgeService, ensuring deposit
 * and withdrawal logic works as expected.
 */

 use peo_bridge::{BridgeEngine, BridgeService};

 #[test]
 fn test_bridge_deposit() {
     let mut service = BridgeService::new();
     let user = "0xUserBridge";
     let deposit_amount = 500;
 
     let result = service.deposit(user, deposit_amount);
     assert!(result.is_ok(), "Deposit should succeed");
     assert_eq!(
         service.get_balance(user),
         500,
         "Balance should reflect the deposited amount"
     );
 }
 
 #[test]
 fn test_bridge_withdraw() {
     let mut service = BridgeService::new();
     let user = "0xUserBridge";
     let initial_deposit = 1000;
     let withdraw_amount = 400;
 
     service.deposit(user, initial_deposit).unwrap();
     let result = service.withdraw(user, withdraw_amount);
     assert!(result.is_ok(), "Withdrawal should succeed");
     assert_eq!(
         service.get_balance(user),
         600,
         "Balance should reflect the withdrawn amount"
     );
 }
 
 #[test]
 fn test_bridge_insufficient_balance() {
     let mut service = BridgeService::new();
     let user = "0xUserBridge";
     service.deposit(user, 100).unwrap();
 
     let result = service.withdraw(user, 200);
     assert!(result.is_err(), "Withdrawal should fail due to insufficient balance");
     assert_eq!(
         service.get_balance(user),
         100,
         "Balance should remain unchanged"
     );
 }
 
 #[test]
 fn test_bridge_proof_verification() {
     let service = BridgeService::new();
     let valid_proof = b"non_empty_proof";
     let invalid_proof: &[u8] = &[];
 
     assert!(
         service.verify_proof(valid_proof),
         "Non-empty proof should be considered valid"
     );
     assert!(
         !service.verify_proof(invalid_proof),
         "Empty proof data should be considered invalid"
     );
 }
 