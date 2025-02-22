/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: GO API HANDLERS
 * ----------------------------------------------------------------------------
 * This file defines HTTP handlers that orchestrate various network operations,
 * such as starting nodes, deploying smart contracts, or monitoring the system.
 *
 * PRINCIPLES:
 * - SRP: each handler focuses on a single type of request.
 * - OCP: new endpoints can be added with minimal changes.
 * - DRY & KISS: re-use common logic for JSON encoding, error handling, etc.
 */

 package main

 import (
	 "encoding/json"
	 "fmt"
	 "log"
	 "math/rand"
	 "net/http"
	 "time"
 
	 "github.com/gorilla/mux"
 )
 
 // registerRoutes wires up our HTTP routes.
 func registerRoutes(r *mux.Router) {
	 r.HandleFunc("/health", healthCheckHandler).Methods("GET")
	 r.HandleFunc("/start-node", startNodeHandler).Methods("POST")
	 r.HandleFunc("/deploy-contract", deployContractHandler).Methods("POST")
	 r.HandleFunc("/status", statusHandler).Methods("GET")
 }
 
 // HealthCheckResponse represents a basic health check JSON.
 type HealthCheckResponse struct {
	 Status string `json:"status"`
 }
 
 // NodeRequest represents a request to start or manage a node.
 type NodeRequest struct {
	 NodeType string `json:"node_type"`
	 ValidatorID string `json:"validator_id,omitempty"`
 }
 
 // ContractRequest represents a request to deploy a smart contract.
 type ContractRequest struct {
	 ContractType string `json:"contract_type"`
	 Parameters   string `json:"parameters,omitempty"`
 }
 
 // healthCheckHandler verifies API is running.
 func healthCheckHandler(w http.ResponseWriter, r *http.Request) {
	 w.Header().Set("Content-Type", "application/json")
	 json.NewEncoder(w).Encode(HealthCheckResponse{Status: "ok"})
 }
 
 // startNodeHandler orchestrates starting a new node (e.g., consensus, EVM, bridge).
 // In a real scenario, it might invoke Docker, systemd, or direct OS calls.
 func startNodeHandler(w http.ResponseWriter, r *http.Request) {
	 w.Header().Set("Content-Type", "application/json")
 
	 var req NodeRequest
	 if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		 http.Error(w, fmt.Sprintf("Invalid request body: %v", err), http.StatusBadRequest)
		 return
	 }
 
	 // Perform a stub action to "start" the node
	 log.Printf("Starting node of type: %s, validator_id: %s", req.NodeType, req.ValidatorID)
 
	 // In a real system, we could call scripts, Docker CLI, or orchestrators here.
	 response := map[string]string{
		 "message":   "Node start request received",
		 "node_type": req.NodeType,
	 }
	 json.NewEncoder(w).Encode(response)
 }
 
 // deployContractHandler demonstrates how one might deploy a contract to the EVM module.
 func deployContractHandler(w http.ResponseWriter, r *http.Request) {
	 w.Header().Set("Content-Type", "application/json")
 
	 var req ContractRequest
	 if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		 http.Error(w, fmt.Sprintf("Invalid contract request: %v", err), http.StatusBadRequest)
		 return
	 }
 
	 log.Printf("Deploying contract: %s with parameters: %s", req.ContractType, req.Parameters)
 
	 // Stub success response with a pseudo address
	 pseudoAddress := fmt.Sprintf("0xDEMO%x", rand.Uint64())
	 response := map[string]string{
		 "message":         "Contract deploy request received",
		 "contract_type":   req.ContractType,
		 "contract_address": pseudoAddress,
	 }
	 json.NewEncoder(w).Encode(response)
 }
 
 // statusHandler returns a mock status of the entire system.
 func statusHandler(w http.ResponseWriter, r *http.Request) {
	 w.Header().Set("Content-Type", "application/json")
 
	 // Stub data: random "uptime" or "synced" state
	 rand.Seed(time.Now().UnixNano())
	 statuses := []string{"running", "synced", "pending", "maintenance"}
	 currentStatus := statuses[rand.Intn(len(statuses))]
 
	 response := map[string]string{
		 "network_status": currentStatus,
		 "description":    "Demo status endpoint for the PeoChain network",
	 }
	 json.NewEncoder(w).Encode(response)
 }
 