//! MCP protocol message definitions
//!
//! This module contains all the MCP-specific message types and their serialization/deserialization
//! logic. These messages follow the JSON-RPC 2.0 specification and represent the various operations
//! supported by the Model Context Protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::protocol::types::*;

// ============================================================================
// Initialization Messages
// ============================================================================

/// Parameters for the initialize request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeParams {
    /// Information about the client
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
    /// Capabilities advertised by the client
    pub capabilities: ClientCapabilities,
    /// Protocol version being used
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

/// Result of the initialize request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeResult {
    /// Information about the server
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
    /// Capabilities advertised by the server
    pub capabilities: ServerCapabilities,
    /// Protocol version being used
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

// ============================================================================
// Tool Messages
// ============================================================================

/// Parameters for the tools/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListToolsParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of the tools/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListToolsResult {
    /// Available tools
    pub tools: Vec<ToolInfo>,
    /// Cursor for pagination (if more tools are available)
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Parameters for the tools/call request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallToolParams {
    /// Name of the tool to call
    pub name: String,
    /// Arguments to pass to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, Value>>,
}

/// Result of the tools/call request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallToolResult {
    /// Content returned by the tool
    pub content: Vec<Content>,
    /// Whether this result represents an error
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

// ============================================================================
// Resource Messages
// ============================================================================

/// Parameters for the resources/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResourcesParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of the resources/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResourcesResult {
    /// Available resources
    pub resources: Vec<ResourceInfo>,
    /// Cursor for pagination (if more resources are available)
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Parameters for the resources/read request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadResourceParams {
    /// URI of the resource to read
    pub uri: String,
}

/// Result of the resources/read request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadResourceResult {
    /// Contents of the resource
    pub contents: Vec<ResourceContent>,
}

/// Parameters for the resources/subscribe request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeResourceParams {
    /// URI of the resource to subscribe to
    pub uri: String,
}

/// Result of the resources/subscribe request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeResourceResult {}

/// Parameters for the resources/unsubscribe request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeResourceParams {
    /// URI of the resource to unsubscribe from
    pub uri: String,
}

/// Result of the resources/unsubscribe request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeResourceResult {}

/// Parameters for the resources/updated notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUpdatedParams {
    /// URI of the resource that was updated
    pub uri: String,
}

/// Parameters for the resources/list_changed notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceListChangedParams {}

// ============================================================================
// Prompt Messages
// ============================================================================

/// Parameters for the prompts/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListPromptsParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of the prompts/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPromptsResult {
    /// Available prompts
    pub prompts: Vec<PromptInfo>,
    /// Cursor for pagination (if more prompts are available)
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Parameters for the prompts/get request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPromptParams {
    /// Name of the prompt to get
    pub name: String,
    /// Arguments to pass to the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, Value>>,
}

/// Result of the prompts/get request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPromptResult {
    /// Description of the prompt result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Messages generated by the prompt
    pub messages: Vec<PromptMessage>,
}

/// Parameters for the prompts/list_changed notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptListChangedParams {}

// ============================================================================
// Sampling Messages
// ============================================================================

/// Parameters for the sampling/createMessage request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMessageParams {
    /// Messages to include in the conversation
    pub messages: Vec<SamplingMessage>,
    /// Model preferences
    #[serde(rename = "modelPreferences", skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<ModelPreferences>,
    /// System prompt
    #[serde(rename = "systemPrompt", skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Whether to include context from the current session
    #[serde(rename = "includeContext", skip_serializing_if = "Option::is_none")]
    pub include_context: Option<String>,
    /// Maximum number of tokens to generate
    #[serde(rename = "maxTokens", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Sampling temperature (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter (0.0 to 1.0)
    #[serde(rename = "topP", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Stop sequences
    #[serde(rename = "stopSequences", skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// Metadata to include with the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

/// Result of the sampling/createMessage request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMessageResult {
    /// Role of the generated message
    pub role: String,
    /// Content of the generated message
    pub content: SamplingContent,
    /// Model used for generation
    pub model: String,
    /// Stop reason
    #[serde(rename = "stopReason", skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
}

/// A message in a sampling conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SamplingMessage {
    /// Role of the message (e.g., "user", "assistant")
    pub role: String,
    /// Content of the message
    pub content: SamplingContent,
}

