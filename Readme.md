
# File Report

A command-line tool written in Rust that scans a directory, filters files by extension and optional size constraints, and generates a Markdown report.

## Features

- Recursively scans a folder and its subfolders
- Filters files by extension (without the dot, e.g., `txt`)
- Optional size filters: include only files larger than or smaller than a given size (in MB)
- Outputs a Markdown report containing:
  - Relative path of each matching file
  - File size in megabytes

## Build

Make sure [Rust and Cargo](https://www.rust-lang.org/tools/install) are installed on your system. Then run:




```bash
git clone https://github.com/karamimoheb/File-Report-MD-Rust
cd file-report
cargo build --release
```

The executable will be located at `target/release/file-report`.

## Usage

```bash
file-report [OPTIONS] --path <PATH> --ext <EXT> --output <OUTPUT>
```

### Required arguments

- `-p, --path <PATH>`: Path to the folder to scan  
- `-e, --ext <EXT>`: File extension to filter (without dot, e.g., `txt`)  
- `-o, --output <OUTPUT>`: Output report file path (Markdown format)

### Optional arguments

- `--larger-than <LARGER_THAN>`: Only include files larger than this size (in MB)  
- `--smaller-than <SMALLER_THAN>`: Only include files smaller than this size (in MB)  
- `-h, --help`: Print help information  
- `-V, --version`: Print version information

## Examples

1. Scan `./documents` for text files (`.txt`) and generate `report.md`:

```bash
file-report --path ./documents --ext txt --output report.md
```

2. Find `.log` files larger than 5 MB in the current directory:

```bash
file-report --path . --ext log --larger-than 5 --output large_logs.md
```

3. Find `.jpg` images smaller than 2 MB in `./images`:

```bash
file-report --path ./images --ext jpg --smaller-than 2 --output small_jpgs.md
```

## Contributing

Issues, suggestions, and pull requests are welcome. Please open an issue first for major changes.

