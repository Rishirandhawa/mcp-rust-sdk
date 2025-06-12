# Examples

This page showcases real-world examples of using MCP Rust SDK in different scenarios.

## Basic Examples

### Simple Echo Server

A minimal server that echoes back messages.

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

struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let message = arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("No message provided");

        Ok(ToolResult {
            content: vec![Content::text(format!("Echo: {}", message))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("echo-server".to_string(), "1.0.0".to_string());

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

    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    Ok(())
}
```

### Basic Client

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::stdio::StdioClientTransport,
};
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("demo-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    let transport = StdioClientTransport::new("./echo-server".to_string()).await?;
    let init_result = session.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name, 
        init_result.server_info.version
    );

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

### HTTP Server with REST API

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

struct WeatherTool;

#[async_trait]
impl ToolHandler for WeatherTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let city = arguments.get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        // Simulate weather API call
        let weather_data = json!({
            "city": city,
            "temperature": 22,
            "condition": "Sunny",
            "humidity": 65
        });

        Ok(ToolResult {
            content: vec![Content::text(format!("Weather in {}: {}", city, weather_data))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("weather-server".to_string(), "1.0.0".to_string());

    server.add_tool(
        "get_weather".to_string(),
        Some("Get current weather for a city".to_string()),
        json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["city"]
        }),
        WeatherTool,
    ).await?;

    // Start HTTP server
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

### WebSocket Real-time Server

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};
use tokio::time::{interval, Duration};

struct RealTimeDataTool;

#[async_trait]
impl ToolHandler for RealTimeDataTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let data_type = arguments.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let current_time = chrono::Utc::now();
        let data = match data_type {
            "stock" => json!({
                "symbol": "AAPL",
                "price": 150.25,
                "change": "+2.5%",
                "timestamp": current_time
            }),
            "crypto" => json!({
                "symbol": "BTC",
                "price": 42000.50,
                "change": "-1.2%",
                "timestamp": current_time
            }),
            _ => json!({
                "message": "Unknown data type",
                "timestamp": current_time
            })
        };

        Ok(ToolResult {
            content: vec![Content::text(format!("Real-time data: {}", data))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("realtime-server".to_string(), "1.0.0".to_string());

    server.add_tool(
        "get_realtime_data".to_string(),
        Some("Get real-time market data".to_string()),
        json!({
            "type": "object",
            "properties": {
                "type": {
                    "type": "string",
                    "enum": ["stock", "crypto"],
                    "description": "Type of data to retrieve"
                }
            },
            "required": ["type"]
        }),
        RealTimeDataTool,
    ).await?;

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

## Advanced Examples

### Database Integration Server

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
    core::{tool::ToolHandler, resource::ResourceHandler},
    protocol::types::{ToolResult, Content, ResourceInfo, ResourceContent},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

// Mock database connection
struct Database {
    data: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("user:1".to_string(), json!({"id": 1, "name": "Alice", "email": "alice@example.com"}).to_string());
        data.insert("user:2".to_string(), json!({"id": 2, "name": "Bob", "email": "bob@example.com"}).to_string());
        
        Self { data }
    }

    async fn query(&self, sql: &str) -> Result<String, String> {
        // Simple mock SQL processing
        if sql.contains("SELECT * FROM users") {
            let users: Vec<&String> = self.data.iter()
                .filter(|(k, _)| k.starts_with("user:"))
                .map(|(_, v)| v)
                .collect();
            Ok(format!("[{}]", users.join(",")))
        } else {
            Err("Unsupported query".to_string())
        }
    }

    async fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
}

struct DatabaseTool {
    db: Database,
}

#[async_trait]
impl ToolHandler for DatabaseTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let query = arguments.get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing query parameter".to_string()))?;

        match self.db.query(query).await {
            Ok(result) => Ok(ToolResult {
                content: vec![Content::text(result)],
                is_error: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

struct DatabaseResource {
    db: Database,
}

#[async_trait]
impl ResourceHandler for DatabaseResource {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> Result<Vec<ResourceContent>, mcp_protocol_sdk::core::error::McpError> {
        if uri.starts_with("db://") {
            let key = &uri[5..];
            match self.db.get(key).await {
                Some(data) => Ok(vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("application/json".to_string()),
                    text: Some(data),
                    blob: None,
                }]),
                None => Err(mcp_protocol_sdk::core::error::McpError::ResourceNotFound(uri.to_string())),
            }
        } else {
            Err(mcp_protocol_sdk::core::error::McpError::ResourceNotFound(uri.to_string()))
        }
    }

    async fn list(&self) -> Result<Vec<ResourceInfo>, mcp_protocol_sdk::core::error::McpError> {
        Ok(vec![
            ResourceInfo {
                uri: "db://user:1".to_string(),
                name: "User 1".to_string(),
                description: Some("User record #1".to_string()),
                mime_type: Some("application/json".to_string()),
            },
            ResourceInfo {
                uri: "db://user:2".to_string(),
                name: "User 2".to_string(),
                description: Some("User record #2".to_string()),
                mime_type: Some("application/json".to_string()),
            },
        ])
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("database-server".to_string(), "1.0.0".to_string());

    let db = Database::new();

    // Add database query tool
    server.add_tool(
        "query_database".to_string(),
        Some("Execute SQL queries on the database".to_string()),
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "SQL query to execute"
                }
            },
            "required": ["query"]
        }),
        DatabaseTool { db: Database::new() },
    ).await?;

    // Add database resource access
    server.add_resource_detailed(
        ResourceInfo {
            uri: "db://".to_string(),
            name: "Database".to_string(),
            description: Some("Database records access".to_string()),
            mime_type: Some("application/json".to_string()),
        },
        DatabaseResource { db: Database::new() },
    ).await?;

    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    Ok(())
}
```

### Multi-Client WebSocket Server

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::{json, Value};

// Shared state for multi-client communication
#[derive(Default)]
struct ChatState {
    messages: Vec<String>,
    users: Vec<String>,
}

struct BroadcastTool {
    state: Arc<RwLock<ChatState>>,
}

#[async_trait]
impl ToolHandler for BroadcastTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let user = arguments.get("user")
            .and_then(|v| v.as_str())
            .unwrap_or("Anonymous");
        let message = arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let formatted_message = format!("{}: {}", user, message);
        
        // Add to shared state
        {
            let mut state = self.state.write().await;
            state.messages.push(formatted_message.clone());
            if !state.users.contains(&user.to_string()) {
                state.users.push(user.to_string());
            }
        }

        Ok(ToolResult {
            content: vec![Content::text(format!("Broadcasted: {}", formatted_message))],
            is_error: None,
        })
    }
}

