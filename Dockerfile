# Use the official Rust image for building and running
FROM rust:1.77-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the application in release mode
RUN cargo build --release

# Move binary to a standard location
RUN cp target/release/game_server /usr/local/bin/game_server

# Create a non-root user
RUN useradd -m appuser && chown appuser:appuser /usr/local/bin/game_server

# Switch to non-root user
USER appuser

# Expose the port (Railway will override this with PORT env var)
EXPOSE 4433

# Run the binary
CMD ["game_server"] 