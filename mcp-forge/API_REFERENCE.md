# MCP-Forge API Reference

This document provides a detailed API reference for all major traits, structs, and functions in MCP-Forge, following SDK standards.

## Traits

### CodeParse

```rust
pub trait CodeParse {
    fn parse_file(&mut self, path: &std::path::Path) -> McpResult<tree_sitter::Tree>;
}
```

- Implemented by: `CodeParser`
- Usage: Parse a source file into an AST using Tree-sitter.

### SymbolExtract

```rust
pub trait SymbolExtract {
    fn extract_symbols(&self, ast: &tree_sitter::Node, file_path: &str) -> Vec<Symbol>;
}
```

- Implemented by: `SymbolExtractor`
- Usage: Extract symbols (functions, classes, etc.) from an AST.

### PatternDetect

```rust
pub trait PatternDetect {
    fn detect_patterns(&self, code: &str, ext: &str) -> Vec<(String, f32)>;
}
```

- Implemented by: `PatternDetector`
- Usage: Detect design patterns in code, returning pattern name and confidence.

### KnowledgeGraphOps

```rust
pub trait KnowledgeGraphOps {
    fn build(&mut self, symbols: &[SymbolNode]);
    fn extract_relationships_from_ast(&mut self, ast: &tree_sitter::Node, file_path: &str);
}
```

- Implemented by: `KnowledgeGraph`
- Usage: Build and query the knowledge graph of symbols and relationships.

### EmbeddingGen

```rust
pub trait EmbeddingGen {
    fn generate_embeddings(&self, texts: Vec<String>) -> McpResult<Vec<Vec<f32>>>;
}
```

- Implemented by: `EmbeddingGenerator`
- Usage: Generate semantic embeddings for code chunks.

### IncrementalOps

```rust
pub trait IncrementalOps {
    fn cache_file_hash(&self, file_path: &Path) -> McpResult<()>;
    fn detect_changes(&self, file_path: &Path) -> McpResult<bool>;
    fn watch(&self) -> McpResult<()>;
}
```

- Implemented by: `IncrementalBuilder`
- Usage: Incremental build logic, file change detection, and watch mode.

### DatabaseQueries

```rust
pub trait DatabaseQueries {
    fn insert_symbol(&self, pool: &SqlitePool, ...);
    fn get_symbol(&self, pool: &SqlitePool, ...);
    // ...other DB methods...
}
```

- Implemented by: DB query functions
- Usage: Async database operations for symbols, relationships, embeddings, patterns.

## Error Handling

- All public APIs use `McpError` and `McpResult<T>` for error handling.
- Error types: Transport, Dispatch, Database, Unknown.

## Example Usage

```rust
let mut parser = CodeParser::new();
let tree = parser.parse_file(Path::new("src/main.rs"))?;
let symbols = SymbolExtractor.extract_symbols(&tree.root_node(), "src/main.rs");
let patterns = PatternDetector.detect_patterns(&code, "rs");
let mut graph = KnowledgeGraph::new();
graph.build(&symbols);
```

---

See the wiki for module-level details and more examples.