struct GetMessagesTool {
    state: Arc<RwLock<ChatState>>,
}

#[async_trait]
impl ToolHandler for GetMessagesTool {
    async fn call(&self, _arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let state = self.state.read().await;
        let messages = state.messages.join("\n");
        
        Ok(ToolResult {
            content: vec![Content::text(messages)],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("chat-server".to_string(), "1.0.0".to_string());
    let state = Arc::new(RwLock::new(ChatState::default()));

    // Add broadcast tool
    server.add_tool(
        "broadcast".to_string(),
        Some("Broadcast a message to all clients".to_string()),
        json!({
            "type": "object",
            "properties": {
                "user": {"type": "string", "description": "Username"},
                "message": {"type": "string", "description": "Message to broadcast"}
            },
            "required": ["user", "message"]
        }),
        BroadcastTool { state: state.clone() },
    ).await?;

    // Add get messages tool
    server.add_tool(
        "get_messages".to_string(),
        Some("Get all chat messages".to_string()),
        json!({"type": "object", "properties": {}}),
        GetMessagesTool { state: state.clone() },
    ).await?;

    let transport = WebSocketServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("Multi-client chat server running on ws://localhost:8080");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

### Client with Session Management

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    client::session::SessionConfig,
    transport::websocket::WebSocketClientTransport,
};
use std::time::Duration;
use tokio::time::sleep;

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
        max_concurrent_requests: 10,
        request_timeout_ms: 30000,
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
            
            // Simulate long-running client with periodic requests
            loop {
                let client = session.client();
                let client_guard = client.lock().await;
                
                match client_guard.list_tools().await {
                    Ok(tools) => {
                        println!("Available tools: {:?}", tools.tools.len());
                    }
                    Err(e) => {
                        eprintln!("Error listing tools: {}", e);
                        // Session will auto-reconnect if needed
                    }
                }
                
                drop(client_guard);
                sleep(Duration::from_secs(5)).await;
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
    
    Ok(())
}
```

## Example Applications

### File Management Server

A server that provides file system operations.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
    core::{tool::ToolHandler, resource::ResourceHandler},
    protocol::types::{ToolResult, Content, ResourceInfo, ResourceContent},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};
use tokio::fs;

struct ReadFileTool;

#[async_trait]
impl ToolHandler for ReadFileTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let path = arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing path parameter".to_string()))?;

        match fs::read_to_string(path).await {
            Ok(content) => Ok(ToolResult {
                content: vec![Content::text(content)],
                is_error: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error reading file: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

struct WriteFileTool;

#[async_trait]
impl ToolHandler for WriteFileTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let path = arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing path parameter".to_string()))?;
        let content = arguments.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing content parameter".to_string()))?;

        match fs::write(path, content).await {
            Ok(_) => Ok(ToolResult {
                content: vec![Content::text(format!("File written successfully: {}", path))],
                is_error: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error writing file: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("file-server".to_string(), "1.0.0".to_string());

    // Add file operations
    server.add_tool(
        "read_file".to_string(),
        Some("Read contents of a file".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path to read"}
            },
            "required": ["path"]
        }),
        ReadFileTool,
    ).await?;

    server.add_tool(
        "write_file".to_string(),
        Some("Write content to a file".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path to write"},
                "content": {"type": "string", "description": "Content to write"}
            },
            "required": ["path", "content"]
        }),
        WriteFileTool,
    ).await?;

    let transport = HttpServerTransport::new("0.0.0.0:3000");
    server.start(transport).await?;
    
    println!("File management server running on http://localhost:3000");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

## Running the Examples

All examples are available in the repository:

```bash
# Clone and run
git clone https://github.com/YOUR_USERNAME/mcp-rust-sdk.git
cd mcp-rust-sdk

# Basic examples
cargo run --example simple_server
cargo run --example client_example

# HTTP examples
cargo run --example http_server --features http
cargo run --example http_client --features http

# WebSocket examples
cargo run --example websocket_server --features websocket
cargo run --example websocket_client --features websocket

# Advanced examples
cargo run --example database_server
```

## Next Steps

- [Transport Guide](transports.md) - Deep dive into transport options
- [Architecture](architecture.md) - Understanding the system design
- [API Reference](https://docs.rs/mcp-rust-sdk) - Complete API documentation
