# MCP-Forge

MCP-Forge is a Rust-based CLI tool that analyzes any codebase and generates a specialized MCP (Model Context Protocol) server. The generated server acts as an AI expert for your codebase, providing deep semantic understanding through AST analysis, knowledge graphs, embeddings, and design pattern detection.

## Features

- **Multi-language Support**: Analyze TypeScript, JavaScript, Python, Rust, and Java codebases. Supports mixed-language and multi-module repositories (e.g., backend, frontend, shared).
- **AST Analysis**: Uses Tree-sitter for fast, accurate abstract syntax tree parsing across all supported languages.
- **Symbol Extraction**: Extracts functions, classes, methods, variables, interfaces, types, modules, and more, with module context.
- **Knowledge Graphs**: Builds a graph of relationships (calls, inheritance, dependencies) between all symbols in your codebase.
- **Pattern Detection**: Identifies Singleton, Factory, Observer, Builder, Repository, MVC, Dependency Injection, Decorator, and more, for all supported languages.
- **Embeddings Generation**: Generates semantic embeddings for code chunks using sentence-transformers (Python bridge).
- **Incremental Builds**: Only re-analyzes changed files using SHA256 hashes and dependency graph, for fast rebuilds.
- **Watch Mode**: Monitors your codebase for changes and automatically triggers incremental analysis and hot-reloads the MCP server.
- **Progress Indicators & Logging**: User-friendly CLI output with progress bars and verbose logging.
- **Cross-platform**: Works on Linux, macOS, and Windows.

## Installation

1. **Install Rust & Cargo**: https://rustup.rs/
2. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/mcp-forge.git
   cd mcp-forge
   ```
3. **Build the project**:
   ```sh
   cargo build --release
   ```

## Usage

### Initialize a Project

```sh
mcp-forge init --name "MyProject"
```

### Build the MCP Server

```sh
mcp-forge build <source> [--output <path>]
```

### Watch Mode (Live Analysis)

```sh
mcp-forge watch <source> [--port <port>]
```

**How Watch Mode Works:**

- Monitors all source files (including multi-module repos: backend, frontend, shared, etc.).
- Detects changes instantly (no git commit required).
- Triggers incremental analysis and rebuilds only affected files/modules.
- Hot reloads the MCP server so new insights are available immediately.
- Shows live progress and statistics in the terminal.

### Analyze Codebase

```sh
mcp-forge analyze <source>
```

## Multi-Module & Multi-Language Support

- MCP-Forge automatically detects modules (e.g., `backend/`, `frontend/`, `shared/`) and tags all symbols and patterns with module context.
- Supports mixed-language repositories (Java, Python, TypeScript, JavaScript, Rust).
- Uses `.gitignore` and config files to exclude irrelevant files/folders.

## Design Pattern Detection

Detects the following patterns in all supported languages:

- Singleton
- Factory
- Observer
- Builder
- Repository
- MVC (Model-View-Controller)
- Dependency Injection
- Decorator

Pattern detection is language-aware and uses AST and code heuristics for high accuracy.

## Incremental Build & Hot Reload

- Uses SHA256 hashes and dependency graph to detect changed files.
- Only re-analyzes and updates affected symbols, relationships, patterns, and embeddings.
- Keeps the MCP server up-to-date with minimal overhead.

## MCP Server Template

- Generated server uses FastMCP (Python) and exposes tools for project overview, symbol search, dependency analysis, pattern search, semantic code search, test coverage, and architecture explanation.
- Embeddings enable natural language code search and semantic queries.

## Configuration

You can use a `.mcpforge.yaml` or similar config file to:

- Exclude files/folders (e.g., `node_modules/`, `*.test.ts`)
- Set max file size for analysis
- Specify languages/modules to include
- Tune analysis options (patterns, embeddings, complexity threshold)

Example:

```yaml
project:
  name: "MyProject"
  description: "AI expert for my codebase"

indexing:
  exclude_patterns:
    - "*.test.ts"
    - "node_modules/"
  max_file_size: "5MB"
  languages:
    - typescript
    - javascript
    - python
    - java

analysis:
  patterns: true
  embeddings: true
  complexity_threshold: 10

server:
  port: 3000
  auto_reload: true
```

## Output & API

- All analysis results (symbols, relationships, patterns, embeddings) are stored in SQLite and available to the generated MCP server.
- The MCP server exposes a REST API for querying project statistics, symbol search, dependency analysis, pattern search, semantic code search, and more.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone the repository:
   ```
   git clone https://github.com/yourusername/mcp-forge.git
   cd mcp-forge
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

### Initialize a Project

To initialize MCP-Forge in your current directory, run:

```
mcp-forge init --name "MyProject"
```

### Build the MCP Server

To build the MCP server from your codebase, use:

```
mcp-forge build <source> [--output <path>]
```

### Watch Mode

To start the tool in watch mode, which automatically rebuilds on file changes:

```
mcp-forge watch <source> [--port <port>]
```

### Analyze Codebase

To show codebase statistics, run:

```
mcp-forge analyze <source>
```

## Project Structure

```
mcp-forge/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── analyzer/
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── symbols.rs
│   │   └── patterns.rs
│   ├── knowledge/
│   │   ├── mod.rs
│   │   ├── graph.rs
│   │   └── relationships.rs
│   ├── database/
│   │   ├── mod.rs
│   │   ├── schema.rs
│   │   └── queries.rs
│   ├── embeddings/
│   │   ├── mod.rs
│   │   └── generator.rs
│   ├── server/
│   │   ├── mod.rs
│   │   └── builder.rs
│   └── incremental/
│       └── mod.rs
├── templates/
│   └── mcp-server/
│       ├── server.py
│       ├── requirements.txt
│       └── start.sh
└── tests/
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
