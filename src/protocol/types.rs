//! MCP protocol type definitions
//!
//! This module contains all the core types defined by the Model Context Protocol
//! specification, including messages, capabilities, and content types.

use serde::{Deserialize, Serialize};

/// Information about an MCP server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerInfo {
    /// Name of the server
    pub name: String,
    /// Version of the server
    pub version: String,
}

/// Information about an MCP client
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientInfo {
    /// Name of the client
    pub name: String,
    /// Version of the client
    pub version: String,
}

/// Capabilities advertised by an MCP server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCapabilities {
    /// Prompt-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    /// Resource-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    /// Tool-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    /// Sampling-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
}

/// Capabilities advertised by an MCP client
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClientCapabilities {
    /// Sampling-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
}

/// Prompt-related server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptsCapability {
    /// Whether the server supports prompt list change notifications
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Resource-related server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourcesCapability {
    /// Whether the server supports resource subscriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    /// Whether the server supports resource list change notifications
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Tool-related server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolsCapability {
    /// Whether the server supports tool list change notifications
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Sampling-related capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SamplingCapability {}

/// Content that can be returned by tools, resources, or prompts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Content {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
    },
    /// Image content
    #[serde(rename = "image")]
    Image {
        /// Base64-encoded image data
        data: String,
        /// MIME type of the image
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
}

/// Content of a resource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceContent {
    /// URI of the resource
    pub uri: String,
    /// MIME type of the content
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Text content (for text resources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Binary content encoded as base64 (for binary resources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
}

/// Result of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolResult {
    /// Content returned by the tool
    pub content: Vec<Content>,
    /// Whether this result represents an error
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

/// Information about a tool
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolInfo {
    /// Name of the tool
    pub name: String,
    /// Description of what the tool does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema describing the tool's input parameters
    #[serde(rename = "inputSchema")]
    pub input_schema: serde_json::Value,
}

/// Information about a resource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceInfo {
    /// URI of the resource
    pub uri: String,
    /// Human-readable name of the resource
    pub name: String,
    /// Description of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type of the resource
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// Information about a prompt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptInfo {
    /// Name of the prompt
    pub name: String,
    /// Description of what the prompt does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Arguments that the prompt accepts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

/// Argument for a prompt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptArgument {
    /// Name of the argument
    pub name: String,
    /// Description of the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this argument is required
    pub required: bool,
}

/// Message in a prompt result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptMessage {
    /// Role of the message (e.g., "user", "assistant", "system")
    pub role: String,
    /// Content of the message
    pub content: PromptContent,
}

/// Content of a prompt message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PromptContent {
    /// Text content
    Text {
        /// Type identifier
        #[serde(rename = "type")]
        content_type: String,
        /// The text content
        text: String,
    },
    /// Image content
    Image {
        /// Type identifier
        #[serde(rename = "type")]
        content_type: String,
        /// Base64-encoded image data
        data: String,
        /// MIME type of the image
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
}

/// Result of prompt execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptResult {
    /// Description of the prompt result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Messages generated by the prompt
    pub messages: Vec<PromptMessage>,
}

// JSON-RPC 2.0 message types

/// JSON-RPC 2.0 request message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID for correlation
    pub id: serde_json::Value,
    /// Method name being called
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 response message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID for correlation
    pub id: serde_json::Value,
    /// Result of the method call (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error information (if unsuccessful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 error information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 notification message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcNotification {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name being called
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

// Standard JSON-RPC error codes
/// Invalid JSON was received
pub const PARSE_ERROR: i32 = -32700;
/// The JSON sent is not a valid Request object
pub const INVALID_REQUEST: i32 = -32600;
/// The method does not exist / is not available
pub const METHOD_NOT_FOUND: i32 = -32601;
/// Invalid method parameter(s)
pub const INVALID_PARAMS: i32 = -32602;
/// Internal JSON-RPC error
pub const INTERNAL_ERROR: i32 = -32603;

// MCP-specific error codes
/// Tool not found
pub const TOOL_NOT_FOUND: i32 = -32000;
/// Resource not found
pub const RESOURCE_NOT_FOUND: i32 = -32001;
/// Prompt not found
pub const PROMPT_NOT_FOUND: i32 = -32002;

impl JsonRpcRequest {
    /// Create a new JSON-RPC request
    pub fn new<T: Serialize>(
        id: serde_json::Value,
        method: String,
        params: Option<T>,
    ) -> Result<Self, serde_json::Error> {
        let params = match params {
            Some(p) => Some(serde_json::to_value(p)?),
            None => None,
        };

        Ok(Self {
            jsonrpc: "2.0".to_string(),
            id,
            method,
            params,
        })
    }
}

impl JsonRpcResponse {
    /// Create a successful JSON-RPC response
    pub fn success<T: Serialize>(
        id: serde_json::Value,
        result: T,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::to_value(result)?),
            error: None,
        })
    }

    /// Create an error JSON-RPC response
    pub fn error(
        id: serde_json::Value,
        code: i32,
        message: String,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data,
            }),
        }
    }
}

impl JsonRpcNotification {
    /// Create a new JSON-RPC notification
    pub fn new<T: Serialize>(method: String, params: Option<T>) -> Result<Self, serde_json::Error> {
        let params = match params {
            Some(p) => Some(serde_json::to_value(p)?),
            None => None,
        };

        Ok(Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
        })
    }
}

// Helper functions for creating common content types

impl Content {
    /// Create text content
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::Text { text: text.into() }
    }

    /// Create image content
    pub fn image<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self::Image {
            data: data.into(),
            mime_type: mime_type.into(),
        }
    }
}

impl PromptContent {
    /// Create text prompt content
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::Text {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }

    /// Create image prompt content
    pub fn image<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self::Image {
            content_type: "image".to_string(),
            data: data.into(),
            mime_type: mime_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_content_serialization() {
        let text_content = Content::text("Hello, world!");
        let json = serde_json::to_value(&text_content).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "text",
                "text": "Hello, world!"
            })
        );

        let image_content = Content::image("data", "image/png");
        let json = serde_json::to_value(&image_content).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "image",
                "data": "data",
                "mimeType": "image/png"
            })
        );
    }

    #[test]
    fn test_jsonrpc_request() {
        let request = JsonRpcRequest::new(
            json!(1),
            "test_method".to_string(),
            Some(json!({"param": "value"})),
        )
        .unwrap();

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "test_method");
        assert_eq!(request.id, json!(1));
    }

    #[test]
    fn test_jsonrpc_response() {
        let response = JsonRpcResponse::success(json!(1), json!({"result": "success"})).unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert!(response.result.is_some());
        assert!(response.error.is_none());

        let error_response = JsonRpcResponse::error(
            json!(1),
            INVALID_PARAMS,
            "Invalid parameters".to_string(),
            None,
        );
        assert!(error_response.result.is_none());
        assert!(error_response.error.is_some());
    }
}
