# Knowledge Graph

- Constructs a graph of all symbols and their relationships (calls, inheritance, dependencies).
- Uses petgraph for graph operations and metrics (coupling, complexity, centrality).
- Enables advanced queries (e.g., "What symbols depend on X?", "Which modules are most coupled?").

## Example

```
Node: backend::UserService
Edge: calls -> backend::UserRepository
Edge: inherits -> shared::BaseService
```
