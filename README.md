# Kit - A Versatile File Utility

**Kit** is a versatile command-line utility that combines file viewing with syntax highlighting, Git integration, and file/directory creation capabilities.

## Author

**danko1122q** <davaniko1122@gmail.com>

Copyright © 2025 danko1122q. All rights reserved.

## Features

### 📄 File Viewing with Syntax Highlighting
View files with beautiful syntax highlighting for numerous programming and markup languages.

### 🔧 File & Directory Creation
- **Create files**: `kit -c <file>` or `kit --create <file>`
- **Create directories**: `kit --mkdir <dir>` (creates recursively like `mkdir -p`)

### 🎨 Git Integration
Shows Git modifications in the sidebar when viewing files.

### 🖥️ Automatic Paging
Smart paging for large files with support for customizable pagers.

## Usage

### View Files
```bash
# View a single file
kit README.md

# View multiple files
kit src/*.rs

# Read from stdin
curl -s https://example.com | kit
```

### Create Files
```bash
# Create a single file
kit -c test.txt

# Create multiple files
kit -c file1.txt file2.md file3.rs
```

### Create Directories
```bash
# Create a directory
kit --mkdir mydir

# Create nested directories (like mkdir -p)
kit --mkdir path/to/nested/directory

# Create multiple directories
kit --mkdir dir1 dir2 dir3
```

### Additional Options
```bash
# List all supported languages
kit --list-languages

# List all themes
kit --list-themes

# Show line numbers
kit -n file.txt

# Set a specific theme
kit --theme=TwoDark file.rs

# Show all non-printable characters
kit -A file.txt
```

## Configuration

Kit can be configured using environment variables:

- `KIT_THEME`: Set the default theme
- `KIT_STYLE`: Set the default style components
- `KIT_PAGER`: Set the default pager
- `KIT_PAGING`: Control paging behavior
- `KIT_TABS`: Set tab width
- `KIT_CONFIG_PATH`: Path to config file
- `KIT_CACHE_PATH`: Path to cache directory

## Building from Source

```bash
# Build release version
cargo build --release

# Binary will be located at
./target/release/kit
```

## License

MIT OR Apache-2.0

## Repository

https://github.com/danko1122q/kit
