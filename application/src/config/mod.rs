mod env;

pub use env::EnvConfig;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn load_core_md(base_dir: &Path) -> Result<String> {
    let path = find_core_md(base_dir)?;
    std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read core.md at {}", path.display()))
}

fn find_core_md(base_dir: &Path) -> Result<PathBuf> {
    let candidate = base_dir.join("core.md");
    if candidate.exists() {
        return Ok(candidate);
    }

    // Walk up to find it (for when run from subdirectory)
    let mut dir = base_dir.to_path_buf();
    while let Some(parent) = dir.parent() {
        let candidate = parent.join("core.md");
        if candidate.exists() {
            return Ok(candidate);
        }
        dir = parent.to_path_buf();
    }

    anyhow::bail!("core.md not found (searched from {})", base_dir.display())
}
