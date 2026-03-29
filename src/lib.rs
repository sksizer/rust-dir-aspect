//! dir-aspect: Detect what kind of application folder a directory is.
//!
//! Checks for marker directories (`.git/`, `.obsidian/`, etc.) to determine
//! which "aspects" a directory has. Useful for enabling feature-specific
//! behavior based on the kind of project a directory represents.
//!
//! # Example
//!
//! ```no_run
//! use dir_aspect::{Aspect, detect_aspects};
//!
//! let aspects = detect_aspects(std::path::Path::new("/path/to/dir"));
//! if aspects.contains(&Aspect::Git) {
//!     println!("This is a git repository");
//! }
//! ```

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Detected aspects of a directory.
///
/// Aspects represent what kind of application folder a directory is,
/// determined by the presence of marker directories or files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "lowercase")]
pub enum Aspect {
    /// Directory contains `.obsidian/` folder (Obsidian vault)
    Obsidian,
    /// Directory contains `.git/` folder (Git repository)
    Git,
    /// Default -- plain directory with no special markers
    Generic,
}

/// Detect aspects of a directory by checking for marker directories.
///
/// Always includes [`Aspect::Generic`]. Adds [`Aspect::Obsidian`] if `.obsidian/`
/// is present, and [`Aspect::Git`] if `.git/` is present.
pub fn detect_aspects(path: &Path) -> Vec<Aspect> {
    let mut aspects = vec![Aspect::Generic];

    if path.join(".obsidian").is_dir() {
        aspects.push(Aspect::Obsidian);
    }

    if path.join(".git").is_dir() {
        aspects.push(Aspect::Git);
    }

    aspects
}

/// Look up the Obsidian vault ID for a directory by reading Obsidian's `obsidian.json`.
///
/// Returns `None` if Obsidian's config file doesn't exist, can't be parsed,
/// or doesn't contain a vault matching the given path.
pub fn detect_obsidian_vault_id(path: &Path) -> Option<String> {
    let config_dir = dirs::config_dir()?;
    let obsidian_json = config_dir.join("obsidian").join("obsidian.json");

    let contents = std::fs::read_to_string(&obsidian_json).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&contents).ok()?;

    let vaults = parsed.get("vaults")?.as_object()?;
    let canonical = path.canonicalize().ok()?;

    for (id, info) in vaults {
        if let Some(vault_path_str) = info.get("path").and_then(|v| v.as_str()) {
            if let Ok(vault_canonical) = Path::new(vault_path_str).canonicalize() {
                if vault_canonical == canonical {
                    return Some(id.clone());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("dir_aspect_test_{}", name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create test dir");
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn plain_directory_returns_generic_only() {
        let dir = create_test_dir("plain");
        let aspects = detect_aspects(&dir);
        assert_eq!(aspects, vec![Aspect::Generic]);
        cleanup(&dir);
    }

    #[test]
    fn obsidian_directory_detected() {
        let dir = create_test_dir("obsidian");
        fs::create_dir_all(dir.join(".obsidian")).expect("create .obsidian");
        let aspects = detect_aspects(&dir);
        assert_eq!(aspects, vec![Aspect::Generic, Aspect::Obsidian]);
        cleanup(&dir);
    }

    #[test]
    fn git_directory_detected() {
        let dir = create_test_dir("git");
        fs::create_dir_all(dir.join(".git")).expect("create .git");
        let aspects = detect_aspects(&dir);
        assert_eq!(aspects, vec![Aspect::Generic, Aspect::Git]);
        cleanup(&dir);
    }

    #[test]
    fn both_obsidian_and_git_detected() {
        let dir = create_test_dir("both");
        fs::create_dir_all(dir.join(".obsidian")).expect("create .obsidian");
        fs::create_dir_all(dir.join(".git")).expect("create .git");
        let aspects = detect_aspects(&dir);
        assert_eq!(aspects, vec![Aspect::Generic, Aspect::Obsidian, Aspect::Git,]);
        cleanup(&dir);
    }
}
