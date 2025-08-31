/// Trait for embedding generation, following SDK standards.
pub trait EmbeddingGen {
    fn generate_embeddings(&self, texts: Vec<String>) -> crate::error::McpResult<Vec<Vec<f32>>>;
}
use std::process::Command;
use std::path::PathBuf;
use serde_json::json;
use crate::error::{McpError, McpResult};

pub struct EmbeddingGenerator {
    python_env: PathBuf,
    model_name: String,
}

impl EmbeddingGen for EmbeddingGenerator {
    pub fn new(python_env: PathBuf, model_name: String) -> Self {
        Self { python_env, model_name }
    }

    pub fn generate_embeddings(&self, texts: Vec<String>) -> McpResult<Vec<Vec<f32>>> {
        let json_texts = serde_json::to_string(&texts).map_err(|e| McpError::Transport(e.to_string()))?;
        let output = Command::new("python3")
            .arg(self.python_env.join("generate_embeddings.py"))
            .arg(json_texts)
            .output()
            .map_err(|e| McpError::Transport(e.to_string()))?;

        if !output.status.success() {
            return Err(McpError::Transport(format!("Python script failed: {}", String::from_utf8_lossy(&output.stderr))));
        }

        let embeddings: Vec<Vec<f32>> = serde_json::from_slice(&output.stdout)
            .map_err(|e| McpError::Transport(e.to_string()))?;
        Ok(embeddings)
    }
}