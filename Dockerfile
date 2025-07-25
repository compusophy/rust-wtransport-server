# Use the official Rust image
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser

# Copy the binary from builder stage
COPY --from=builder /app/target/release/game_server /usr/local/bin/game_server

# Change ownership to appuser
RUN chown appuser:appuser /usr/local/bin/game_server

# Switch to non-root user
USER appuser

# Expose the port (Railway will override this with PORT env var)
EXPOSE 4433

# Run the binary
CMD ["game_server"] 