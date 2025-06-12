//! MCP protocol implementation
//!
//! This module contains the core protocol types and message handling for the
//! Model Context Protocol, including JSON-RPC message serialization and validation.

pub mod messages;
pub mod types;
pub mod validation;

// Re-export commonly used types
pub use messages::{methods, MCP_PROTOCOL_VERSION};
pub use types::*;
pub use validation::*;
