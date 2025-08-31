# Architecture

MCP-Forge is organized into modular Rust crates and Python templates:

- **src/analyzer/**: Parsing, symbol extraction, pattern detection
- **src/knowledge/**: Knowledge graph and relationships
- **src/database/**: SQLite schema and async queries
- **src/embeddings/**: Python bridge for code embeddings
- **src/server/**: Build pipeline and MCP server generation
- **src/incremental/**: Incremental build and watch mode
- **templates/mcp-server/**: Python FastMCP server template

---

## Data Flow

1. File discovery (walkdir, .gitignore)
2. AST parsing (Tree-sitter)
3. Symbol extraction
4. Knowledge graph construction
5. Pattern detection
6. Embedding generation
7. Database storage
8. MCP server generation
