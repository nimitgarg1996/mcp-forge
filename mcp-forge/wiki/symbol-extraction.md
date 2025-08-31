# Symbol Extraction

- Uses Tree-sitter for AST parsing in TypeScript, JavaScript, Python, Rust, and Java.
- Extracts functions, methods, classes, variables, interfaces, types, modules, packages, imports/exports.
- Associates each symbol with file, module, scope, documentation, and signature.
- Supports multi-module repositories (e.g., backend, frontend, shared).

## Example

```rust
Symbol {
    id: "backend/src/service/UserService.java::getUser",
    name: "getUser",
    kind: "method",
    file_path: "backend/src/service/UserService.java",
    start_line: 42,
    end_line: 56,
    scope: "backend",
    documentation: Some("Gets a user by ID."),
    signature: Some("getUser(id: int): User"),
}
```
