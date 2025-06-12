# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup and core architecture
- Complete MCP protocol implementation with JSON-RPC 2.0 support
- Multi-transport architecture (STDIO, HTTP, WebSocket)
- Comprehensive server implementation with tool, resource, and prompt management
- Full-featured client implementation with session management
- Auto-reconnection and connection lifecycle management
- Real-time bidirectional communication support
- JSON Schema validation for tool parameters
- URI-based resource management with MIME type detection
- Dynamic prompt template system
- Comprehensive error handling and recovery mechanisms
- 8 complete examples covering all transport types and use cases
- Extensive test suite with 85+ unit tests and integration tests
- Performance benchmarks for optimization
- Complete documentation with usage guides and API reference

### Features

#### Core Protocol
- JSON-RPC 2.0 compliant request/response handling
- Complete MCP message type definitions
- Protocol validation and error handling
- Extensible message framework

#### Transport Layer
- **STDIO Transport**: Process-based communication (default)
- **HTTP Transport**: RESTful API with Server-Sent Events for real-time notifications
- **WebSocket Transport**: Full-duplex real-time bidirectional communication
- Pluggable transport architecture for easy extension
- Configurable timeouts, compression, and connection parameters

#### Server Implementation
- Multi-client support with async architecture
- Dynamic tool registration and execution
- Resource discovery and content serving
- Prompt template registration and retrieval
- Real-time notification broadcasting
- Graceful shutdown and cleanup

#### Client Implementation
- Session management with auto-reconnection
- Exponential backoff retry logic
- Connection pooling and lifecycle management
- Request timeout and cancellation
- Statistics and monitoring hooks

#### Developer Experience
- Type-safe Rust implementation with compile-time guarantees
- Comprehensive error types covering all failure modes
- Detailed documentation with examples
- Multiple feature flags for selective compilation
- Memory-efficient async design
- Extensive logging and debugging support

### Examples
- `simple_server` - Basic MCP server with essential tools
- `echo_server` - Demonstration server with tool examples
- `client_example` - Basic client usage and connection
- `database_server` - Database integration patterns
- `http_server` - HTTP transport with REST API and SSE
- `http_client` - HTTP client with real-time updates
- `websocket_server` - Real-time WebSocket server
- `websocket_client` - WebSocket client with bidirectional messaging

### Dependencies
- `tokio` - Async runtime and utilities
- `serde` - Serialization framework
- `serde_json` - JSON serialization
- `async-trait` - Async trait support
- `uuid` - Unique identifier generation
- `url` - URL parsing and manipulation
- `thiserror` - Error handling
- `tracing` - Structured logging
- `axum` - HTTP server framework (optional)
- `reqwest` - HTTP client (optional)
- `tokio-tungstenite` - WebSocket support (optional)

## [0.1.0] - 2024-01-XX

### Added
- Initial release of MCP Rust SDK
- Complete implementation of Model Context Protocol specification
- Multi-transport support (STDIO, HTTP, WebSocket)
- Comprehensive documentation and examples
- Full test suite with CI/CD integration

## Release Process

### Version Numbering
This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking changes that require user code updates
- **MINOR**: New features that are backwards compatible
- **PATCH**: Bug fixes and improvements that are backwards compatible

### Release Categories

#### Major Releases (X.0.0)
- Breaking API changes
- Major architectural changes
- Removal of deprecated features
- Minimum supported Rust version changes

#### Minor Releases (0.X.0)
- New features and capabilities
- New transport types
- New tool/resource/prompt features
- Performance improvements
- Enhanced developer experience

#### Patch Releases (0.0.X)
- Bug fixes
- Security updates
- Documentation improvements
- Minor performance optimizations

### Change Categories

#### Added
- New features, APIs, or capabilities
- New examples or documentation
- New transport types or protocols

#### Changed
- Modifications to existing functionality
- API improvements or enhancements
- Performance optimizations
- Updated dependencies

#### Deprecated
- Features that will be removed in future versions
- Prefer alternative approaches
- Migration guides provided

#### Removed
- Deleted features or APIs
- Breaking changes with major version bumps
- Cleanup of deprecated functionality

#### Fixed
- Bug fixes and error corrections
- Security vulnerability fixes
- Memory leaks or performance issues

#### Security
- Security-related improvements
- Vulnerability fixes
- Privacy enhancements

### Migration Guides

When breaking changes are introduced, detailed migration guides will be provided:

1. **What Changed**: Clear description of the changes
2. **Why**: Rationale for the breaking change
3. **Migration Steps**: Step-by-step guide to update code
4. **Examples**: Before/after code examples
5. **Timeline**: Deprecation and removal timeline

### Compatibility

#### Rust Version Compatibility
- **Minimum Supported Rust Version (MSRV)**: 1.70.0
- MSRV changes are considered breaking changes
- MSRV policy: Support last 6 months of stable releases

#### Platform Compatibility
- **Primary**: Linux, macOS, Windows
- **Architecture**: x86_64, aarch64
- **Testing**: All platforms tested in CI

#### Protocol Compatibility
- **MCP Protocol**: Follows official specification
- **JSON-RPC**: 2.0 compliant
- **Backwards Compatibility**: Maintained within major versions

---

**Note**: This changelog follows the [Keep a Changelog](https://keepachangelog.com/) format. For detailed commit history, see the [GitHub repository](https://github.com/Rishirandhawa/mcp-rust-sdk).
