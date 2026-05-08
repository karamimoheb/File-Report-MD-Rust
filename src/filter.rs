use std::path::PathBuf;

pub fn filter_entries(
    entries: Vec<(PathBuf, u64)>,
    extension: &str,
    larger_than_bytes: Option<u64>,
    smaller_than_bytes: Option<u64>,
) -> Vec<(PathBuf, u64)> {
    entries
        .into_iter()
        .filter(|(path, size)| {
            // Check extension (case-insensitive)
            match path.extension().and_then(|ext| ext.to_str()) {
                Some(ext) if ext.eq_ignore_ascii_case(extension) => {}
                _ => return false,
            }

            // Check size constraints
            if let Some(min) = larger_than_bytes {
                if *size <= min {
                    return false;
                }
            }
            if let Some(max) = smaller_than_bytes {
                if *size >= max {
                    return false;
                }
            }

            true
        })
        .collect()
}

