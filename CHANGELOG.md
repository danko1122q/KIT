# Changelog

All notable changes to Kit will be documented in this file.

## v1.0.0 - 2025-01-04

### 🎉 Initial Release

**Author**: danko1122q <davaniko1122@gmail.com>

First stable release of **Kit** - a versatile file utility with powerful features.

### ✨ Features

- **Syntax Highlighting**: Beautiful syntax highlighting for 200+ programming and markup languages
- **File Viewing**: Advanced file viewing with line numbers, Git integration, and customizable themes
- **File Creation**: Create files with `-c` / `--create` flag (similar to touch command)
- **Directory Creation**: Create directories recursively with `--mkdir` flag (like mkdir -p)
- **Git Integration**: Shows file modifications and changes in the sidebar
- **Automatic Paging**: Smart paging for large files with customizable pager support
- **Theme Support**: Multiple color themes for different terminal backgrounds
- **Custom Syntax Mappings**: Map file patterns to specific syntaxes
- **Configuration Files**: Full configuration file support for persistent settings

### 🎨 Customization

- Environment variables for easy configuration:
  - `KIT_THEME`: Set default theme
  - `KIT_STYLE`: Set default style components  
  - `KIT_PAGER`: Set default pager
  - `KIT_PAGING`: Control paging behavior
  - `KIT_TABS`: Set tab width
  - `KIT_CONFIG_PATH`: Custom config file location
  - `KIT_CACHE_PATH`: Custom cache directory

- Configuration directory: `~/.config/kit`
- Cache directory: `~/.cache/kit`

### 📝 Copyright

Copyright © 2025 danko1122q. All rights reserved.

### 📦 Technical Details

- Language: Rust
- Edition: 2021
- Minimum Rust Version: 1.70
- License: MIT OR Apache-2.0

### 🙏 Acknowledgments

Built with Rust and powered by syntect for syntax highlighting.
