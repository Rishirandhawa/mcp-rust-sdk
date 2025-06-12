# MCP Rust SDK

A comprehensive Rust implementation of the Model Context Protocol (MCP), providing both client and server capabilities with multiple transport mechanisms.

[![Crates.io](https://img.shields.io/crates/v/mcp-rust-sdk.svg)](https://crates.io/crates/mcp-rust-sdk)
[![Documentation](https://docs.rs/mcp-rust-sdk/badge.svg)](https://docs.rs/mcp-rust-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/rishirandhawa/mcp-rust-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/rishirandhawa/mcp-rust-sdk/actions/workflows/ci.yml)
[![Security Audit](https://github.com/rishirandhawa/mcp-rust-sdk/actions/workflows/security.yml/badge.svg)](https://github.com/rishirandhawa/mcp-rust-sdk/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/rishirandhawa/mcp-rust-sdk/branch/main/graph/badge.svg)](https://codecov.io/gh/rishirandhawa/mcp-rust-sdk)

## Overview

The Model Context Protocol (MCP) enables AI assistants to securely interact with external systems through a standardized protocol. This SDK provides:

- **Complete MCP Implementation**: Full support for MCP protocol specification
- **Multiple Transports**: STDIO, HTTP, and WebSocket transport layers
- **Type Safety**: Comprehensive Rust type system for all MCP constructs
- **Async/Await**: Built on Tokio for high-performance async operations
- **Extensible Architecture**: Easy to add custom tools, resources, and prompts

## Features

### 🚀 Core Features
- ✅ **MCP Server**: Create servers that expose tools, resources, and prompts
- ✅ **MCP Client**: Connect to MCP servers and invoke their capabilities
- ✅ **Transport Abstraction**: Pluggable transport system
- ✅ **Session Management**: Automatic reconnection and session handling
- ✅ **Real-time Communication**: Support for notifications and streaming

### 🔌 Transport Support
- ✅ **STDIO Transport**: Communication via standard input/output (default)
- ✅ **HTTP Transport**: RESTful communication with Server-Sent Events
- ✅ **WebSocket Transport**: Real-time bidirectional communication

### 🛠️ Built-in Components
- ✅ **Tool System**: Define and execute tools with JSON Schema validation
- ✅ **Resource Management**: Serve and access resources with URI-based addressing
- ✅ **Prompt Templates**: Create and manage reusable prompt templates
- ✅ **Error Handling**: Comprehensive error types and recovery mechanisms

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mcp-rust-sdk = "0.1.0"

# For specific transport support:
# mcp-rust-sdk = { version = "0.1.0", features = ["http"] }
# mcp-rust-sdk = { version = "0.1.0", features = ["websocket"] }
# mcp-rust-sdk = { version = "0.1.0", features = ["full"] }
```

### Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `stdio` | STDIO transport support | ✅ |
| `http` | HTTP transport with Axum and Reqwest | ❌ |
| `websocket` | WebSocket transport with Tokio-Tungstenite | ❌ |
| `validation` | JSON Schema validation | ❌ |
| `full` | All features enabled | ❌ |

## Quick Start

### Creating an MCP Server

```rust
use mcp_rust_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

// Define a simple echo tool
struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_rust_sdk::core::error::McpError> {
        let message = arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Hello, World!");

        Ok(ToolResult {
            content: vec![Content::text(format!("Echo: {}", message))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("echo-server".to_string(), "1.0.0".to_string());

    // Add the echo tool
    server.add_tool(
        "echo".to_string(),
        Some("Echoes back the provided message".to_string()),
        json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Message to echo back"
                }
            },
            "required": ["message"]
        }),
        EchoTool,
    ).await?;

    // Start the server with STDIO transport
    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    Ok(())
}
```

### Creating an MCP Client

```rust
use mcp_rust_sdk::{
    client::{McpClient, ClientSession},
    transport::stdio::StdioClientTransport,
};
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client and connect
    let client = McpClient::new("demo-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    let transport = StdioClientTransport::new("./echo-server".to_string()).await?;
    let init_result = session.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name, 
        init_result.server_info.version
    );

    // Call a tool
    let client = session.client();
    let client_guard = client.lock().await;
    
    let mut args = HashMap::new();
    args.insert("message".to_string(), json!("Hello from client!"));
    
    let result = client_guard.call_tool("echo".to_string(), Some(args)).await?;
    println!("Tool result: {:?}", result);

    Ok(())
}
```

## Transport Examples

### HTTP Server with Real-time Updates

```rust
use mcp_rust_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("http-server".to_string(), "1.0.0".to_string());
    
    // Add your tools, resources, and prompts here...
    
    // Start HTTP server with SSE support
    let transport = HttpServerTransport::new("0.0.0.0:3000");
    server.start(transport).await?;
    
    println!("HTTP server running on http://localhost:3000");
    println!("API endpoint: http://localhost:3000/mcp");
    println!("SSE events: http://localhost:3000/mcp/events");
    
    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

### WebSocket Server for Real-time Communication

```rust
use mcp_rust_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("ws-server".to_string(), "1.0.0".to_string());
    
    // Add your tools, resources, and prompts here...
    
    // Start WebSocket server
    let transport = WebSocketServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("WebSocket server running on ws://localhost:8080");
    
    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

## Advanced Features

### Resource Management

```rust
use mcp_rust_sdk::{
    core::resource::ResourceHandler,
    protocol::types::{ResourceInfo, ResourceContent},
};
use async_trait::async_trait;
use std::collections::HashMap;

struct FileSystemResource;

#[async_trait]
impl ResourceHandler for FileSystemResource {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> Result<Vec<ResourceContent>, mcp_rust_sdk::core::error::McpError> {
        // Implement file reading logic
        if uri.starts_with("file://") {
            let path = &uri[7..]; // Remove "file://" prefix
            match tokio::fs::read_to_string(path).await {
                Ok(content) => Ok(vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("text/plain".to_string()),
                    text: Some(content),
                    blob: None,
                }]),
                Err(e) => Err(mcp_rust_sdk::core::error::McpError::ResourceNotFound(uri.to_string())),
            }
        } else {
            Err(mcp_rust_sdk::core::error::McpError::ResourceNotFound(uri.to_string()))
        }
    }

    async fn list(&self) -> Result<Vec<ResourceInfo>, mcp_rust_sdk::core::error::McpError> {
        Ok(vec![
            ResourceInfo {
                uri: "file://".to_string(),
                name: "File System".to_string(),
                description: Some("Access to local file system".to_string()),
                mime_type: Some("text/plain".to_string()),
            }
        ])
    }
}

// Add to server
server.add_resource_detailed(
    ResourceInfo {
        uri: "file://".to_string(),
        name: "File System".to_string(),
        description: Some("Access to local file system".to_string()),
        mime_type: Some("text/plain".to_string()),
    },
    FileSystemResource,
).await?;
```

### Custom Prompt Templates

```rust
use mcp_rust_sdk::{
    core::prompt::PromptHandler,
    protocol::types::{PromptInfo, PromptMessage, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;

struct CodeReviewPrompt;

#[async_trait]
impl PromptHandler for CodeReviewPrompt {
    async fn get_prompt(&self, arguments: HashMap<String, serde_json::Value>) -> Result<Vec<PromptMessage>, mcp_rust_sdk::core::error::McpError> {
        let code = arguments.get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let language = arguments.get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        Ok(vec![
            PromptMessage {
                role: "system".to_string(),
                content: vec![Content::text(format!(
                    "You are an expert code reviewer. Please review the following {} code and provide detailed feedback.",
                    language
                ))],
            },
            PromptMessage {
                role: "user".to_string(),
                content: vec![Content::text(format!("Code to review:\n\n```{}\n{}\n```", language, code))],
            },
        ])
    }
}

// Add to server
server.add_prompt_detailed(
    PromptInfo {
        name: "code_review".to_string(),
        description: Some("Generate a code review prompt".to_string()),
        arguments: Some(vec![
            json!({
                "name": "code",
                "description": "The code to review",
                "required": true
            }),
            json!({
                "name": "language",
                "description": "Programming language",
                "required": false
            }),
        ]),
    },
    CodeReviewPrompt,
).await?;
```

### Session Management with Auto-Reconnection

```rust
use mcp_rust_sdk::{
    client::{McpClient, ClientSession},
    client::session::SessionConfig,
    transport::websocket::WebSocketClientTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("resilient-client".to_string(), "1.0.0".to_string());
    
    // Configure session with auto-reconnection
    let session_config = SessionConfig {
        auto_reconnect: true,
        max_reconnect_attempts: 10,
        reconnect_delay_ms: 2000,
        connection_timeout_ms: 15000,
        heartbeat_interval_ms: 30000,
        ..Default::default()
    };
    
    let session = ClientSession::with_config(client, session_config);
    
    // Connect with WebSocket transport
    let transport = WebSocketClientTransport::new("ws://localhost:8080").await?;
    
    match session.connect(transport).await {
        Ok(init_result) => {
            println!("Connected to: {} v{}", 
                init_result.server_info.name, 
                init_result.server_info.version
            );
            
            // Use the client...
            // Session will automatically reconnect if connection is lost
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
    
    Ok(())
}
```

## Examples

The repository includes comprehensive examples demonstrating various use cases:

### Basic Examples
- **`simple_server`**: Basic MCP server with a few tools
- **`echo_server`**: Echo server demonstrating tool handling
- **`client_example`**: Simple client connecting to a server
- **`database_server`**: Server providing database access tools

### Transport Examples
- **`http_server`**: HTTP server with REST API and SSE
- **`http_client`**: HTTP client with real-time updates
- **`websocket_server`**: WebSocket server for real-time communication
- **`websocket_client`**: WebSocket client with bidirectional messaging

Run examples with:

```bash
# STDIO examples (default)
cargo run --example simple_server
cargo run --example client_example

# HTTP examples
cargo run --example http_server --features http
cargo run --example http_client --features http

# WebSocket examples
cargo run --example websocket_server --features websocket
cargo run --example websocket_client --features websocket
```

## Architecture

### Core Components

```
mcp-rust-sdk/
├── core/               # Core MCP abstractions
│   ├── error.rs       # Error types and handling
│   ├── tool.rs        # Tool system
│   ├── resource.rs    # Resource management
│   └── prompt.rs      # Prompt templates
├── protocol/          # MCP protocol implementation
│   ├── types.rs       # JSON-RPC and MCP types
│   ├── messages.rs    # Message definitions
│   └── validation.rs  # Protocol validation
├── transport/         # Transport layer
│   ├── traits.rs      # Transport abstractions
│   ├── stdio.rs       # STDIO transport
│   ├── http.rs        # HTTP transport (optional)
│   └── websocket.rs   # WebSocket transport (optional)
├── server/            # Server implementation
│   ├── mcp_server.rs  # Main server logic
│   ├── handlers.rs    # Request handlers
│   └── lifecycle.rs   # Server lifecycle management
├── client/            # Client implementation
│   ├── mcp_client.rs  # Main client logic
│   └── session.rs     # Session management
└── utils/             # Utilities
    └── uri.rs         # URI handling utilities
```

### Transport Layer Design

The SDK uses a pluggable transport architecture:

1. **Transport Traits**: Common interface for all transports
2. **STDIO Transport**: Process-based communication (default)
3. **HTTP Transport**: RESTful API with Server-Sent Events
4. **WebSocket Transport**: Real-time bidirectional communication

Each transport handles:
- Connection management
- Message serialization/deserialization
- Error recovery
- Protocol negotiation

## Configuration

### Transport Configuration

```rust
use mcp_rust_sdk::transport::traits::TransportConfig;

let config = TransportConfig {
    connect_timeout_ms: Some(30_000),
    read_timeout_ms: Some(60_000),
    write_timeout_ms: Some(30_000),
    max_message_size: Some(1024 * 1024), // 1MB
    compression: true,
    headers: [("User-Agent".to_string(), "MyMCPClient/1.0".to_string())].into(),
    ..Default::default()
};
```

### Server Configuration

```rust
use mcp_rust_sdk::server::McpServer;

let mut server = McpServer::builder()
    .name("my-server")
    .version("1.0.0")
    .description("My custom MCP server")
    .max_connections(100)
    .request_timeout_ms(30_000)
    .enable_logging(true)
    .build();
```

### Client Configuration

```rust
use mcp_rust_sdk::client::session::SessionConfig;

let session_config = SessionConfig {
    auto_reconnect: true,
    max_reconnect_attempts: 5,
    reconnect_delay_ms: 1000,
    connection_timeout_ms: 15000,
    heartbeat_interval_ms: 20000,
    max_concurrent_requests: 10,
    request_timeout_ms: 30000,
    ..Default::default()
};
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with specific features
cargo test --features http
cargo test --features websocket
cargo test --features full

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_echo_tool
```

### Integration Tests

The SDK includes comprehensive integration tests:

```bash
# Test client-server communication
cargo test test_client_server_integration

# Test different transports
cargo test test_http_transport
cargo test test_websocket_transport

# Test error handling
cargo test test_error_recovery
```

## Performance

### Benchmarks

Run performance benchmarks:

```bash
cargo bench
```

Key performance characteristics:
- **Throughput**: >10,000 requests/second on modern hardware
- **Latency**: <1ms for simple tool calls (STDIO transport)
- **Memory**: Minimal allocation overhead with efficient serialization
- **Concurrent Connections**: Supports thousands of concurrent WebSocket connections

### Optimization Tips

1. **Choose the Right Transport**:
   - STDIO: Lowest latency for single client
   - HTTP: Best for RESTful integrations
   - WebSocket: Optimal for real-time multi-client scenarios

2. **Configure Timeouts Appropriately**:
   - Lower timeouts for faster failure detection
   - Higher timeouts for long-running operations

3. **Use Connection Pooling** (HTTP transport):
   - Reuse connections for better performance
   - Configure appropriate pool sizes

4. **Enable Compression** for large payloads:
   - Reduces bandwidth usage
   - May increase CPU usage

## Error Handling

The SDK provides comprehensive error handling:

```rust
use mcp_rust_sdk::core::error::{McpError, McpResult};

// All operations return McpResult<T>
match client.call_tool("my_tool".to_string(), None).await {
    Ok(result) => println!("Success: {:?}", result),
    Err(McpError::ToolNotFound(tool_name)) => {
        eprintln!("Tool '{}' not found", tool_name);
    }
    Err(McpError::ValidationError(msg)) => {
        eprintln!("Validation failed: {}", msg);
    }
    Err(McpError::Transport(msg)) => {
        eprintln!("Transport error: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

### Error Categories

- **Protocol Errors**: Invalid JSON-RPC or MCP messages
- **Transport Errors**: Connection, timeout, or communication issues
- **Validation Errors**: Schema validation failures
- **Resource Errors**: Resource not found or access denied
- **Tool Errors**: Tool execution failures
- **Authentication Errors**: Authentication or authorization failures

## Logging and Debugging

Enable logging for debugging:

```rust
use tracing_subscriber;

// Initialize logging
tracing_subscriber::fmt::init();

// Different log levels available:
// TRACE: Very detailed execution information
// DEBUG: Detailed information for debugging
// INFO: General information about execution
// WARN: Warning conditions
// ERROR: Error conditions
```

Set log level via environment variable:

```bash
RUST_LOG=debug cargo run --example simple_server
RUST_LOG=mcp_rust_sdk=trace cargo run --example client_example
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/rishirandhawa/mcp-rust-sdk.git
   cd mcp-rust-sdk
   ```

2. Install Rust and dependencies:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Run tests:
   ```bash
   cargo test --all-features
   ```

4. Check formatting and linting:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-features -- -D warnings
   ```

### Code Guidelines

- Follow Rust naming conventions
- Add comprehensive tests for new features
- Document public APIs with doc comments
- Use `cargo fmt` for consistent formatting
- Ensure `cargo clippy` passes without warnings

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Model Context Protocol](https://modelcontextprotocol.io/) specification
- [Tokio](https://tokio.rs/) for async runtime
- [Serde](https://serde.rs/) for serialization
- [Axum](https://github.com/tokio-rs/axum) for HTTP server support
- [Tungstenite](https://github.com/snapview/tungstenite-rs) for WebSocket support

## Links

- [Model Context Protocol Documentation](https://modelcontextprotocol.io/introduction)
- [API Documentation](https://docs.rs/mcp-rust-sdk)
- [Repository](https://github.com/rishirandhawa/mcp-rust-sdk)
- [Issues](https://github.com/rishirandhawa/mcp-rust-sdk/issues)
- [Crates.io](https://crates.io/crates/mcp-rust-sdk)
