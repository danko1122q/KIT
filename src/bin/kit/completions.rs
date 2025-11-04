use std::env;
use std::fs;

/// Returns the content of the Bash completion script
pub fn bash_completion() -> String {
    let path = env::var("KIT_GENERATED_COMPLETION_BASH")
        .unwrap_or_else(|_| "completions/bash_default".into());
    fs::read_to_string(path).unwrap_or_default()
}

/// Returns the content of the Fish completion script
pub fn fish_completion() -> String {
    let path = env::var("KIT_GENERATED_COMPLETION_FISH")
        .unwrap_or_else(|_| "completions/fish_default".into());
    fs::read_to_string(path).unwrap_or_default()
}

/// Returns the content of the PS1 completion script
pub fn ps1_completion() -> String {
    let path = env::var("KIT_GENERATED_COMPLETION_PS1")
        .unwrap_or_else(|_| "completions/ps1_default".into());
    fs::read_to_string(path).unwrap_or_default()
}

/// Returns the content of the Zsh completion script
pub fn zsh_completion() -> String {
    let path = env::var("KIT_GENERATED_COMPLETION_ZSH")
        .unwrap_or_else(|_| "completions/zsh_default".into());
    fs::read_to_string(path).unwrap_or_default()
}
