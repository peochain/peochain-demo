/*!
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: GO API MAIN
 * ----------------------------------------------------------------------------
 * This file sets up the HTTP server and defines the primary entry point
 * for our REST API. It uses handlers from handlers.go.
 *
 * PRINCIPLES:
 * - SRP: main.go is strictly responsible for bootstrapping the server.
 * - OCP & LSP: adding new routes or replacing the handler logic doesn't require
 *   modifying existing code beyond registering new endpoints.
 * - ISP & DIP: the handlers can depend on interfaces or high-level logic, not
 *   concrete implementations from other modules.
 * - DRY & KISS: keep server setup simple and rely on well-documented
 *   standard libraries or minimal 3rd-party packages.
 */

 package main

 import (
	 "log"
	 "net/http"
	 "os"
	 "strconv"
 
	 "github.com/gorilla/mux"
 )
 
 func main() {
	 // Retrieve port from environment or default to 8080.
	 port := getEnvVarOrDefault("API_PORT", "8080")
 
	 // Create a new mux router.
	 r := mux.NewRouter()
 
	 // Register routes from handlers.go
	 registerRoutes(r)
 
	 // Start the HTTP server.
	 log.Printf("API service listening on port %s", port)
	 if err := http.ListenAndServe(":"+port, r); err != nil {
		 log.Fatalf("Failed to start API service: %v", err)
	 }
 }
 
 // getEnvVarOrDefault returns the value of the given environment variable or falls back.
 func getEnvVarOrDefault(envKey, defaultVal string) string {
	 val := os.Getenv(envKey)
	 if val == "" {
		 return defaultVal
	 }
	 return val
 }
 
 // parseIntEnvVar attempts to parse an int from the environment.
 // If parsing fails, returns the fallback value.
 func parseIntEnvVar(envKey string, fallback int) int {
	 valStr := os.Getenv(envKey)
	 if valStr == "" {
		 return fallback
	 }
	 val, err := strconv.Atoi(valStr)
	 if err != nil {
		 return fallback
	 }
	 return val
 }
 