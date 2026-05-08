use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod filter;
mod report;
mod scanner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the folder to scan
    #[arg(short, long)]
    path: PathBuf,

    /// File extension to filter (without dot, e.g., "txt")
    #[arg(short, long)]
    ext: String,

    /// Only include files larger than this size in MB
    #[arg(long)]
    larger_than: Option<f64>,

    /// Only include files smaller than this size in MB
    #[arg(long)]
    smaller_than: Option<f64>,

    /// Output report file path (Markdown)
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let larger_than_bytes = args.larger_than.map(|mb| (mb * 1_000_000.0) as u64);
    let smaller_than_bytes = args.smaller_than.map(|mb| (mb * 1_000_000.0) as u64);

    let entries = scanner::scan_directory(&args.path)?;
    let filtered = filter::filter_entries(entries, &args.ext, larger_than_bytes, smaller_than_bytes);
    report::generate_report(&filtered, &args.output)?;

    println!("Report written to {}", args.output.display());
    Ok(())
}

