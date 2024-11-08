# filecat

**filecat** is a command-line utility written in Rust that recursively lists files in the current directory and its subdirectories. It allows optional filtering by file extensions, exclusion of specified files or directories, inclusion of hidden files, and outputs the results in JSON format.

## Features

- **Recursive File Listing**: Traverses the current directory and all subdirectories to list files.
- **Extension Filtering**: Include only files with specified extensions.
- **Exclusion of Files/Directories**: Exclude specified files or directories from the listing.
- **Hidden Files**: Optionally include hidden files and directories.
- **JSON Output**: Outputs the file list in JSON format, including path, directory, filename, and extension.

## Installation

### **Build from Source**

1. **Clone the Repository**

   ```bash
   git clone https://github.com/shaneturner/filecat.git
   cd filecat
   ```

2. **Build the Utility**

   ```bash
   cargo build --release
   ```

3. **Install the Binary**

   Optionally, you can move the binary to a directory in your `PATH`:

   ```bash
   sudo mv ./target/release/filecat /usr/local/bin/
   ```

## Usage

```bash
filecat [OPTIONS]
```

### Options

- `-e`, `--extensions <EXTENSIONS>...`: File extensions to include (e.g., `txt`, `rs`). If not specified, all files are included.
- `-x`, `--exclude <EXCLUDE>...`: Files or directories to exclude from the traversal.
- `-o`, `--output <OUTPUT>`: Output JSON file path. Default is `file_list.json`.
- `--include-hidden`: Include hidden files and directories.
- `--help`: Print help information.
- `--version`: Print version information.

### **Examples**

#### **List All Files**

List all files recursively and save the output to `file_list.json`:

```bash
filecat
```

#### **Include Only Specific Extensions**

Include only `.rs` and `.toml` files:

```bash
filecat -e rs toml
```

#### **Exclude Specific Files or Directories**

Exclude the `target` directory and `temp.txt` file:

```bash
filecat -x target temp.txt
```

#### **Include Hidden Files**

Include hidden files and directories in the listing:

```bash
filecat --include-hidden
```

#### **Combine Options**

Include `.rs` files, exclude the `tests` directory, include hidden files, and specify an output file:

```bash
filecat -e rs -x tests --include-hidden -o my_files.json
```

## Output Format

The output is a JSON array of objects, each representing a file with the following fields:

- **`path`**: The relative path to the file.
- **`directory`**: The directory containing the file.
- **`filename`**: The name of the file.
- **`extension`**: The file extension (if any).

### **Sample Output**

```json
[
  {
    "path": "src/main.rs",
    "directory": "src",
    "filename": "main.rs",
    "extension": "rs"
  },
  {
    "path": "Cargo.toml",
    "directory": "",
    "filename": "Cargo.toml",
    "extension": "toml"
  }
]
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/shaneturner/filecat).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

**Shane Turner**

- Website: [shaneturner.dev](https://shaneturner.dev)
