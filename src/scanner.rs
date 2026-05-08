use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

pub fn scan_directory(dir: &Path) -> Result<Vec<(PathBuf, u64)>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(dir).into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Warning: skipping entry: {}", err);
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let metadata = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Warning: cannot read metadata for {}: {}", path.display(), err);
                continue;
            }
        };

        let abs_path = match path.canonicalize() {
            Ok(p) => p,
            Err(err) => {
                eprintln!("Warning: cannot canonicalize {}: {}", path.display(), err);
                continue;
            }
        };

        entries.push((abs_path, metadata.len()));
    }

    Ok(entries)
}

