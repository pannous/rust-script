//! Script mode utilities for our custom Rust fork
//!
//! This module contains helper functions for supporting script mode,
//! including shebang detection and output path generation.

use std::fs;
use std::path::PathBuf;
use rustc_session::config::Input;

/// Check if the input file has a shebang line (used for script mode auto-run).
/// A shebang is `#!` at the start, but NOT `#![` which is a Rust attribute.
pub fn has_shebang(input: &Input) -> bool {
    match input {
        Input::File(path) => {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(rest) = content.strip_prefix("#!") {
                    rest.chars().next() != Some('[')
                } else {
                    false
                }
            } else {
                false
            }
        }
        Input::Str { input, .. } => {
            if let Some(rest) = input.strip_prefix("#!") {
                rest.chars().next() != Some('[')
            } else {
                false
            }
        }
    }
}

/// Get the default output executable path for script mode.
/// Returns a path relative to current directory (prefixed with ./) so Command can find it.
pub fn get_script_output_path(input: &Input) -> Option<PathBuf> {
    match input {
        Input::File(path) => {
            let stem = path.file_stem()?;
            // Prefix with "./" so Command::new finds it in current directory
            Some(PathBuf::from(".").join(stem))
        }
        Input::Str { .. } => None,
    }
}
