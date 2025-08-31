use indicatif::{ProgressBar, ProgressStyle};

pub async fn build(source: std::path::PathBuf, output: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use crate::error::{McpError, McpResult};

    // 1. Collect source files (respect .gitignore)
    let source_files = walkdir::WalkDir::new(&source)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                Some(entry.path().to_path_buf())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // 2. Parse files with Tree-sitter (parallel processing with rayon)
    let parsed_files: Vec<ParsedFile> = rayon::scope(|s| {
        let mut results = Vec::new();
        for file in source_files {
            s.spawn(|_| {
                if let Ok(parsed) = CodeParser::parse_file(&file) {
                    results.push(parsed);
                }
            });
        }
        results
    });

    // 3. Extract symbols from AST and detect patterns per file/module
    let mut symbol_extractor = SymbolExtractor::new();
    let mut all_patterns = Vec::new();
    for parsed in &parsed_files {
        let file_path = parsed.file_path(); // Assume ParsedFile has file_path()
        let ext = std::path::Path::new(&file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let module = file_path.split('/').nth(1).unwrap_or("");
        symbol_extractor.extract(parsed);
        let code = std::fs::read_to_string(&file_path).unwrap_or_default();
        let pattern_detector = PatternDetector::new();
        let detected = pattern_detector.detect_patterns(&code, ext);
        for (name, confidence) in detected {
            // Store pattern with module context
            all_patterns.push((name, ext.to_string(), module.to_string(), confidence, file_path.clone()));
        }
    }

    // 4. Build knowledge graph
    let mut knowledge_graph = KnowledgeGraph::new();
    knowledge_graph.build(&symbol_extractor.symbols);

    // 5. Patterns for all files/modules
    // Convert all_patterns to pattern DB objects as needed
    let patterns = all_patterns; // Replace with proper struct if needed

    // 6. Generate embeddings (batch processing)
    let embeddings = EmbeddingGenerator::generate(&symbol_extractor.symbols);

    // 7. Store everything in SQLite
    let db = Database::connect(&output.join("database.sqlite")).await?;
    let pb_db_symbols = ProgressBar::new(symbol_extractor.symbols.len() as u64);
    pb_db_symbols.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/blue}] {pos}/{len} {msg}")
        .progress_chars("##-"));
    pb_db_symbols.set_message("Storing symbols in database");
    for symbol in &symbol_extractor.symbols {
        db.store_symbol(symbol).await?;
        pb_db_symbols.inc(1);
    }
    pb_db_symbols.finish_with_message("Symbols stored");

    let pb_db_rels = ProgressBar::new(knowledge_graph.relationships.len() as u64);
    pb_db_rels.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .progress_chars("##-"));
    pb_db_rels.set_message("Storing relationships in database");
    for rel in &knowledge_graph.relationships {
        db.store_relationship(rel).await?;
        pb_db_rels.inc(1);
    }
    pb_db_rels.finish_with_message("Relationships stored");

    let pb_db_emb = ProgressBar::new(embeddings.len() as u64);
    pb_db_emb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.magenta/blue}] {pos}/{len} {msg}")
        .progress_chars("##-"));
    pb_db_emb.set_message("Storing embeddings in database");
    for emb in &embeddings {
        db.store_embedding(emb).await?;
        pb_db_emb.inc(1);
    }
    pb_db_emb.finish_with_message("Embeddings stored");

    let pb_db_pat = ProgressBar::new(patterns.len() as u64);
    pb_db_pat.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.yellow/blue}] {pos}/{len} {msg}")
        .progress_chars("##-"));
    pb_db_pat.set_message("Storing patterns in database");
    for pat in &patterns {
        db.store_pattern(pat).await?;
        pb_db_pat.inc(1);
    }
    pb_db_pat.finish_with_message("Patterns stored");

    // 8. Copy server template to output
    std::fs::copy("templates/mcp-server/server.py", output.join("server.py")).map_err(|e| McpError::Transport(e.to_string()))?;
    std::fs::copy("templates/mcp-server/requirements.txt", output.join("requirements.txt")).map_err(|e| McpError::Transport(e.to_string()))?;
    std::fs::copy("templates/mcp-server/start.sh", output.join("start.sh")).map_err(|e| McpError::Transport(e.to_string()))?;

    // 9. Copy database to output directory
    std::fs::copy(output.join("database.sqlite"), output.join("database.sqlite")).map_err(|e| McpError::Transport(e.to_string()))?;

    // 10. Generate config.json with project metadata
    let config = json!({
        "project": {
            "name": "MyProject",
            "description": "AI expert for my codebase"
        }
    });
    std::fs::write(output.join("config.json"), serde_json::to_string(&config)?)?;

    // 11. Make start.sh executable
    let mut permissions = std::fs::metadata(output.join("start.sh")).map_err(|e| McpError::Transport(e.to_string()))?.permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(output.join("start.sh"), permissions).map_err(|e| McpError::Transport(e.to_string()))?;

    Ok(())
}