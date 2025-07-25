# Use the latest official Rust image for building and running
FROM rust:latest

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

# Expose the port (Railway will override this with PORT env var)
EXPOSE 4433

# Run the binary (as root to avoid permission issues)
CMD ["game_server"] 