# Getting Started with MCP Rust SDK

Welcome to MCP Rust SDK! This guide will help you build your first MCP server and client in minutes.

## Installation

Add MCP Rust SDK to your `Cargo.toml`:

```toml
[dependencies]
mcp-rust-sdk = "0.1.0"

# For specific features:
mcp-rust-sdk = { version = "0.1.0", features = ["http"] }
mcp-rust-sdk = { version = "0.1.0", features = ["websocket"] }
mcp-rust-sdk = { version = "0.1.0", features = ["full"] }
```

### Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `stdio` | STDIO transport support | ✅ |
| `http` | HTTP transport with SSE | ❌ |
| `websocket` | WebSocket transport | ❌ |
| `validation` | JSON Schema validation | ❌ |
| `full` | All features enabled | ❌ |

## Your First MCP Server

Let's create a simple echo server:

```rust
use mcp_protocol_sdk::{
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
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
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
    // Create server
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

    // Start the server
    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    Ok(())
}
```

## Your First MCP Client

Now let's create a client to connect to the server:

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::stdio::StdioClientTransport,
};
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = McpClient::new("demo-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    // Connect to server (assuming server binary is at ./echo-server)
    let transport = StdioClientTransport::new("./echo-server".to_string()).await?;
    let init_result = session.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name, 
        init_result.server_info.version
    );

    // List available tools
    let client = session.client();
    let client_guard = client.lock().await;
    
    let tools = client_guard.list_tools().await?;
    println!("Available tools: {:?}", tools);

    // Call the echo tool
    let mut args = HashMap::new();
    args.insert("message".to_string(), json!("Hello from client!"));
    
    let result = client_guard.call_tool("echo".to_string(), Some(args)).await?;
    println!("Tool result: {:?}", result);

    Ok(())
}
```

## Next Steps

### Add More Tools

```rust
// Define a calculator tool
struct CalculatorTool;

#[async_trait]
impl ToolHandler for CalculatorTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, McpError> {
        let a = arguments.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let b = arguments.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let operation = arguments.get("operation").and_then(|v| v.as_str()).unwrap_or("add");

        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => if b != 0.0 { a / b } else { return Err(McpError::ToolExecutionError("Division by zero".to_string())); },
            _ => return Err(McpError::ToolExecutionError("Unknown operation".to_string())),
        };

        Ok(ToolResult {
            content: vec![Content::text(format!("{}", result))],
            is_error: None,
        })
    }
}

// Add to server
server.add_tool(
    "calculator".to_string(),
    Some("Performs basic mathematical operations".to_string()),
    json!({
        "type": "object",
        "properties": {
            "a": {"type": "number", "description": "First number"},
            "b": {"type": "number", "description": "Second number"},
            "operation": {
                "type": "string",
                "enum": ["add", "subtract", "multiply", "divide"],
                "description": "Operation to perform"
            }
        },
        "required": ["a", "b", "operation"]
    }),
    CalculatorTool,
).await?;
```

### Add Resources

```rust
use mcp_protocol_sdk::{
    core::resource::ResourceHandler,
    protocol::types::{ResourceInfo, ResourceContent},
};

struct FileSystemResource;

#[async_trait]
impl ResourceHandler for FileSystemResource {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> Result<Vec<ResourceContent>, McpError> {
        if uri.starts_with("file://") {
            let path = &uri[7..];
            match tokio::fs::read_to_string(path).await {
                Ok(content) => Ok(vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("text/plain".to_string()),
                    text: Some(content),
                    blob: None,
                }]),
                Err(_) => Err(McpError::ResourceNotFound(uri.to_string())),
            }
        } else {
            Err(McpError::ResourceNotFound(uri.to_string()))
        }
    }

    async fn list(&self) -> Result<Vec<ResourceInfo>, McpError> {
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

### Different Transports

#### HTTP Server

```rust
use mcp_protocol_sdk::transport::http::HttpServerTransport;

let transport = HttpServerTransport::new("0.0.0.0:3000");
server.start(transport).await?;

// Access via:
// HTTP API: http://localhost:3000/mcp
// SSE Events: http://localhost:3000/mcp/events
```

#### WebSocket Server

```rust
use mcp_protocol_sdk::transport::websocket::WebSocketServerTransport;

let transport = WebSocketServerTransport::new("0.0.0.0:8080");
server.start(transport).await?;

// Connect via: ws://localhost:8080
```

## Common Patterns

### Error Handling

```rust
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

### Session Configuration

```rust
use mcp_protocol_sdk::client::session::SessionConfig;

let session_config = SessionConfig {
    auto_reconnect: true,
    max_reconnect_attempts: 5,
    reconnect_delay_ms: 1000,
    connection_timeout_ms: 15000,
    heartbeat_interval_ms: 20000,
    ..Default::default()
};

let session = ClientSession::with_config(client, session_config);
```

### Logging

```rust
use tracing_subscriber;

// Initialize logging
tracing_subscriber::fmt::init();

// Set log level via environment
// RUST_LOG=debug cargo run
```

## What's Next?

- [**Examples**](examples.md) - More complete examples
- [**Transports Guide**](transports.md) - Transport layer details
- [**Architecture**](architecture.md) - System design overview
- [**API Reference**](https://docs.rs/mcp-rust-sdk) - Complete API docs

## Need Help?

- [GitHub Issues](https://github.com/YOUR_USERNAME/mcp-rust-sdk/issues) - Bug reports and feature requests
- [Discussions](https://github.com/YOUR_USERNAME/mcp-rust-sdk/discussions) - Community support
- [Examples](https://github.com/YOUR_USERNAME/mcp-rust-sdk/tree/main/examples) - Real code examples
