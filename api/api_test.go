package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gorilla/mux"
)

func setupRouter() *mux.Router {
	r := mux.NewRouter()
	registerRoutes(r)
	return r
}

func TestHealthEndpoint(t *testing.T) {
	router := setupRouter()

	req := httptest.NewRequest(http.MethodGet, "/health", nil)
	rr := httptest.NewRecorder()

	router.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected status %d got %d", http.StatusOK, rr.Code)
	}

	var resp HealthCheckResponse
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Status != "ok" {
		t.Errorf("expected status 'ok' got '%s'", resp.Status)
	}
}

func TestStatusEndpoint(t *testing.T) {
	router := setupRouter()

	req := httptest.NewRequest(http.MethodGet, "/status", nil)
	rr := httptest.NewRecorder()

	router.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected status %d got %d", http.StatusOK, rr.Code)
	}

	var resp map[string]string
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp["network_status"] == "" {
		t.Error("network_status should not be empty")
	}
	if resp["description"] == "" {
		t.Error("description should not be empty")
	}
}
