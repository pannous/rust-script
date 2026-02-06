//! Auto-provide external crates for script mode
//!
//! This module contains logic to automatically discover and provide external
//! crates from the sysroot when running in script mode, so users don't need
//! to manually declare dependencies.

use std::collections::BTreeSet;
use rustc_data_structures::fx::FxHashMap;
use rustc_session::config::{ExternEntry, ExternLocation};
use rustc_session::search_paths::{PathKind, SearchPath};
use rustc_session::utils::CanonicalizedPath;
use crate::interface::Config;

/// Auto-provide external crates for script mode.
/// This discovers .rlib files in the sysroot and automatically adds them as extern crates,
/// excluding standard library crates which are already available.
pub fn auto_provide_external_crates(config: &mut Config) {
    if !config.opts.unstable_opts.script {
        return;
    }

    let sysroot = config.opts.sysroot.path();
    let target_triple = config.opts.target_triple.tuple();
    let lib_dir = sysroot.join("lib/rustlib").join(target_triple).join("lib");

    // Stdlib crates that should not be auto-provided (they're already available)
    let stdlib_crates = [
        "std", "core", "alloc", "proc_macro", "test", "panic_abort", "panic_unwind",
        "unwind", "rustc_std_workspace_core", "rustc_std_workspace_alloc",
        "rustc_std_workspace_std", "std_detect", "addr2line", "cfg_if", "compiler_builtins",
        "getopts", "gimli", "hashbrown", "libc", "memchr", "miniz_oxide", "object",
        "rustc_demangle", "sysroot",
    ];

    // Find all external crate rlib files
    if !lib_dir.exists() {
        return;
    }

    let Ok(entries) = std::fs::read_dir(&lib_dir) else { return };

    // Group files by crate name
    let mut crate_files: FxHashMap<String, Vec<_>> = FxHashMap::default();

    for entry in entries.filter_map(|e| e.ok()) {
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else { continue };

        // Match pattern: lib<crate>-<hash>.rlib
        if !name.starts_with("lib") || !name.ends_with(".rlib") {
            continue;
        }

        // Extract crate name (between "lib" and "-")
        let Some(rest) = name.strip_prefix("lib") else { continue };
        let Some(crate_name) = rest.split('-').next() else { continue };

        // Skip stdlib crates
        if stdlib_crates.contains(&crate_name) {
            continue;
        }

        crate_files
            .entry(crate_name.to_string())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    // Add library search path if we found any external crates
    if crate_files.is_empty() {
        return;
    }

    let search_path = SearchPath::new(PathKind::All, lib_dir.clone());
    config.opts.search_paths.push(search_path);

    // Auto-provide each external crate (sorted for determinism)
    // into_iter() is stable here because we sort immediately after
    #[allow(rustc::potential_query_instability)]
    let mut sorted_crates: Vec<_> = crate_files.into_iter().collect();
    sorted_crates.sort_by(|a, b| a.0.cmp(&b.0));

    for (crate_name, files) in sorted_crates {
        if config.opts.externs.contains_key(&crate_name) {
            continue;
        }

        let paths: BTreeSet<_> = files
            .into_iter()
            .map(|e| CanonicalizedPath::new(e.path()))
            .collect();

        if paths.is_empty() {
            continue;
        }

        config.opts.externs.insert(
            crate_name,
            ExternEntry {
                location: ExternLocation::ExactPaths(paths),
                is_private_dep: false,
                add_prelude: true,
                nounused_dep: true,
                force: false,
            },
        );
    }
}
