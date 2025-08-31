# Incremental Build & Watch Mode

- Uses SHA256 hashes and dependency graph to detect changed files.
- Only re-analyzes and updates affected symbols, relationships, patterns, and embeddings.
- Watch mode uses the notify crate to monitor file system events and triggers incremental builds.
- Hot reloads the MCP server for live updates.
- Works with uncommitted local changes (no git required).

## Example Workflow

1. Developer edits a file in `frontend/src/components/`
2. Watch mode detects the change
3. Only affected symbols and patterns are re-analyzed
4. MCP server is hot-reloaded with new insights
