# GitHub Repository Configuration Guide

## Branch Protection Rules

### Main Branch Protection
Configure the following settings for the `main` branch:

- ✅ **Require a pull request before merging**
  - Require approvals: 1
  - Dismiss stale PR approvals when new commits are pushed
  - Require review from code owners

- ✅ **Require status checks to pass before merging**
  - Require branches to be up to date before merging
  - Required status checks:
    - CI / Check
    - CI / Test Suite (ubuntu-latest, stable)
    - CI / Rustfmt
    - CI / Clippy
    - CI / Documentation
    - CI / Security Audit

- ✅ **Require conversation resolution before merging**
- ✅ **Require signed commits**
- ✅ **Include administrators** (enforce rules for admins too)
- ✅ **Allow force pushes: Everyone** (disabled)
- ✅ **Allow deletions** (disabled)

## Repository Settings

### General
- Description: "A comprehensive Rust SDK for the Model Context Protocol (MCP)"
- Website: (link to docs.rs when published)
- Topics: rust, mcp, model-context-protocol, sdk, ai, llm, tool-calling

### Features
- ✅ Wikis: Disabled (use README and docs)
- ✅ Issues: Enabled
- ✅ Sponsorships: Enabled (if using GitHub Sponsors)
- ✅ Preserve this repository: Enabled (for archival)
- ✅ Discussions: Consider enabling for community Q&A

### Pull Requests
- ✅ Allow merge commits: Disabled
- ✅ Allow squash merging: Enabled (default)
- ✅ Allow rebase merging: Enabled
- ✅ Always suggest updating pull request branches: Enabled
- ✅ Automatically delete head branches: Enabled

### Security
- ✅ Dependency graph: Enabled
- ✅ Dependabot alerts: Enabled
- ✅ Dependabot security updates: Enabled
- ✅ Dependabot version updates: Enabled (via dependabot.yml)

### Actions
- ✅ Actions permissions: Allow all actions and reusable workflows
- ✅ Fork pull request workflows: Require approval for first-time contributors

## Labels Configuration

Create these additional labels for better issue/PR organization:

### Type Labels
- `type:bug` (color: #d73a4a) - Something isn't working
- `type:feature` (color: #a2eeef) - New feature or request
- `type:docs` (color: #0075ca) - Documentation improvements
- `type:performance` (color: #fbca04) - Performance improvements
- `type:refactor` (color: #7057ff) - Code refactoring

### Priority Labels  
- `priority:low` (color: #0e8a16) - Low priority
- `priority:medium` (color: #fbca04) - Medium priority
- `priority:high` (color: #ff6b35) - High priority
- `priority:critical` (color: #d73a4a) - Critical priority

### Component Labels
- `component:transport` (color: #1d76db) - Transport layer
- `component:server` (color: #1d76db) - Server implementation
- `component:client` (color: #1d76db) - Client implementation
- `component:protocol` (color: #1d76db) - Protocol implementation

### Status Labels
- `status:blocked` (color: #d73a4a) - Blocked by dependencies
- `status:wip` (color: #fbca04) - Work in progress
- `status:ready` (color: #0e8a16) - Ready for implementation

## Secrets Configuration

If using automated releases or publishing to crates.io:

### Repository Secrets
- `CARGO_REGISTRY_TOKEN`: Token for publishing to crates.io
- `CODECOV_TOKEN`: Token for code coverage reporting (if using Codecov)

## Environments

### Production Environment
For automated releases:
- Environment name: `production`
- Protection rules: Require reviewers (yourself)
- Environment secrets: Same as repository secrets but environment-scoped
