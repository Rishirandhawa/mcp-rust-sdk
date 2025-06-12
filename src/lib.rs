//! # MCP Rust SDK
//!
//! A comprehensive Rust SDK for the [Model Context Protocol (MCP)](https://modelcontextprotocol.io/),
//! providing both server and client implementations with full MCP specification compliance.
//!
//! ## Features
//!
//! - 🚀 **High Performance**: Built with Rust's zero-cost abstractions and async/await
//! - 🛡️ **Type Safety**: Leverages Rust's type system to prevent runtime errors
//! - 🔌 **Multiple Transports**: Support for STDIO, HTTP/SSE, and WebSocket transports
//! - 🎯 **Full MCP Compliance**: Complete implementation of the MCP specification
//! - 📚 **Rich Ecosystem**: Tools, resources, prompts, and sampling support
//!
//! ## Quick Start
//!
//! ### Server Example
//!
//! ```rust,no_run
//! use mcp_protocol_sdk::{
//!     server::McpServer,
//!     core::{tool::ToolHandler, error::McpResult},
//!     protocol::types::{Content, ToolResult},
//! };
//! use async_trait::async_trait;
//! use std::collections::HashMap;
//! use serde_json::Value;
//!
//! struct EchoHandler;
//!
//! #[async_trait]
//! impl ToolHandler for EchoHandler {
//!     async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
//!         let message = arguments.get("message")
//!             .and_then(|v| v.as_str())
//!             .unwrap_or("Hello, World!");
//!         
//!         Ok(ToolResult {
//!             content: vec![Content::Text { text: message.to_string() }],
//!             is_error: None,
//!         })
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> McpResult<()> {
//!     let mut server = McpServer::new("echo-server".to_string(), "1.0.0".to_string());
//!     
//!     server.add_tool(
//!         "echo".to_string(),
//!         Some("Echo a message".to_string()),
//!         serde_json::json!({
//!             "type": "object",
//!             "properties": {
//!                 "message": { "type": "string" }
//!             }
//!         }),
//!         EchoHandler,
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Module Organization
//!
//! - [`core`]: Core abstractions for resources, tools, prompts, and errors
//! - [`protocol`]: MCP protocol types and message definitions  
//! - [`transport`]: Transport layer implementations (STDIO, HTTP, WebSocket)
//! - [`server`]: MCP server implementation and lifecycle management
//! - [`client`]: MCP client implementation and session management
//! - [`utils`]: Utility functions and helpers

pub mod client;
pub mod core;
pub mod protocol;
pub mod server;
pub mod transport;
pub mod utils;

// Re-export commonly used types for convenience
pub use core::error::{McpError, McpResult};
pub use protocol::types::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::client::McpClient;
    pub use crate::core::{
        error::{McpError, McpResult},
        prompt::{Prompt, PromptHandler},
        resource::{Resource, ResourceHandler},
        tool::{Tool, ToolHandler},
        PromptInfo, ResourceInfo, ToolInfo,
    };
    pub use crate::protocol::types::*;
    pub use crate::server::McpServer;
    pub use async_trait::async_trait;
    pub use serde_json::{json, Value};
    pub use std::collections::HashMap;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // Basic smoke test to ensure all modules are accessible
        let _error = McpError::Protocol("test".to_string());
    }
}
