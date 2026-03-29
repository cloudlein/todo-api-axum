# --- Builder Stage ---
FROM rust:1.85-slim-bullseye AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy your source code
COPY . .

# Build the application for release
RUN cargo build --release

# --- Runtime Stage ---
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Install runtime dependencies (OpenSSL is required for database connections)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/todo_api_axum .

# Expose the application port
EXPOSE 8000

# Set the binary as the entrypoint
CMD ["./todo_api_axum"]
