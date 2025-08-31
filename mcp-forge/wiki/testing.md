# Testing

- Unit tests for all core modules (analyzer, knowledge, database, embeddings, server).
- Integration tests for the full build pipeline using sample multi-module, multi-language projects.
- Benchmark tests for performance on large codebases.
- Test pattern detection accuracy and incremental build correctness.

## Example

### Rust Unit Test (File Discovery)

```rust
#[test]
fn test_discover_files() {
    let files = discover_files(Path::new("src"));
    assert!(!files.is_empty(), "Should discover files in src directory");
}
```

### Rust Unit Test (Python Parsing)

```rust
#[test]
fn test_parse_python_file() {
    let mut parser = CodeParser::new();
    let path = Path::new("templates/mcp-server/generate_embeddings.py");
    let tree = parser.parse_file(path);
    assert!(tree.is_ok(), "Should parse Python file successfully");
}
```

### Pattern Detection Test (Rust)

```rust
#[test]
fn test_detect_singleton_java() {
    let code = r#"
    public class ConfigManager {
        private static ConfigManager instance;
        private ConfigManager() {}
        public static ConfigManager getInstance() {
            if (instance == null) instance = new ConfigManager();
            return instance;
        }
    }
    "#;
    let confidence = detect_singleton_lang(code, "java");
    assert!(confidence > 0.5, "Should detect Singleton pattern in Java");
}
```

### Integration Test (Full Pipeline)

```rust
#[test]
fn test_full_pipeline() {
    // Discover files
    let files = discover_files(Path::new("src"));
    // Parse and extract symbols
    let parser = CodeParser::new();
    for file in files {
        let tree = parser.parse_file(&file);
        assert!(tree.is_ok());
        // ...extract symbols, detect patterns, store in DB...
    }
    // ...assert DB contains expected symbols/patterns...
}
```
