/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: BRIDGE MODULE TEST
 * ----------------------------------------------------------------------------
 * This file contains integration tests for the BridgeService, ensuring deposit
 * and withdrawal logic works as expected.
 */

 use peo_bridge::{BridgeEngine, BridgeService, Transaction, OperationType};
 use peo_bridge::bridge::ProofError;

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
     let valid_proof = vec![1u8, 2, 3]; // first byte non-zero
     let invalid_proof: Vec<u8> = vec![];
     let malformed_proof = vec![0u8, 1, 2]; // first byte zero
 
     assert!(service.verify_proof(&valid_proof).is_ok(), "Valid proof should be accepted");
     assert!(matches!(service.verify_proof(&invalid_proof), Err(ProofError::EmptyProof)), "Empty proof should be rejected");
     assert!(matches!(service.verify_proof(&malformed_proof), Err(ProofError::InvalidFormat)), "Malformed proof should be rejected");
 }
 
 #[test]
 fn test_bridge_user_validation() {
     let mut service = BridgeService::new();
     
     // Test empty user ID
     let result = service.deposit("", 100);
     assert!(result.is_err(), "Empty user ID should be rejected");
     
     // Test overly long user ID
     let long_user_id = "x".repeat(300);
     let result = service.deposit(&long_user_id, 100);
     assert!(result.is_err(), "Overly long user ID should be rejected");
     
     // Test invalid characters
     let invalid_user_id = "user@#$%";
     let result = service.deposit(invalid_user_id, 100);
     assert!(result.is_err(), "Invalid characters in user ID should be rejected");
 }
 
 #[test]
 fn test_bridge_overflow_protection() {
     let mut service = BridgeService::new();
     let user = "validUser";
     
     // Test deposit overflow protection
     service.deposit(user, u64::MAX - 100).unwrap();
     let result = service.deposit(user, 200);
     assert!(result.is_err(), "Deposit causing overflow should be rejected");
 }
 
 #[test]
 fn test_structured_transaction_deposit() {
     let mut service = BridgeService::new();
     let tx = Transaction {
         user: "0xUserBridgeStruct".to_string(),
         amount: 777,
         op_type: OperationType::Deposit,
     };
     let buf = tx.to_bytes().expect("Serialization should succeed");
     let tx2 = Transaction::from_bytes(&buf).expect("Deserialization should succeed");
     let result = service.process_transaction(&tx2);
     assert!(result.is_ok(), "Structured deposit should succeed");
     assert_eq!(service.get_balance(&tx2.user), 777, "Balance should match structured deposit");
 }
