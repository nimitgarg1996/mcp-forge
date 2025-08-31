/// Trait for code parsing, following SDK standards.
pub trait CodeParse {
    fn parse_file(&mut self, path: &std::path::Path) -> crate::error::McpResult<tree_sitter::Tree>;
}

use std::path::Path;
use tree_sitter::{Language, Parser, Tree};
use crate::error::{McpError, McpResult};

pub struct CodeParser {
    parsers: std::collections::HashMap<String, Parser>,
}

impl CodeParse for CodeParser {
}

impl CodeParser {
    pub fn new() -> Self {
        let mut parsers = std::collections::HashMap::new();
        parsers.insert("typescript".to_string(), Parser::new());
        parsers.insert("javascript".to_string(), Parser::new());
        parsers.insert("python".to_string(), Parser::new());
        parsers.insert("rust".to_string(), Parser::new());

        Self { parsers }
    }

    pub fn parse_file(&mut self, path: &Path) -> McpResult<Tree> {
        let extension = path.extension()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or(McpError::Transport("Unsupported file type".to_string()))?;

        let language = match extension {
            "ts" => "typescript",
            "js" => "javascript",
            "py" => "python",
            "rs" => "rust",
            _ => return Err(McpError::Transport("Unsupported file type".to_string())),
        };

        let parser = self.parsers.get_mut(language)
            .ok_or(McpError::Dispatch("Parser not found".to_string()))?;

        let source_code = std::fs::read_to_string(path)
            .map_err(|e| McpError::Transport(e.to_string()))?;

        let lang = match language {
            "typescript" => tree_sitter_typescript::language_typescript(),
            "javascript" => tree_sitter_javascript::language(),
            "python" => tree_sitter_python::language(),
            "rust" => tree_sitter_rust::language(),
            _ => return Err(McpError::Transport("Unsupported language".to_string())),
        };
    parser.set_language(lang).map_err(|_| McpError::Dispatch("Failed to set parser language".to_string()))?;
        let tree = parser.parse(&source_code, None)
            .ok_or(McpError::Dispatch("Failed to parse file".to_string()))?;

    Ok(tree)
    }
}