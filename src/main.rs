use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;
use walkdir::WalkDir;

/// A CLI utility to list files recursively with optional extension filtering.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File extensions to include (e.g., txt, rs). If not specified, all files are included.
    #[arg(short = 'e', long, num_args(1..))]
    extensions: Vec<String>,

    /// Files or directories to exclude from the traversal
    #[arg(short = 'x', long, num_args(1..))]
    exclude: Vec<String>,

    /// Output JSON file path
    #[arg(short, long, default_value = "file_list.json")]
    output: String,

    /// Include hidden files and directories
    #[arg(long)]
    include_hidden: bool,
}

#[derive(Serialize)]
struct FileEntry {
    path: String,
    directory: String,
    filename: String,
    extension: Option<String>,
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    if entry.depth() == 0 {
        false // Root directory is not hidden
    } else {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }
}

fn should_skip(entry: &walkdir::DirEntry, exclude_paths: &Vec<PathBuf>) -> bool {
    let relative_path = entry.path().strip_prefix(".").unwrap_or(entry.path());

    for exclude_path in exclude_paths {
        let exclude_path = exclude_path.strip_prefix(".").unwrap_or(exclude_path);

        if relative_path.starts_with(exclude_path) {
            return true;
        }
    }
    false
}

fn main() {
    let args = Args::parse();

    // Collect the extensions into a HashSet for efficient lookup
    let extensions: Option<std::collections::HashSet<_>> = if !args.extensions.is_empty() {
        Some(
            args.extensions
                .iter()
                .map(|ext| ext.to_lowercase())
                .collect(),
        )
    } else {
        None
    };

    // Collect the exclude paths into a Vec of PathBufs
    let exclude_paths: Vec<std::path::PathBuf> = args
        .exclude
        .iter()
        .map(|p| std::path::Path::new(p).to_owned())
        .collect();

    let mut file_list = Vec::new();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            // Exclude hidden files/directories if not including hidden
            let include_entry = if args.include_hidden {
                true
            } else {
                !is_hidden(e)
            };

            // Exclude specified files/directories
            let exclude_entry = should_skip(e, &exclude_paths);

            include_entry && !exclude_entry
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if let Some(ref exts) = extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !exts.contains(&ext.to_lowercase()) {
                    continue;
                }
            } else {
                continue;
            }
        }

        let relative_path = path.strip_prefix(".").unwrap_or(path).to_owned();

        let directory = relative_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new(""))
            .to_string_lossy()
            .into_owned();

        let filename = relative_path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new(""))
            .to_string_lossy()
            .into_owned();

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_string());

        file_list.push(FileEntry {
            path: relative_path.to_string_lossy().into_owned(),
            directory,
            filename,
            extension,
        });
    }

    // Serialize the file list to JSON
    match serde_json::to_string_pretty(&file_list) {
        Ok(json) => {
            let json_with_newline = format!("{}\n", json);
            std::fs::write(&args.output, json_with_newline).expect("Failed to write JSON output");
            println!("File list written to: {}", args.output);
        }
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}
