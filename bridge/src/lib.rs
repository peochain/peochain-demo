/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST BRIDGE LIB
 * ----------------------------------------------------------------------------
 * This file re-exports the bridging functionality as a library crate.
 */

 pub mod bridge;

 pub use bridge::{BridgeEngine, BridgeService};
 