/// Content for sampling messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SamplingContent {
    /// Simple text content
    Text(String),
    /// Complex content with multiple parts
    Complex(Vec<Content>),
}

/// Model preferences for sampling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelPreferences {
    /// Hints about cost constraints
    #[serde(rename = "costPriority", skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f32>,
    /// Hints about speed constraints
    #[serde(rename = "speedPriority", skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f32>,
    /// Hints about quality constraints
    #[serde(rename = "qualityPriority", skip_serializing_if = "Option::is_none")]
    pub quality_priority: Option<f32>,
}

// ============================================================================
// Tool List Changed Notification
// ============================================================================

/// Parameters for the tools/list_changed notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolListChangedParams {}

// ============================================================================
// Ping Messages
// ============================================================================

/// Parameters for the ping request (no parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingParams {}

/// Result of the ping request (no result)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingResult {}

// ============================================================================
// Logging Messages
// ============================================================================

/// Parameters for the logging/setLevel request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetLoggingLevelParams {
    /// The logging level to set
    pub level: LoggingLevel,
}

/// Result of the logging/setLevel request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetLoggingLevelResult {}

/// Logging level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

/// Parameters for the logging/message notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingMessageParams {
    /// The logging level
    pub level: LoggingLevel,
    /// The logger name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    /// The log message data
    pub data: Value,
}

// ============================================================================
// Progress Messages
// ============================================================================

/// Parameters for the progress notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressParams {
    /// Unique identifier for the progress operation
    #[serde(rename = "progressToken")]
    pub progress_token: String,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Optional total count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
}

// ============================================================================
// Message Helpers and Constructors
// ============================================================================

impl InitializeParams {
    /// Create new initialize parameters
    pub fn new(
        client_info: ClientInfo,
        capabilities: ClientCapabilities,
        protocol_version: String,
    ) -> Self {
        Self {
            client_info,
            capabilities,
            protocol_version,
        }
    }
}

impl InitializeResult {
    /// Create new initialize result
    pub fn new(
        server_info: ServerInfo,
        capabilities: ServerCapabilities,
        protocol_version: String,
    ) -> Self {
        Self {
            server_info,
            capabilities,
            protocol_version,
        }
    }
}

impl CallToolParams {
    /// Create new call tool parameters
    pub fn new(name: String, arguments: Option<HashMap<String, Value>>) -> Self {
        Self { name, arguments }
    }
}

impl ReadResourceParams {
    /// Create new read resource parameters
    pub fn new(uri: String) -> Self {
        Self { uri }
    }
}

impl GetPromptParams {
    /// Create new get prompt parameters
    pub fn new(name: String, arguments: Option<HashMap<String, Value>>) -> Self {
        Self { name, arguments }
    }
}

impl SamplingMessage {
    /// Create a user message
    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: "user".to_string(),
            content: SamplingContent::Text(content.into()),
        }
    }

    /// Create an assistant message
    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: "assistant".to_string(),
            content: SamplingContent::Text(content.into()),
        }
    }

    /// Create a system message
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: "system".to_string(),
            content: SamplingContent::Text(content.into()),
        }
    }

    /// Create a message with complex content
    pub fn with_content<S: Into<String>>(role: S, content: Vec<Content>) -> Self {
        Self {
            role: role.into(),
            content: SamplingContent::Complex(content),
        }
    }
}

impl Default for ModelPreferences {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelPreferences {
    /// Create model preferences with default values
    pub fn new() -> Self {
        Self {
            cost_priority: None,
            speed_priority: None,
            quality_priority: None,
        }
    }

    /// Set cost priority
    pub fn with_cost_priority(mut self, priority: f32) -> Self {
        self.cost_priority = Some(priority);
        self
    }

    /// Set speed priority
    pub fn with_speed_priority(mut self, priority: f32) -> Self {
        self.speed_priority = Some(priority);
        self
    }

    /// Set quality priority
    pub fn with_quality_priority(mut self, priority: f32) -> Self {
        self.quality_priority = Some(priority);
        self
    }
}

// ============================================================================
// Message Type Constants
// ============================================================================

/// Constant for MCP protocol version
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// JSON-RPC method names for MCP messages
pub mod methods {
    /// Initialize the connection
    pub const INITIALIZE: &str = "initialize";

    /// Ping to check connection
    pub const PING: &str = "ping";

