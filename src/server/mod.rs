//! MCP server implementation
//! 
//! This module provides the main server implementation for the Model Context Protocol.

pub mod mcp_server;
pub mod handlers;
pub mod lifecycle;

// Re-export the main server type
pub use mcp_server::McpServer;
