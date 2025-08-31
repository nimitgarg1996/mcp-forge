#[cfg(test)]
mod tests {
    use std::path::Path;
    use mcp_forge::analyzer::{discover_files, parser::CodeParser};

    #[test]
    fn test_discover_files() {
        let files = discover_files(Path::new("src"));
        assert!(!files.is_empty(), "Should discover files in src directory");
    }

    #[test]
    fn test_parse_python_file() {
        let mut parser = CodeParser::new();
        let path = Path::new("templates/mcp-server/generate_embeddings.py");
        let tree = parser.parse_file(path);
        assert!(tree.is_ok(), "Should parse Python file successfully");
    }
}
