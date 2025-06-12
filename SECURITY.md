# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible
for receiving such patches depends on the CVSS v3.0 Rating:

| CVSS v3.0 | Supported Versions                        |
| --------- | ----------------------------------------- |
| 9.0-10.0  | Releases within the last three months    |
| 4.0-8.9   | Most recent release                       |

## Reporting a Vulnerability

Please report (suspected) security vulnerabilities to **rishi.randhawa@example.com**.
You will receive a response from us within 48 hours. If the issue is confirmed,
we will release a patch as soon as possible depending on complexity but historically
within a few days.

### Reporting Process

1. **Email**: Send your findings to rishi.randhawa@example.com
2. **Include**: As much information as possible about the vulnerability
3. **Wait**: We will acknowledge receipt within 48 hours
4. **Collaborate**: We may ask for additional information or guidance

### What to Include

When reporting a security vulnerability, please include:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact of the vulnerability
- Any suggested fixes or mitigations
- Your contact information for follow-up

### What to Expect

- **Acknowledgment**: We'll acknowledge receipt within 48 hours
- **Assessment**: We'll assess the vulnerability and determine severity
- **Communication**: We'll keep you informed of our progress
- **Fix**: We'll work on a fix and coordinate disclosure
- **Credit**: We'll credit you in the security advisory (if desired)

### Scope

This security policy applies to:

- All versions of the mcp-rust-sdk crate
- Security vulnerabilities in the core protocol implementation
- Transport layer security issues
- Authentication and authorization bypasses
- Code injection vulnerabilities

### Out of Scope

The following are generally considered out of scope:

- Social engineering attacks
- Physical attacks
- Denial of service attacks that require unreasonable resource consumption
- Vulnerabilities in third-party dependencies (please report to the respective maintainers)

### Security Best Practices

When using this SDK, please consider these security best practices:

1. **Validate Input**: Always validate and sanitize input to your tools and resources
2. **Least Privilege**: Run servers with minimal required permissions
3. **Secure Transport**: Use TLS/SSL for HTTP and WebSocket transports in production
4. **Authentication**: Implement proper authentication for your MCP servers
5. **Rate Limiting**: Implement rate limiting to prevent abuse
6. **Logging**: Enable security logging to detect and respond to threats
7. **Updates**: Keep dependencies and the SDK updated to the latest version

### Responsible Disclosure

We ask that you:

- Allow us reasonable time to address the issue before public disclosure
- Avoid exploiting the vulnerability or demonstrating additional impact
- Avoid accessing, modifying, or deleting other users' data
- Avoid degrading the user experience for other users

### Recognition

We believe in recognizing security researchers who help make our software safer:

- We'll acknowledge your contribution in the security advisory
- We'll include your name in our Hall of Fame (if you wish)
- For significant findings, we'll provide public recognition

Thank you for helping keep the MCP Rust SDK secure!
