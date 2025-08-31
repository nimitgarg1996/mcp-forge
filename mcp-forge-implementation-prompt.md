# MCP-Forge Implementation Prompt

## Project Overview

Build MCP-Forge, a Rust-based CLI tool that analyzes any codebase and generates a specialized MCP (Model Context Protocol) server. The generated server acts as an AI expert on that specific codebase, providing deep semantic understanding through AST analysis, knowledge graphs, and embeddings.

## Core Requirements

### Technology Stack
- **Language**: Rust (for the CLI tool)
- **Parsing**: Tree-sitter for multi-language AST analysis
- **Database**: SQLite for knowledge storage
- **Embeddings**: sentence-transformers (via Python bridge)
- **Generated Server**: Python with FastMCP

### Project Structure
```
mcp-forge/
├── Cargo.toml
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── analyzer/
│   │   ├── mod.rs
│   │   ├── parser.rs           # Tree-sitter integration
│   │   ├── symbols.rs          # Symbol extraction
│   │   └── patterns.rs         # Pattern detection
│   ├── knowledge/
│   │   ├── mod.rs
│   │   ├── graph.rs            # Knowledge graph construction
│   │   └── relationships.rs    # Relationship mapping
│   ├── database/
│   │   ├── mod.rs
│   │   ├── schema.rs           # SQLite schema
│   │   └── queries.rs          # Database operations
│   ├── embeddings/
│   │   ├── mod.rs
│   │   └── generator.rs        # Embedding generation
│   ├── server/
│   │   ├── mod.rs
│   │   └── builder.rs          # MCP server generation
│   └── incremental/
│       └── mod.rs              # Incremental build support
├── templates/
│   └── mcp-server/
│       ├── server.py           # MCP server template
│       ├── requirements.txt    # Python dependencies
│       └── start.sh           # Startup script
└── tests/
```

## Implementation Instructions

### Step 1: Initialize Rust Project

Create a new Rust project with these dependencies in `Cargo.toml`:

```toml
[package]
name = "mcp-forge"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
tree-sitter = "0.20"
tree-sitter-typescript = "0.20"
tree-sitter-javascript = "0.20"
tree-sitter-python = "0.20"
tree-sitter-rust = "0.20"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-rustls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
walkdir = "2.3"
ignore = "0.4"
notify = "6.0"
rayon = "1.7"
sha2 = "0.10"
chrono = { version = "0.4", features = ["serde"] }
petgraph = "0.6"
env_logger = "0.10"
log = "0.4"
```

### Step 2: Implement CLI Interface (src/main.rs)

Create the main CLI with these commands:
- `init`: Initialize MCP-Forge in current directory
- `build <source> [--output <path>]`: Build MCP server from codebase
- `watch <source> [--port <port>]`: Watch mode with auto-rebuild
- `analyze <source>`: Show codebase statistics

The CLI should use clap derive macros for clean argument parsing.

### Step 3: Implement Tree-sitter Parser (src/analyzer/parser.rs)

Create a `CodeParser` struct that:
1. Initializes Tree-sitter parsers for TypeScript, JavaScript, Python, and Rust
2. Parses files and extracts AST
3. Implements `parse_file(&Path) -> Result<ParsedFile>`
4. Detects language from file extension
5. Handles parse errors gracefully

### Step 4: Extract Symbols (src/analyzer/symbols.rs)

Create a `SymbolExtractor` that traverses the AST and extracts:
- Functions/methods (name, parameters, return type, body location)
- Classes (name, members, inheritance)
- Variables/constants (name, type, scope)
- Interfaces/types (TypeScript)
- Modules/packages

Each symbol should include:
- Unique ID (file_path::symbol_name)
- Location (file, start_line, end_line)
- Documentation (preceding comments)
- Scope (global, class, function)
- Signature (for functions)

### Step 5: Build Knowledge Graph (src/knowledge/graph.rs)

Implement `KnowledgeGraph` struct that:
1. Creates nodes for each symbol
2. Identifies relationships:
   - Function calls (A calls B)
   - Class inheritance (A extends B)
   - Interface implementation
   - Variable usage
   - Import/export dependencies
3. Calculates metrics:
   - Coupling (afferent/efferent)
   - Complexity scores
   - Centrality measures

Use petgraph for graph operations.

### Step 6: Database Layer (src/database/schema.rs)

