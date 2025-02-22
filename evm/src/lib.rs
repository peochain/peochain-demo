/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST EVM LIB
 * ----------------------------------------------------------------------------
 * This file re-exports EVM logic for external usage (e.g., from other modules).
 */

// Let the compiler know we have a sibling file named evm_core.rs
mod evm_core;

// Re-export the main types so other crates can use them.
pub use evm_core::{BasicEvmExecutor, EvmExecutor};
