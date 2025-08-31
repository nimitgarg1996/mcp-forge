mod.rs

pub mod parser;
pub mod symbols;
pub mod patterns;

use walkdir::WalkDir;
use ignore::gitignore::GitignoreBuilder;
use std::path::{Path, PathBuf};

/// Discover source files, respecting .gitignore
pub fn discover_files(source: &Path) -> Vec<PathBuf> {
	let mut files = Vec::new();
	let mut builder = GitignoreBuilder::new(source);
	builder.add(".gitignore");
	let gitignore = builder.build().unwrap();

	for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
		let path = entry.path();
		if path.is_file() {
			let is_ignored = gitignore.matched(path, false).is_ignore();
			if !is_ignored {
				files.push(path.to_path_buf());
			}
		}
	}
	files
}