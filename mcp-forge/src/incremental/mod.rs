    /// Trait for incremental build operations, following SDK standards.
    pub trait IncrementalOps {
        fn cache_file_hash(&self, file_path: &Path) -> crate::error::McpResult<()>;
        fn detect_changes(&self, file_path: &Path) -> crate::error::McpResult<bool>;
        fn watch(&self) -> crate::error::McpResult<()>;
    }
pub mod incremental {
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use sha2::{Sha256, Digest};
    use std::sync::{Arc, Mutex};
    use notify::{Watcher, RecursiveMode, watcher};
    use std::time::Duration;
    use crate::error::{McpError, McpResult};

    #[derive(Default)]
    pub struct IncrementalBuilder {
        file_hashes: Arc<Mutex<HashMap<String, String>>>,
        cache_dir: PathBuf,
    }

    impl IncrementalOps for IncrementalBuilder {
        pub fn new(cache_dir: PathBuf) -> Self {
            Self {
                file_hashes: Arc::new(Mutex::new(HashMap::new())),
                cache_dir,
            }
        }

        pub fn cache_file_hash(&self, file_path: &Path) -> McpResult<()> {
            let mut hasher = Sha256::new();
            let content = fs::read(file_path).map_err(|e| McpError::Transport(e.to_string()))?;
            hasher.update(&content);
            let hash = format!("{:x}", hasher.finalize());

            let mut file_hashes = self.file_hashes.lock().unwrap();
            file_hashes.insert(file_path.to_string_lossy().to_string(), hash);
            Ok(())
        }

        pub fn detect_changes(&self, file_path: &Path) -> McpResult<bool> {
            let mut hasher = Sha256::new();
            let content = fs::read(file_path).map_err(|e| McpError::Transport(e.to_string()))?;
            hasher.update(&content);
            let current_hash = format!("{:x}", hasher.finalize());

            let file_hashes = self.file_hashes.lock().unwrap();
            if let Some(stored_hash) = file_hashes.get(file_path.to_string_lossy().as_ref()) {
                return Ok(&current_hash != stored_hash);
            }
            Ok(true)
        }

        pub fn watch(&self) -> McpResult<()> {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = watcher(tx, Duration::from_secs(2)).map_err(|e| McpError::Transport(e.to_string()))?;

            watcher.watch(&self.cache_dir, RecursiveMode::Recursive).map_err(|e| McpError::Transport(e.to_string()))?;

            loop {
                match rx.recv() {
                    Ok(event) => {
                        // Handle file change event
                        if let notify::DebouncedEvent::Write(path) = event {
                            if self.detect_changes(&path)? {
                                println!("File changed: {:?}", path);
                                // Trigger incremental build logic here
                            }
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
            // Ok(())
        }
    }
}