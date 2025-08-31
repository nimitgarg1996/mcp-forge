CREATE TABLE symbols (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_line INTEGER,
    end_line INTEGER,
    scope TEXT,
    documentation TEXT,
    signature TEXT
);

CREATE TABLE relationships (
    id INTEGER PRIMARY KEY,
    from_symbol_id TEXT,
    to_symbol_id TEXT,
    kind TEXT,
    strength REAL,
    FOREIGN KEY (from_symbol_id) REFERENCES symbols(id),
    FOREIGN KEY (to_symbol_id) REFERENCES symbols(id)
);

CREATE TABLE embeddings (
    id INTEGER PRIMARY KEY,
    symbol_id TEXT,
    embedding BLOB,
    content TEXT,
    model_name TEXT,
    FOREIGN KEY (symbol_id) REFERENCES symbols(id)
);

CREATE TABLE patterns (
    id INTEGER PRIMARY KEY,
    name TEXT,
    pattern_type TEXT,
    occurrences INTEGER,
    locations TEXT
);