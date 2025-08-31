# Embeddings

- Uses a Python bridge to generate semantic embeddings for code chunks via sentence-transformers.
- Embeddings are stored in the database and used for semantic code search and similarity queries.
- Enables natural language search in the MCP server.

## Example

```
Embedding: [0.123, 0.456, ...]
Symbol: backend/src/service/UserService.java::getUser
Model: all-MiniLM-L6-v2
```
