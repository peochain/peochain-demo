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
     executor.set_balance("0xSender", 500);
     executor.set_balance("0xReceiver", 200);
 
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
     executor.set_balance("0xTestUser", 1000);
     assert_eq!(
         executor.get_balance("0xTestUser"),
         1000,
         "Balance should be correctly set to 1000"
     );
 }
 