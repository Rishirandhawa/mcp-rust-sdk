//! MCP server implementation
//!
//! This module provides the main server implementation for the Model Context Protocol.

pub mod handlers;
pub mod lifecycle;
pub mod mcp_server;

// Re-export the main server type
pub use mcp_server::McpServer;
