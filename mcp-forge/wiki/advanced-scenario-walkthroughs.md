# Advanced Scenario Implementation Walkthroughs

## 1. Multi-Module, Multi-Language Monorepo

**Step-by-step:**

1. Organize your repo:
   - `backend/` (Java, Python)
   - `frontend/` (TypeScript, JavaScript)
   - `shared/` (Rust)
2. Run:
   ```sh
   mcp-forge build . --output dist/
   ```
3. MCP-Forge will:
   - Discover all files, respecting `.gitignore` and module boundaries.
   - Parse each file with the correct Tree-sitter parser.
   - Tag symbols/patterns with module and language context.
   - Build a cross-language knowledge graph.
   - Store results in SQLite and expose via the MCP server.
4. Query the MCP server for cross-module relationships, semantic search, and pattern analysis.

## 2. Custom Pattern Detection Extension

**Step-by-step:**

1. In `src/analyzer/patterns.rs`, add a method to `PatternDetector`:
   ```rust
   impl PatternDetect for PatternDetector {
       fn detect_cqrs(&self, code: &str, ext: &str) -> Option<f32> {
           // Custom detection logic
       }
   }
   ```
2. Add a test in `tests/analyzer_tests.rs`:
   ```rust
   #[test]
   fn test_detect_cqrs() {
       let code = "...";
       let confidence = PatternDetector.detect_cqrs(code, "rs");
       assert!(confidence.is_some());
   }
   ```
3. Document the pattern in `wiki/pattern-detection.md`.
4. Rebuild and run `mcp-forge build` to verify detection.

## 3. Integrating External Tools (Lint, Coverage)

**Step-by-step:**

1. In `src/server/builder.rs`, add code to run external tools:
   ```rust
   let lint_output = std::process::Command::new("eslint").arg("src/").output()?;
   let coverage_output = std::process::Command::new("pytest").arg("--cov").output()?;
   ```
2. Parse and store results in the database.
3. Add REST endpoints in the MCP server to expose lint/coverage data.
4. Query via API or CLI for code quality insights.

## 4. Semantic Search with Natural Language

**Step-by-step:**

1. Ensure embeddings are generated for all code chunks.
2. In the MCP server, implement a `/semantic_search` endpoint:
   ```python
   @app.post('/semantic_search')
   def semantic_search(query: str):
       # Use embeddings to find matching code
   ```
3. Use the Python client example to query for code by description.

## 5. Automated Refactoring Suggestions

**Step-by-step:**

1. In the MCP server, add a tool for refactoring suggestions:
   ```python
   @app.post('/refactor_suggestions')
   def refactor_suggestions():
       # Analyze AST and patterns for refactoring opportunities
   ```
2. Use AST analysis to find candidates (e.g., long methods, duplicate code).
3. Return suggestions via API and CLI.

---

For more details, see the API reference and onboarding guide.