Create SQLite schema with tables:

```sql
CREATE TABLE symbols (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_line INTEGER,
    end_line INTEGER,
    scope TEXT,
    documentation TEXT,
    signature TEXT
);

CREATE TABLE relationships (
    id INTEGER PRIMARY KEY,
    from_symbol_id TEXT,
    to_symbol_id TEXT,
    kind TEXT,
    strength REAL,
    FOREIGN KEY (from_symbol_id) REFERENCES symbols(id),
    FOREIGN KEY (to_symbol_id) REFERENCES symbols(id)
);

CREATE TABLE embeddings (
    id INTEGER PRIMARY KEY,
    symbol_id TEXT,
    embedding BLOB,
    content TEXT,
    model_name TEXT,
    FOREIGN KEY (symbol_id) REFERENCES symbols(id)
);

CREATE TABLE patterns (
    id INTEGER PRIMARY KEY,
    name TEXT,
    pattern_type TEXT,
    occurrences INTEGER,
    locations TEXT
);
```

Implement async functions using sqlx for all database operations.

### Step 7: Pattern Detection (src/analyzer/patterns.rs)

Implement pattern detection for:
1. **Singleton**: Private constructor + getInstance method
2. **Factory**: Methods returning different types based on input
3. **Observer**: Subscribe/notify patterns
4. **Builder**: Chained method calls
5. **Repository**: Data access patterns
6. **MVC**: Model-View-Controller structure
7. **Dependency Injection**: Constructor parameters
8. **Decorator**: Wrapper patterns

Each pattern detector should return confidence scores.

### Step 8: Embedding Generation (src/embeddings/generator.rs)

Create Python bridge for embeddings:
1. Set up Python virtual environment programmatically
2. Install sentence-transformers if not present
3. Generate embeddings for code chunks
4. Store as BLOB in SQLite

Python script to call:
```python
import sys
import json
from sentence_transformers import SentenceTransformer

model = SentenceTransformer('all-MiniLM-L6-v2')
texts = json.loads(sys.argv[1])
embeddings = model.encode(texts)
print(json.dumps(embeddings.tolist()))
```

### Step 9: MCP Server Template (templates/mcp-server/server.py)

Create a complete MCP server template with these tools:

```python
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("{{PROJECT_NAME}}_Expert")

@mcp.tool()
def get_project_overview() -> dict:
    """Get comprehensive project statistics and structure."""
    # Query database for metrics
    pass

@mcp.tool()
def find_symbol(name: str, kind: str = None) -> list:
    """Find symbols by name and optional type."""
    # Search symbols table
    pass

@mcp.tool()
def analyze_dependencies(symbol_name: str, depth: int = 2) -> dict:
    """Trace dependencies for a symbol."""
    # Traverse relationship graph
    pass

@mcp.tool()
def trace_data_flow(variable: str) -> dict:
    """Trace how data flows through the system."""
    # Follow variable usage
    pass

@mcp.tool()
def find_patterns(pattern_type: str = None) -> list:
    """Find design patterns in codebase."""
    # Query patterns table
    pass

@mcp.tool()
def analyze_complexity(module: str = None, threshold: int = 10) -> dict:
    """Analyze code complexity metrics."""
    # Calculate complexity scores
    pass

@mcp.tool()
def suggest_refactoring(symbol: str = None) -> list:
    """Suggest refactoring opportunities."""
    # Analyze issues and suggest improvements
    pass

@mcp.tool()
def semantic_search(query: str, limit: int = 10) -> list:
    """Search code using natural language."""
    # Use embeddings for similarity search
    pass

@mcp.tool()
def get_test_coverage(module: str = None) -> dict:
    """Analyze test coverage."""
    # Find test files and map to source
    pass

@mcp.tool()
def explain_architecture() -> dict:
    """Explain the overall system architecture."""
    # Analyze top-level structure
    pass
```

### Step 10: Build Pipeline (src/server/builder.rs)

Implement the complete build pipeline:

```rust
pub async fn build(source: PathBuf, output: PathBuf) -> Result<()> {
    // 1. Collect source files (respect .gitignore)
    // 2. Parse files with Tree-sitter (parallel processing with rayon)
    // 3. Extract symbols from AST
    // 4. Build knowledge graph
    // 5. Detect patterns
    // 6. Generate embeddings (batch processing)
    // 7. Store everything in SQLite
    // 8. Copy server template to output
    // 9. Copy database to output directory
    // 10. Generate config.json with project metadata
    // 11. Make start.sh executable
}
```

