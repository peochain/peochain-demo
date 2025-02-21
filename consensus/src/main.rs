/*!
 * -------------------------------------------------------------------------
 * PEOCHAIN-DEMO: RUST CONSENSUS MAIN
 * -------------------------------------------------------------------------
 * This file launches the PoSyg + DCS consensus node.
 * It uses the core logic from lib.rs and demonstrates
 * how validators interact with the synergy scoring mechanism.
 *
 * PRINCIPLES:
 * - SRP: main.rs focuses solely on initialization and node startup.
 * - OCP: future consensus extensions can be integrated via traits.
 * - LSP: any new consensus mechanism can replace PoSyg, provided it
 *   respects the same interface.
 * - ISP: only relevant interfaces for node startup are included here.
 * - DIP: dependencies on synergy logic rely on abstractions provided in lib.rs.
 * - DRY: repeated routines (like synergy scoring) live in lib.rs.
 * - KISS: keep the node startup minimal and efficient.
 */

 use std::env;
 use std::process;
 use peo_consensus::{ConsensusEngine, PosygDcsEngine};
 
 fn main() {
     println!("PeoChain Consensus Node - Starting...");
 
     // Retrieve command-line arguments (if any) to configure node
     let args: Vec<String> = env::args().collect();
 
     // For demonstration purposes, check if we have a 'validator_id' argument
     let validator_id = match args.get(1) {
         Some(id) => id.to_string(),
         None => {
             eprintln!("No validator_id provided. Usage: ./consensus_node <validator_id>");
             process::exit(1);
         }
     };
 
     // Instantiate our PoSyg + DCS consensus engine
     let engine = PosygDcsEngine::new(validator_id);
 
     // Launch the consensus process
     match engine.run() {
         Ok(_) => {
             println!("Consensus node is up and running!");
         }
         Err(e) => {
             eprintln!("Consensus node failed to start: {}", e);
             process::exit(1);
         }
     }
 }
 