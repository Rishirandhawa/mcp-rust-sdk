# Contributing to MCP Rust SDK

First off, thank you for considering contributing to the MCP Rust SDK! 🎉 

It's people like you that make this project a valuable tool for the Rust and MCP communities.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Contribution Types](#contribution-types)
- [Development Setup](#development-setup)
- [Code Standards](#code-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Guidelines](#documentation-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)
- [Release Process](#release-process)
- [Community](#community)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [rishi.randhawa@example.com](mailto:rishi.randhawa@example.com).

## Getting Started

### Ways to Contribute

- 🐛 **Bug Reports**: Report bugs through GitHub issues
- ✨ **Feature Requests**: Suggest new features or improvements
- 📖 **Documentation**: Improve docs, tutorials, and examples
- 🧪 **Testing**: Add tests and improve test coverage
- 💻 **Code**: Implement features, fix bugs, improve performance
- 🎨 **Examples**: Create new examples and tutorials
- 🔍 **Code Review**: Review pull requests from other contributors

### Good First Issues

Look for issues labeled with:
- `good first issue` - Easy issues perfect for newcomers
- `help wanted` - Issues where we'd especially like community help
- `documentation` - Documentation improvements needed

## Development Process

We use a Git flow inspired development process:

1. **Main Branch**: Always stable, deployable code
2. **Feature Branches**: New features developed in separate branches
3. **Pull Requests**: All changes must go through PR review
4. **Continuous Integration**: All PRs must pass CI checks

### Branch Naming Convention

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `test/description` - Test improvements

## Contribution Types

### 🐛 Bug Fixes

1. Search existing issues to avoid duplicates
2. Create a new issue with the bug report template
3. Fork the repository and create a fix branch
4. Write tests that reproduce the bug
5. Implement the fix
6. Ensure all tests pass
7. Submit a pull request

### ✨ New Features

1. Discuss the feature in an issue first
2. Get consensus from maintainers
3. Create a feature branch
4. Implement the feature with tests
5. Update documentation
6. Submit a pull request

### 📖 Documentation

We welcome improvements to:
- README and getting started guides
- API documentation and examples
- Code comments and inline docs
- Tutorial content
- Architecture documentation

## Development Setup

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Git**: For version control
- **GitHub CLI** (optional): For easier GitHub integration

### Local Setup

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/mcp-rust-sdk.git
   cd mcp-rust-sdk
   ```

2. **Install Dependencies**:
   ```bash
   # Rust should install dependencies automatically
   cargo check
   ```

3. **Set Up Git Hooks** (optional):
   ```bash
   # Install pre-commit hooks for formatting
   echo "cargo fmt" > .git/hooks/pre-commit
   chmod +x .git/hooks/pre-commit
   ```

4. **Test Your Setup**:
   ```bash
   # Run all tests
   cargo test --all-features
   
   # Check formatting
   cargo fmt --all -- --check
   
   # Run linter
   cargo clippy --all-features -- -D warnings
   ```

### Development Commands

```bash
# Basic development cycle
cargo check                    # Quick compile check
cargo test                     # Run tests
cargo test --all-features      # Test with all features
cargo fmt                      # Format code
cargo clippy                   # Lint code

# Feature-specific testing
cargo test --features http     # Test HTTP transport
cargo test --features websocket # Test WebSocket transport

# Example testing
cargo check --example simple_server
cargo run --example simple_server

# Documentation
cargo doc --open              # Build and open docs
cargo doc --all-features --no-deps
```

## Code Standards

### Rust Style Guide

We follow the official Rust style guide with these additions:

1. **Formatting**: Use `cargo fmt` (rustfmt) for consistent formatting
2. **Linting**: Code must pass `cargo clippy` without warnings
3. **Documentation**: All public APIs must have documentation
4. **Testing**: New code requires appropriate tests

### Code Quality Standards

- **Error Handling**: Use proper error types, avoid `unwrap()` in library code
- **Async Code**: Use `async`/`await` appropriately, avoid blocking calls
- **Memory Safety**: Leverage Rust's ownership system, minimize `unsafe` code
- **Performance**: Consider performance implications of design decisions

### API Design Principles

1. **Ergonomic**: APIs should be easy to use correctly
2. **Consistent**: Similar operations should have similar APIs
3. **Composable**: Components should work well together
4. **Extensible**: Design for future extension
5. **Type Safe**: Leverage Rust's type system for safety

### Code Organization

```
src/
├── core/           # Core abstractions and types
├── protocol/       # MCP protocol implementation
├── transport/      # Transport layer implementations
├── server/         # Server implementation
├── client/         # Client implementation
└── utils/          # Utility functions
```

## Testing Guidelines

### Test Requirements

- **Unit Tests**: All public functions should have unit tests
- **Integration Tests**: Complex features need integration tests
- **Example Tests**: Examples should compile and run correctly
- **Documentation Tests**: Code in documentation should be tested

### Test Categories

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test component interactions
3. **Feature Tests**: Test with different feature combinations
4. **Platform Tests**: Test on different operating systems
5. **Performance Tests**: Benchmark critical paths

### Writing Good Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name_should_do_something() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
    
    #[tokio::test]
    async fn test_async_function() {
        // Test async functions
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Test Coverage

We aim for:
- **80%+ overall coverage**
- **95%+ for core functionality**
- **100% for critical error paths**

## Documentation Guidelines

### Documentation Requirements

1. **Public APIs**: All public items must have documentation
2. **Examples**: Include usage examples in documentation
3. **Error Cases**: Document when functions can fail
4. **Safety**: Document any safety requirements

### Documentation Style

```rust
/// Brief one-line description of what this does.
///
/// Longer description with more details about the behavior,
/// edge cases, and any important information.
///
/// # Arguments
///
/// * `param1` - Description of the first parameter
/// * `param2` - Description of the second parameter
///
/// # Returns
///
/// Description of what this function returns.
///
/// # Errors
///
/// Description of when this function returns an error.
///
/// # Examples
///
/// ```
/// use mcp_protocol_sdk::*;
/// 
/// let result = function_name("example").unwrap();
/// assert_eq!(result, expected_value);
/// ```
pub fn function_name(param1: &str, param2: i32) -> Result<String, Error> {
    // Implementation
}
```

### README Updates

When adding new features, update:
- Feature list in README
- Usage examples
- Installation instructions (if needed)
- Compatibility information

## Pull Request Process

### Before Submitting

1. **Sync with main**: Rebase your branch on latest main
2. **Run tests**: Ensure all tests pass locally
3. **Check formatting**: Run `cargo fmt` and `cargo clippy`
4. **Update docs**: Update relevant documentation
5. **Add changelog entry**: Add entry to CHANGELOG.md if applicable

### PR Requirements

- [ ] **Clear title**: Descriptive title following conventional commits
- [ ] **Description**: Explain what the PR does and why
- [ ] **Tests**: Include appropriate tests
- [ ] **Documentation**: Update documentation as needed
- [ ] **No breaking changes**: Or clearly document them
- [ ] **CI passes**: All CI checks must pass

### PR Review Process

1. **Automated checks**: CI must pass before review
2. **Code review**: At least one maintainer review required
3. **Testing**: Reviewers may test changes locally
4. **Approval**: PR needs maintainer approval to merge
5. **Merge**: Maintainer will merge using squash commits

### Conventional Commits

We use conventional commit format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only changes
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests
- `chore`: Changes to build process or auxiliary tools

**Examples:**
```
feat(transport): add WebSocket transport support
fix(server): handle connection errors gracefully
docs(readme): update installation instructions
```

## Issue Guidelines

### Before Creating an Issue

1. **Search existing issues**: Avoid duplicates
2. **Check documentation**: Ensure it's not already documented
3. **Minimal reproduction**: Create a minimal example

### Issue Types

- **Bug Report**: Use the bug report template
- **Feature Request**: Use the feature request template
- **Documentation**: Use the documentation template
- **Question**: Use the question template or discussions

### Writing Good Issues

- **Clear title**: Descriptive and specific
- **Complete template**: Fill out all relevant sections
- **Minimal example**: Provide minimal reproduction code
- **Environment info**: Include version, OS, Rust version
- **Expected vs actual**: Clearly describe the difference

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, backwards compatible

### Release Checklist

1. **Update version**: Update `Cargo.toml` version
2. **Update changelog**: Document all changes
3. **Run full test suite**: All tests must pass
4. **Create release PR**: Review all changes
5. **Tag release**: Create git tag after merge
6. **Publish**: Automated CI publishes to crates.io

## Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and community discussion
- **GitHub PRs**: Code review and collaboration

### Getting Help

1. **Check documentation**: README and docs.rs
2. **Search issues**: Look for existing solutions
3. **Ask in discussions**: Community Q&A
4. **Create an issue**: If you find a bug or need a feature

### Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- GitHub contributors page
- Release notes (for significant contributions)

## Development Philosophy

### Principles

1. **Correctness**: Code should be correct before being fast
2. **Safety**: Leverage Rust's safety features
3. **Performance**: Design with performance in mind
4. **Usability**: APIs should be intuitive and well-documented
5. **Maintainability**: Code should be easy to understand and modify

### Best Practices

- **Start small**: Begin with small, focused contributions
- **Ask questions**: Don't hesitate to ask for clarification
- **Share ideas**: Discuss major changes before implementing
- **Learn continuously**: Each contribution is a learning opportunity
- **Help others**: Review others' PRs and answer questions

## Thank You

Thank you for contributing to MCP Rust SDK! Your efforts help make this project better for everyone in the Rust and MCP communities.

For questions about contributing, feel free to:
- Open a discussion on GitHub
- Ask in an issue
- Reach out to maintainers

Happy coding! 🦀
