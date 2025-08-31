# Advanced MCP-Forge Scenarios

## Scenario 1: Multi-Module, Multi-Language Monorepo

Suppose you have a repository with:

- `backend/` (Java, Python)
- `frontend/` (TypeScript, JavaScript)
- `shared/` (Rust)

**Workflow:**

```sh
mcp-forge build . --output dist/
```

- MCP-Forge will auto-detect modules and languages.
- Symbols and patterns are tagged with module context.
- Cross-language relationships (e.g., TypeScript frontend calling Rust shared code) are mapped in the knowledge graph.
- All results are available in the generated MCP server for semantic queries.

## Scenario 2: Custom Pattern Detection Extension

You want to add detection for a custom architectural pattern (e.g., CQRS):

1. Implement a new method in `PatternDetector` and extend the `PatternDetect` trait.
2. Add test cases in `tests/analyzer_tests.rs`.
3. Document the pattern in `wiki/pattern-detection.md`.
4. Rebuild and verify detection in the MCP server.

## Scenario 3: Integrating External Tools (e.g., Lint, Coverage)

You want MCP-Forge to surface lint errors and test coverage:

- Extend the build pipeline in `server/builder.rs` to run external tools (e.g., ESLint, pytest, Jacoco).
- Store results in the database and expose them via the MCP server API.
- Add new REST endpoints for lint/coverage queries.

## Scenario 4: Semantic Search with Natural Language

Query the MCP server for code matching a natural language description:

```python
# Python client example
import requests
resp = requests.post('http://localhost:3000/semantic_search', json={"query": "Find all functions that update user profiles"})
print(resp.json())
```

- The server uses embeddings to match code chunks to the query.
- Results include symbol metadata, file/module, and code snippet.

## Scenario 5: Automated Refactoring Suggestions

- Add a new tool to the MCP server for refactoring suggestions (e.g., extract method, rename variable).
- Use AST analysis and pattern detection to identify refactoring opportunities.
- Expose suggestions via the API and CLI.

---

For implementation details, see the API reference and onboarding guide.
