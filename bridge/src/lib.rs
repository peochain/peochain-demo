/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST BRIDGE LIB
 * ----------------------------------------------------------------------------
 * This file re-exports the bridging functionality as a library crate.
 */

pub mod bridge;
pub mod bounded_string;
pub mod structured_types;

pub use bridge::{BridgeEngine, BridgeService, Transaction, OperationType};
pub use structured_types::{StructuredTransaction, StructuredBlock, TransactionType};
 