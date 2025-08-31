# Project Overview

MCP-Forge is a Rust CLI tool that analyzes codebases and generates an MCP server with deep semantic understanding. It supports multi-language, multi-module repositories and provides tools for code search, dependency analysis, and more.

---

## Key Features

- Multi-language AST analysis (TypeScript, JavaScript, Python, Rust, Java)
- Symbol extraction and knowledge graph construction
- Design pattern detection (Singleton, Factory, Observer, etc.)
- Embedding generation via Python bridge
- Incremental build and watch mode
- MCP server generation with FastMCP

---

## Typical Workflow

1. Initialize MCP-Forge in your project
2. Build the MCP server from your codebase
3. Use watch mode for live analysis and hot reload
4. Query the MCP server for insights
