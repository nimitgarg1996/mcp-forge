# MCP-Forge Onboarding Guide

Welcome to MCP-Forge! This guide will help you get started as a contributor or user.

## Prerequisites

- Rust & Cargo (https://rustup.rs/)
- Python 3 (for embeddings)
- SQLite (for database)

## Project Setup

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/mcp-forge.git
   cd mcp-forge
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```
3. (Optional) Set up Python environment for embeddings:
   ```sh
   cd templates/mcp-server
   python3 -m venv .venv
   source .venv/bin/activate
   pip install -r requirements.txt
   ```

## Key Concepts

- **Traits**: All major modules use public traits for extensibility (see API_REFERENCE.md).
- **Error Handling**: All APIs use `McpError` and `McpResult<T>`.
- **Multi-language & Multi-module**: Supports TypeScript, JavaScript, Python, Rust, Java; tags symbols/patterns with module context.
- **Incremental Build & Watch**: Only changed files are re-analyzed; watch mode triggers hot reloads.

## Typical Workflow

1. **Initialize**: `mcp-forge init --name "MyProject"`
2. **Build**: `mcp-forge build <source> [--output <path>]`
3. **Watch**: `mcp-forge watch <source> [--port <port>]`
4. **Analyze**: `mcp-forge analyze <source>`

## Contributing

- Follow Rust best practices and SDK trait conventions.
- Write unit and integration tests for new features (see wiki/testing.md).
- Document new traits, structs, and functions in code and API_REFERENCE.md.
- Open issues or pull requests for improvements.

## Resources

- [API Reference](./API_REFERENCE.md)
- [Wiki](./wiki/Home.md)
- [Testing Guide](./wiki/testing.md)

For questions, open an issue or join the project discussions!
