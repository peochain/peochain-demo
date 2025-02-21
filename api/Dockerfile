# ------------------------------------------------------------------
# PEOCHAIN-DEMO: API DOCKERFILE
# ------------------------------------------------------------------
# This Dockerfile creates a container for the Go API service.

FROM golang:1.19 as builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Build the API service
RUN CGO_ENABLED=0 GOOS=linux go build -o peochain-api main.go

FROM alpine:3.17
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/peochain-api /usr/local/bin/peochain-api

# Expose the default API port
EXPOSE 8080

ENTRYPOINT ["peochain-api"]
