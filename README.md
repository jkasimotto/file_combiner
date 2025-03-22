# File Combiner

A teeny-tiny CLI tool to select and combine multiple files from your codebase into a single text file for AI agents or other purposes.

## Features

- **Regex Pattern Matching**: Select files using regex patterns
- **Interactive Selection**: Choose files visually using arrow keys and checkboxes
- **Directory Filtering**: Limit search to specific directories
- **Combined Selection**: Pre-filter with regex, then make final choices interactively
- **Path Preservation**: Each file in the output is prefixed with its original path as a comment
- **Color-coded Output**: Easy-to-read terminal messages

## Installation

### Building from Source

```bash
# Clone the repository (or create from scratch)
git clone https://github.com/yourusername/file_combiner.git
cd file_combiner

# Build the project
cargo build --release

# The executable will be in target/release/file_combiner
```

### Adding to PATH

**Option 1: Create a symbolic link (recommended)**

```bash
ln -s "$(pwd)/target/release/file_combiner" /usr/local/bin/file_combiner
```

**Option 2: Add to your PATH in shell configuration**

For Bash:
```bash
echo 'export PATH="$PATH:$(pwd)/target/release"' >> ~/.bash_profile
source ~/.bash_profile
```

For Zsh:
```bash
echo 'export PATH="$PATH:$(pwd)/target/release"' >> ~/.zshrc
source ~/.zshrc
```

**Option 3: Copy to an existing PATH location**

```bash
cp "$(pwd)/target/release/file_combiner" /usr/local/bin/
chmod +x /usr/local/bin/file_combiner
```

## Usage

### Selecting Files with Regex Pattern

```bash
file_combiner -r "\.rs$" -o combined.txt
```
This selects all files with a `.rs` extension and combines them into `combined.txt`.

### Interactive Selection

```bash
file_combiner -i -o combined.txt
```
This shows a checkbox list of all files for you to select with arrow keys.

### Limiting to Specific Directories

```bash
file_combiner -r "\.rs$" -d src,tests -o combined.txt
```
This only searches in the `/src` and `/tests` directories.

### Combining Approaches

```bash
file_combiner -r "\.rs$" -i -o combined.txt
```
This pre-filters files with the regex pattern, then lets you make the final selection interactively.

## Command Line Options

| Option | Description |
|--------|-------------|
| `-r, --regex <PATTERN>` | Use regex pattern to select files |
| `-i, --interactive` | Use interactive selection |
| `-o, --output <FILE>` | Output file path (default: "combined.txt") |
| `-d, --dirs <DIRS>` | Limit search to specific directories (comma separated) |
| `--help` | Print help information |
| `--version` | Print version information |

## Output Format

The combined output file will contain each selected file with a header comment:

```
// ===== FILE: src/main.rs =====

[content of main.rs]

// ===== FILE: src/lib.rs =====

[content of lib.rs]
```

## License

MIT