# Database Layer

- SQLite database stores all symbols, relationships, patterns, and embeddings.
- Async operations via sqlx for fast, concurrent access.
- Schema includes tables for symbols, relationships, embeddings, and patterns.
- Used by both the CLI and the generated MCP server for queries.

## Example Table: symbols

| id                                            | name    | kind   | file_path                            | start_line | end_line | scope   | documentation      | signature              |
| --------------------------------------------- | ------- | ------ | ------------------------------------ | ---------- | -------- | ------- | ------------------ | ---------------------- |
| backend/src/service/UserService.java::getUser | getUser | method | backend/src/service/UserService.java | 42         | 56       | backend | Gets a user by ID. | getUser(id: int): User |
