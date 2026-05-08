use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use url::Url;

/// Heuristic to determine whether a filename is Persian/Arabic script.
fn is_persian_name(name: &str) -> bool {
    name.chars().any(|c| {
        matches!(
            c,
            '\u{0600}'..='\u{06FF}' |
            '\u{0750}'..='\u{077F}' |
            '\u{08A0}'..='\u{08FF}' |
            '\u{FB50}'..='\u{FDFF}' |
            '\u{FE70}'..='\u{FEFF}'
        )
    })
}

pub fn generate_report(entries: &[(PathBuf, u64)], output_path: &Path) -> Result<()> {
    // Group entries by parent directory using a BTreeMap for sorted directory order.
    let mut dir_map: BTreeMap<PathBuf, Vec<(PathBuf, u64)>> = BTreeMap::new();
    for (path, size) in entries {
        let parent = path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("/"));
        dir_map.entry(parent).or_default().push((path.clone(), *size));
    }

    let file = File::create(output_path)
        .with_context(|| format!("Failed to create output file {}", output_path.display()))?;
    let mut writer = BufWriter::new(file);

    let total_count = entries.len();
    let total_size_mb: f64 = entries.iter().map(|(_, s)| *s as f64 / 1_000_000.0).sum();

    writeln!(writer, "# File Report")?;
    writeln!(writer)?;

    if entries.is_empty() {
        writeln!(writer, "No matching files found.")?;
    } else {
        writeln!(
            writer,
            "Found {} matching file(s) in {} directory(ies). Total size: {:.2} MB.\n",
            total_count,
            dir_map.len(),
            total_size_mb
        )?;

        for (dir, files) in dir_map {
            // Directory heading
            writeln!(writer, "## {}", dir.display())?;
            let dir_size_mb: f64 = files.iter().map(|(_, s)| *s as f64 / 1_000_000.0).sum();
            writeln!(writer, "Directory total size: {:.2} MB", dir_size_mb)?;
            writeln!(writer)?;

            // Separate files by name script: English (ASCII) vs Persian (Arabic script)
            let mut eng_files = Vec::new();
            let mut per_files = Vec::new();
            for (path, size) in files {
                let is_eng = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|name| !is_persian_name(name))
                    .unwrap_or(true);
                if is_eng {
                    eng_files.push((path, size));
                } else {
                    per_files.push((path, size));
                }
            }

            // Helper to write a table for a group
            fn write_group_table<W: Write>(
                writer: &mut W,
                group_name: &str,
                files: &[(PathBuf, u64)],
            ) -> Result<()> {
                if files.is_empty() {
                    return Ok(());
                }
                writeln!(writer, "### {} files", group_name)?;
                writeln!(writer)?;
                writeln!(writer, "| # | File | Size (MB) |")?;
                writeln!(writer, "|---|------|-----------|")?;

                let mut sorted = files.to_vec();
                sorted.sort_by(|a, b| b.1.cmp(&a.1)); // descending by size

                for (i, (path, size)) in sorted.iter().enumerate() {
                    let file_url = Url::from_file_path(path)
                        .unwrap_or_else(|_| Url::parse("about:blank").unwrap());
                    let file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    writeln!(
                        writer,
                        "| {} | [{}]({}) | {:.2} |",
                        i + 1,
                        file_name,
                        file_url,
                        *size as f64 / 1_000_000.0
                    )?;
                }
                writeln!(writer)?;
                Ok(())
            }

            write_group_table(&mut writer, "English", &eng_files)?;
            write_group_table(&mut writer, "Persian", &per_files)?;
        }
    }

    writer.flush()?;
    Ok(())
}