    /// List available tools
    pub const TOOLS_LIST: &str = "tools/list";
    /// Call a tool
    pub const TOOLS_CALL: &str = "tools/call";
    /// Notification when tool list changes
    pub const TOOLS_LIST_CHANGED: &str = "tools/list_changed";

    /// List available resources
    pub const RESOURCES_LIST: &str = "resources/list";
    /// Read a resource
    pub const RESOURCES_READ: &str = "resources/read";
    /// Subscribe to resource updates
    pub const RESOURCES_SUBSCRIBE: &str = "resources/subscribe";
    /// Unsubscribe from resource updates
    pub const RESOURCES_UNSUBSCRIBE: &str = "resources/unsubscribe";
    /// Notification when a resource is updated
    pub const RESOURCES_UPDATED: &str = "resources/updated";
    /// Notification when resource list changes
    pub const RESOURCES_LIST_CHANGED: &str = "resources/list_changed";

    /// List available prompts
    pub const PROMPTS_LIST: &str = "prompts/list";
    /// Get a prompt
    pub const PROMPTS_GET: &str = "prompts/get";
    /// Notification when prompt list changes
    pub const PROMPTS_LIST_CHANGED: &str = "prompts/list_changed";

    /// Create a message using sampling
    pub const SAMPLING_CREATE_MESSAGE: &str = "sampling/createMessage";

    /// Set logging level
    pub const LOGGING_SET_LEVEL: &str = "logging/setLevel";
    /// Log message notification
    pub const LOGGING_MESSAGE: &str = "logging/message";

    /// Progress notification
    pub const PROGRESS: &str = "progress";
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_initialize_params_serialization() {
        let params = InitializeParams::new(
            ClientInfo {
                name: "test-client".to_string(),
                version: "1.0.0".to_string(),
            },
            ClientCapabilities::default(),
            MCP_PROTOCOL_VERSION.to_string(),
        );

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["clientInfo"]["name"], "test-client");
        assert_eq!(json["protocolVersion"], MCP_PROTOCOL_VERSION);
    }

    #[test]
    fn test_call_tool_params() {
        let mut args = HashMap::new();
        args.insert("param1".to_string(), json!("value1"));
        args.insert("param2".to_string(), json!(42));

        let params = CallToolParams::new("test_tool".to_string(), Some(args));
        let json = serde_json::to_value(&params).unwrap();

        assert_eq!(json["name"], "test_tool");
        assert_eq!(json["arguments"]["param1"], "value1");
        assert_eq!(json["arguments"]["param2"], 42);
    }

    #[test]
    fn test_sampling_message_creation() {
        let user_msg = SamplingMessage::user("Hello, world!");
        assert_eq!(user_msg.role, "user");

        if let SamplingContent::Text(text) = user_msg.content {
            assert_eq!(text, "Hello, world!");
        } else {
            panic!("Expected text content");
        }

        let assistant_msg = SamplingMessage::assistant("Hello back!");
        assert_eq!(assistant_msg.role, "assistant");
    }

    #[test]
    fn test_model_preferences_builder() {
        let prefs = ModelPreferences::default()
            .with_cost_priority(0.8)
            .with_speed_priority(0.6)
            .with_quality_priority(0.9);

        assert_eq!(prefs.cost_priority, Some(0.8));
        assert_eq!(prefs.speed_priority, Some(0.6));
        assert_eq!(prefs.quality_priority, Some(0.9));
    }

    #[test]
    fn test_read_resource_params() {
        let params = ReadResourceParams::new("file:///path/to/file.txt".to_string());
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["uri"], "file:///path/to/file.txt");
    }

    #[test]
    fn test_logging_level_serialization() {
        let level = LoggingLevel::Warning;
        let json = serde_json::to_value(&level).unwrap();
        assert_eq!(json, "warning");

        let deserialized: LoggingLevel = serde_json::from_value(json!("error")).unwrap();
        assert_eq!(deserialized, LoggingLevel::Error);
    }

    #[test]
    fn test_method_constants() {
        assert_eq!(methods::INITIALIZE, "initialize");
        assert_eq!(methods::TOOLS_LIST, "tools/list");
        assert_eq!(methods::TOOLS_CALL, "tools/call");
        assert_eq!(methods::RESOURCES_READ, "resources/read");
        assert_eq!(methods::PROMPTS_GET, "prompts/get");
        assert_eq!(methods::SAMPLING_CREATE_MESSAGE, "sampling/createMessage");
    }
}
