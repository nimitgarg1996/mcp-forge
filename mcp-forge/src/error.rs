//! Centralized error types for MCP-Forge, following rust-mcp-sdk standards.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Transport error: {0}")]
    Transport(String),
    #[error("Dispatch error: {0}")]
    Dispatch(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type McpResult<T> = Result<T, McpError>;
