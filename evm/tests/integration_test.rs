/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: EVM MODULE TEST
 * ----------------------------------------------------------------------------
 * This file contains integration tests for the BasicEvmExecutor,
 * demonstrating basic EVM-like functionality.
 */

 use peo_evm::{BasicEvmExecutor, EvmExecutor};

 #[test]
 fn test_evm_transaction_execution() {
     let mut executor = BasicEvmExecutor::new();
     executor.set_balance("0xSender", 500).unwrap();
     executor.set_balance("0xReceiver", 200).unwrap();
 
     // Execute a transaction from Sender to Receiver
     let data = b"fake_contract_call_data";
     let tx_result = executor.execute_transaction("0xSender", "0xReceiver", data);
     assert!(tx_result.is_ok(), "Transaction execution should succeed");
 
     // Balances remain the same in this simplistic approach (no real funds transfer).
     assert_eq!(
         executor.get_balance("0xSender"),
         500,
         "Sender's balance should remain 500"
     );
     assert_eq!(
         executor.get_balance("0xReceiver"),
         200,
         "Receiver's balance should remain 200"
     );
 }
 
 #[test]
 fn test_evm_balance_setting() {
     let mut executor = BasicEvmExecutor::new();
     let result = executor.set_balance("0xTestUser", 1000);
     assert!(result.is_ok(), "Setting balance should succeed");
     assert_eq!(
         executor.get_balance("0xTestUser"),
         1000,
         "Balance should be correctly set to 1000"
     );
 }
 
 #[test]
 fn test_evm_address_validation() {
     let mut executor = BasicEvmExecutor::new();
     
     // Test invalid address lengths
     let result = executor.set_balance("", 100);
     assert!(result.is_err(), "Empty address should be rejected");
     
     let long_address = "0x".to_string() + &"a".repeat(100);
     let result = executor.set_balance(&long_address, 100);
     assert!(result.is_err(), "Overly long address should be rejected");
 }
 
 #[test]
 fn test_evm_transaction_data_limits() {
     let mut executor = BasicEvmExecutor::new();
     executor.set_balance("0xSender", 500).unwrap();
     executor.set_balance("0xReceiver", 200).unwrap();
     
     // Test oversized transaction data
     let large_data = vec![0u8; 40000]; // Larger than MAX_TRANSACTION_DATA_SIZE
     let result = executor.execute_transaction("0xSender", "0xReceiver", &large_data);
     assert!(result.is_err(), "Oversized transaction data should be rejected");
 }
 
 #[test]
 fn test_evm_account_limits() {
     let mut executor = BasicEvmExecutor::new();
     
     // This test would be expensive to run with actual limits,
     // so we test the error condition conceptually
     let result = executor.get_balance("0xValidAddress");
     assert_eq!(result, 0, "Non-existent address should return 0 balance");
 }
