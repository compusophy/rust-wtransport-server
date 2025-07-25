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

- Rust 1.70+ 
- Cargo

### Running Locally

```bash
# Clone the repository
git clone https://github.com/compusophy/rust-wtransport-server.git
cd rust-wtransport-server

# Build and run
cargo run
```

The server will start on `https://localhost:4433` with a self-signed certificate.

### Environment Variables

- `PORT` - Server port (defaults to 4433)
- `RUST_LOG` - Logging level (info, debug, warn, error)

## Railway Deployment

This server is configured for one-click deployment on [Railway](https://railway.app):

1. Fork this repository
2. Connect your GitHub account to Railway
3. Create a new project from your repository
4. Railway will automatically detect and deploy the Rust application

The server will be automatically configured with:
- Environment-based port binding
- Production-optimized builds
- Automatic TLS certificate generation

## Architecture

- **Async Runtime**: Tokio for high-performance async I/O
- **WebTransport**: wtransport crate for WebTransport protocol support
- **Serialization**: Binary serialization with bincode for efficiency
- **Concurrency**: Thread-safe game state with Arc<RwLock<HashMap>>
- **Broadcasting**: Tokio broadcast channels for message distribution

## Client Integration

Clients can connect using the WebTransport API in modern browsers:

```javascript
const transport = new WebTransport('https://your-server.railway.app');
await transport.ready;

// Send messages via datagrams
const writer = transport.datagrams.writable.getWriter();
await writer.write(messageData);

// Receive messages
const reader = transport.datagrams.readable.getReader();
const { value } = await reader.read();
```

## License

MIT License - see LICENSE file for details.

## Contributing

Pull requests welcome! Please ensure tests pass and follow Rust formatting conventions. 