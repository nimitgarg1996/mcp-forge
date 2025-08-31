# Core Modules

## Analyzer

- **parser.rs**: Tree-sitter integration for multi-language AST parsing
- **symbols.rs**: Symbol extraction for functions, classes, variables, etc.
- **patterns.rs**: Design pattern detection (language-aware)

## Knowledge

- **graph.rs**: Knowledge graph construction and metrics
- **relationships.rs**: Relationship mapping (calls, inheritance, dependencies)

## Database

- **schema.rs**: SQLite schema for symbols, relationships, patterns, embeddings
- **queries.rs**: Async database operations (sqlx)

## Embeddings

- **generator.rs**: Python bridge for sentence-transformers embeddings

## Server

- **builder.rs**: Build pipeline for MCP server generation

## Incremental

- **mod.rs**: Incremental build logic, file watching, cache
