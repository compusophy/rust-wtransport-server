# Rust WebTransport Game Server

A high-performance WebTransport game server built in Rust for real-time multiplayer games.

## Features

- **WebTransport Protocol**: Modern, low-latency communication over QUIC/HTTP3
- **Real-time Multiplayer**: Support for multiple concurrent players
- **Dual Communication**: Both reliable streams and unreliable datagrams
- **Player Management**: Automatic player joining/leaving with unique IDs
- **Game State Sync**: Automatic state synchronization for new connections
- **Message Broadcasting**: Efficient message distribution to all players

## Supported Game Messages

- `PlayerJoined` - Player connection events
- `PlayerLeft` - Player disconnection events  
- `PlayerMoved` - Real-time position updates
- `ChatMessage` - Text communication
- `GameState` - Full game state for new players

## Local Development

### Prerequisites

- Rust 1.81+ 
- Cargo

### Running Locally

```bash
# Clone the repository
git clone https://github.com/compusophy/rust-wtransport-server.git
cd rust-wtransport-server

# Build and run
cargo run
```

The server will start on `https://localhost:8080` with a self-signed certificate.

### Environment Variables

- `PORT` - Server port (defaults to 8080)
- `RUST_LOG` - Logging level (info, debug, warn, error)

## Cloud Deployment

### Google Cloud Run (Recommended for WebTransport)

Google Cloud Run provides experimental HTTP/3 support, making it ideal for WebTransport servers.

#### Prerequisites
- Google Cloud account with billing enabled
- Google Cloud CLI installed and authenticated
- Docker installed (for local testing)

#### Quick Deploy

1. **Enable APIs and set up project:**
```bash
# Set your project ID
export PROJECT_ID="your-project-id"
gcloud config set project $PROJECT_ID

# Enable required APIs
gcloud services enable cloudbuild.googleapis.com run.googleapis.com
```

2. **Deploy using the script:**
```bash
# Make script executable
chmod +x deploy-cloudrun.sh

# Deploy (replace with your actual project ID)
./deploy-cloudrun.sh your-project-id
```

3. **Get your server URL:**
The script will output your live server URL like:
```
https://rust-wtransport-server-hash.a.run.app
```

#### Manual Deploy
```bash
# Build and deploy manually
gcloud builds submit --tag gcr.io/$PROJECT_ID/rust-wtransport-server
gcloud run deploy rust-wtransport-server \
  --image gcr.io/$PROJECT_ID/rust-wtransport-server \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --port 8080 \
  --memory 512Mi \
  --timeout 3600
```

### Railway Deployment (Static Frontend)

Railway works great for static frontends but has limitations with WebTransport/HTTP3.

1. Connect your GitHub account to Railway
2. Create a new project from your repository
3. Railway will automatically detect and deploy the Rust application

## Architecture

- **Async Runtime**: Tokio for high-performance async I/O
- **WebTransport**: wtransport crate for WebTransport protocol support
- **Serialization**: Binary serialization with bincode for efficiency
- **Concurrency**: Thread-safe game state with Arc<RwLock<HashMap>>
- **Broadcasting**: Tokio broadcast channels for message distribution

## Client Integration

Clients can connect using the WebTransport API in modern browsers:

```javascript
// Connect to your deployed server
const transport = new WebTransport('https://your-server-url.run.app');
await transport.ready;

// Send messages via datagrams (fast, unreliable)
const writer = transport.datagrams.writable.getWriter();
await writer.write(messageData);

// Receive messages
const reader = transport.datagrams.readable.getReader();
const { value } = await reader.read();

// Or use streams for reliable delivery
const stream = await transport.createUnidirectionalStream();
const streamWriter = stream.writable.getWriter();
await streamWriter.write(messageData);
```

## Production Notes

- **HTTP/3 Support**: Google Cloud Run provides experimental HTTP/3
- **Scaling**: Configured for 1-10 instances with 1000 concurrent connections
- **Timeouts**: 3600s timeout for long-lived WebTransport connections
- **Resources**: 512Mi memory, 1 CPU core optimized for real-time processing

## License

MIT License - see LICENSE file for details.

## Contributing

Pull requests welcome! Please ensure tests pass and follow Rust formatting conventions. 