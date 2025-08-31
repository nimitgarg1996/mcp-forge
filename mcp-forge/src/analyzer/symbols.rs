/// Trait for symbol extraction, following SDK standards.
pub trait SymbolExtract {
    fn extract_symbols(&self, ast: &tree_sitter::Node, file_path: &str) -> Vec<Symbol>;
}
pub struct SymbolExtractor {
    // Optionally store module context, etc.
}

impl SymbolExtract for SymbolExtractor {
}

impl SymbolExtractor {
    pub fn new() -> Self {
        // Initialize the SymbolExtractor
        SymbolExtractor {
            // Initialize fields
        }
    }

    pub fn extract_symbols(&self, ast: &tree_sitter::Node, file_path: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let mut to_visit = vec![*ast];
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let module = file_path.split('/').nth(1).unwrap_or(""); // crude module detection
        while let Some(node) = to_visit.pop() {
            let kind = node.kind();
            match kind {
                // Python
                "function_definition" | "def" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "function".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "class_definition" | "class_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "class".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                // JavaScript/TypeScript
                "function_declaration" | "method_definition" | "arrow_function" | "function_expression" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "function".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "variable_declaration" | "const_declaration" | "let_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "variable".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "interface_declaration" | "type_alias_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "interface".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                // Java
                "method_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "method".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "field_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "field".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "interface_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "interface".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "package_declaration" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: "package".to_string(),
                        file_path: file_path.to_string(),
                        start_line: node.start_position().row + 1,
                        end_line: node.end_position().row + 1,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                "import_declaration" | "import_statement" | "export_statement" => {
                    let name = node.child_by_field_name("name")
                        .map(|n| n.utf8_text(file_path.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    symbols.push(Symbol {
                        id: format!("{}::{}", file_path, name),
                        name,
                        kind: kind.to_string(),
                        file_path: file_path.to_string(),
                        start_line: node.start_position().row + 1,
                        end_line: node.end_position().row + 1,
                        scope: module.to_string(),
                        documentation: None,
                        signature: None,
                    });
                }
                _ => {}
            }
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    to_visit.push(child);
                }
            }
        }
        symbols
    }
}

pub struct Symbol {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub scope: String,
    pub documentation: Option<String>,
    pub signature: Option<String>,
}