### Step 11: Incremental Build Support (src/incremental/mod.rs)

Implement incremental building:
1. Cache file hashes in `.mcp-forge-cache/`
2. Detect changed files using SHA256
3. Find affected symbols through dependency graph
4. Only rebuild changed components
5. Update database incrementally

### Step 12: Watch Mode (src/watch/mod.rs)

Implement file watching:
1. Use notify crate for file system events
2. Debounce rapid changes (2 second delay)
3. Trigger incremental builds
4. Hot reload the MCP server
5. Show live statistics in terminal

### Step 13: Testing

Create comprehensive tests:
- Unit tests for each component
- Integration test with sample TypeScript project
- Test pattern detection accuracy
- Benchmark performance on large codebases
- Test incremental build correctness

### Step 14: Error Handling

Implement robust error handling:
- Use anyhow for error propagation
- Graceful handling of unsupported languages
- Clear error messages for users
- Recovery from partial failures
- Progress indicators for long operations

### Step 15: Documentation

Create documentation:
- README with quick start guide
- Configuration file examples (.mcpforge.yaml)
- API documentation for generated server
- Architecture overview
- Contributing guidelines

## Key Implementation Details

### Symbol Extraction Algorithm
For each AST node, check if it matches known patterns:
- Functions: `function_declaration`, `method_definition`, `arrow_function`
- Classes: `class_declaration`, `class_definition`
- Variables: `variable_declaration`, `const_declaration`
- Imports: `import_statement`, `import_from`

### Relationship Detection
After extracting symbols, make a second pass to find:
- Function calls: Look for `call_expression` nodes
- Inheritance: Check `extends` clauses
- Implementations: Check `implements` clauses
- Imports: Track module dependencies

### Performance Optimizations
- Use rayon for parallel file processing
- Batch database inserts (chunks of 1000)
- Memory-mapped file reading for large files
- Incremental index updates
- LRU cache for frequently accessed symbols

### Configuration File Format
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

analysis:
  patterns: true
  embeddings: true
  complexity_threshold: 10

server:
  port: 3000
  auto_reload: true
```

## Success Criteria

The implementation is complete when:
1. ✅ Can analyze a 10,000 file TypeScript project in under 5 minutes
2. ✅ Generates working MCP server with all 10 tools functional
3. ✅ Incremental builds complete in under 10 seconds
4. ✅ Watch mode works with hot reload
5. ✅ All pattern detectors have >80% accuracy
6. ✅ Semantic search returns relevant results
7. ✅ Database queries respond in <100ms
8. ✅ Single binary under 50MB
9. ✅ Works on Linux, macOS, and Windows
10. ✅ Comprehensive test coverage (>80%)

## Additional Notes for AI Assistant

- Prioritize working code over perfect code - iterate quickly
- Use existing crates/libraries where possible
- Add logging throughout for debugging
- Handle edge cases (empty files, syntax errors, etc.)
- Make the CLI output user-friendly with progress bars
- Consider adding a --verbose flag for detailed output
- Cache Tree-sitter parsers for performance
- Use async/await for I/O operations
- Implement graceful shutdown in watch mode
- Add version checking for compatibility

## Example Usage After Implementation

```bash
# Initialize in a TypeScript project
cd ~/my-typescript-project
mcp-forge init --name "MyProject"

# Build the MCP server
mcp-forge build . --output ./mcp-server

# Start in watch mode
mcp-forge watch . --port 3000

# In another terminal, test the server
curl -X POST http://localhost:3000/tool \
  -H "Content-Type: application/json" \
  -d '{"tool": "get_project_overview"}'
```

This should return detailed project statistics and demonstrate that the MCP server is working correctly.

## Start Building

Begin with `src/main.rs` to establish the CLI structure, then implement components in this order:
1. File discovery (walkdir + gitignore)
2. Tree-sitter parsing for one language
3. Symbol extraction
4. Database layer
5. Basic MCP server generation
6. Knowledge graph
7. Pattern detection
8. Embeddings
9. Incremental builds
10. Watch mode

Each component should be testable independently. Focus on getting an end-to-end flow working first, then add advanced features.