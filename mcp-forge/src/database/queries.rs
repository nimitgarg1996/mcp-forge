/// Trait for database queries, following SDK standards.
pub trait DatabaseQueries {
    fn insert_symbol(&self, pool: &sqlx::SqlitePool, id: &str, name: &str, kind: &str, file_path: &str, start_line: i32, end_line: i32, scope: &str, documentation: Option<&str>, signature: Option<&str>) -> crate::error::McpResult<()>;
    fn get_symbol(&self, pool: &sqlx::SqlitePool, id: &str) -> crate::error::McpResult<Option<Symbol>>;
    fn insert_relationship(&self, pool: &sqlx::SqlitePool, from_symbol_id: &str, to_symbol_id: &str, kind: &str, strength: f64) -> crate::error::McpResult<()>;
    fn get_relationships(&self, pool: &sqlx::SqlitePool, symbol_id: &str) -> crate::error::McpResult<Vec<Relationship>>;
    fn insert_embedding(&self, pool: &sqlx::SqlitePool, symbol_id: &str, embedding: Vec<u8>, content: &str, model_name: &str) -> crate::error::McpResult<()>;
    fn get_embeddings(&self, pool: &sqlx::SqlitePool, symbol_id: &str) -> crate::error::McpResult<Vec<Embedding>>;
    fn insert_pattern(&self, pool: &sqlx::SqlitePool, name: &str, pattern_type: &str, occurrences: i32, locations: &str) -> crate::error::McpResult<()>;
    fn get_patterns(&self, pool: &sqlx::SqlitePool) -> crate::error::McpResult<Vec<Pattern>>;
}
use sqlx::{query, query_as, SqlitePool};
use crate::error::{McpError, McpResult};

pub async fn insert_symbol(pool: &SqlitePool, id: &str, name: &str, kind: &str, file_path: &str, start_line: i32, end_line: i32, scope: &str, documentation: Option<&str>, signature: Option<&str>) -> McpResult<()> {
    query("INSERT INTO symbols (id, name, kind, file_path, start_line, end_line, scope, documentation, signature) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(id)
        .bind(name)
        .bind(kind)
        .bind(file_path)
        .bind(start_line)
        .bind(end_line)
        .bind(scope)
        .bind(documentation)
        .bind(signature)
        .execute(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_symbol(pool: &SqlitePool, id: &str) -> McpResult<Option<Symbol>> {
    query_as::<_, Symbol>("SELECT * FROM symbols WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))
}

pub async fn insert_relationship(pool: &SqlitePool, from_symbol_id: &str, to_symbol_id: &str, kind: &str, strength: f64) -> McpResult<()> {
    query("INSERT INTO relationships (from_symbol_id, to_symbol_id, kind, strength) VALUES (?, ?, ?, ?)")
        .bind(from_symbol_id)
        .bind(to_symbol_id)
        .bind(kind)
        .bind(strength)
        .execute(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_relationships(pool: &SqlitePool, symbol_id: &str) -> McpResult<Vec<Relationship>> {
    query_as::<_, Relationship>("SELECT * FROM relationships WHERE from_symbol_id = ?")
        .bind(symbol_id)
        .fetch_all(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))
}

pub async fn insert_embedding(pool: &SqlitePool, symbol_id: &str, embedding: Vec<u8>, content: &str, model_name: &str) -> McpResult<()> {
    query("INSERT INTO embeddings (symbol_id, embedding, content, model_name) VALUES (?, ?, ?, ?)")
        .bind(symbol_id)
        .bind(embedding)
        .bind(content)
        .bind(model_name)
        .execute(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_embeddings(pool: &SqlitePool, symbol_id: &str) -> McpResult<Vec<Embedding>> {
    query_as::<_, Embedding>("SELECT * FROM embeddings WHERE symbol_id = ?")
        .bind(symbol_id)
        .fetch_all(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))
}

pub async fn insert_pattern(pool: &SqlitePool, name: &str, pattern_type: &str, occurrences: i32, locations: &str) -> McpResult<()> {
    query("INSERT INTO patterns (name, pattern_type, occurrences, locations) VALUES (?, ?, ?, ?)")
        .bind(name)
        .bind(pattern_type)
        .bind(occurrences)
        .bind(locations)
        .execute(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_patterns(pool: &SqlitePool) -> McpResult<Vec<Pattern>> {
    query_as::<_, Pattern>("SELECT * FROM patterns")
        .fetch_all(pool)
        .await
        .map_err(|e| McpError::Database(e.to_string()))
}