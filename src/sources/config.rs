use std::fs;
use std::path::{Path, PathBuf};

use nix::unistd::Uid;
use serde::{Deserialize, Serialize};

use crate::errors::{RhostmanError, RhostmanResult};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcesConfig {
    #[serde(default)]
    pub sources: Vec<Source>,
}

impl SourcesConfig {
    /// Loads the config at `path`. A missing file is treated as an empty
    /// config, not an error, since "no sources tracked yet" is the normal
    /// starting state.
    pub fn load(path: &Path) -> RhostmanResult<SourcesConfig> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(SourcesConfig::default()),
            Err(err) => Err(err.into()),
        }
    }

    pub fn save(&self, path: &Path) -> RhostmanResult<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let rendered = toml::to_string_pretty(self)?;
        fs::write(path, rendered)?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Source> {
        self.sources.iter().find(|s| s.name == name)
    }

    pub fn add(&mut self, name: &str, url: &str) -> RhostmanResult<()> {
        if self.get(name).is_some() {
            return Err(RhostmanError::SourceExists(name.to_string()));
        }
        self.sources.push(Source {
            name: name.to_string(),
            url: url.to_string(),
        });
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> RhostmanResult<Source> {
        let index = self
            .sources
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| RhostmanError::SourceNotFound(name.to_string()))?;
        Ok(self.sources.remove(index))
    }
}

/// Chooses the tracked-sources config path based on whether the process is
/// running as root: root writes to the system-wide `/etc/rhostman/sources.toml`
/// (since `/etc/hosts` mutation normally requires root, and per-user config
/// dirs resolve to root's own home under `sudo`, not the invoking user's),
/// while a non-root invocation uses the ordinary per-user config dir.
pub fn default_sources_config_path() -> PathBuf {
    if Uid::effective().is_root() {
        PathBuf::from("/etc/rhostman/sources.toml")
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rhostman")
            .join("sources.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_file_returns_empty_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sources.toml");
        let cfg = SourcesConfig::load(&path).unwrap();
        assert_eq!(cfg, SourcesConfig::default());
    }

    #[test]
    fn add_then_save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("sources.toml");

        let mut cfg = SourcesConfig::default();
        cfg.add("mylist", "https://example.com/hosts.txt").unwrap();
        cfg.save(&path).unwrap();

        let loaded = SourcesConfig::load(&path).unwrap();
        assert_eq!(loaded.get("mylist").unwrap().url, "https://example.com/hosts.txt");
    }

    #[test]
    fn add_duplicate_name_errors() {
        let mut cfg = SourcesConfig::default();
        cfg.add("mylist", "https://example.com/a.txt").unwrap();
        let err = cfg.add("mylist", "https://example.com/b.txt").unwrap_err();
        assert!(matches!(err, RhostmanError::SourceExists(name) if name == "mylist"));
    }

    #[test]
    fn remove_missing_name_errors() {
        let mut cfg = SourcesConfig::default();
        let err = cfg.remove("nope").unwrap_err();
        assert!(matches!(err, RhostmanError::SourceNotFound(name) if name == "nope"));
    }

    #[test]
    fn remove_existing_name_returns_it_and_drops_it() {
        let mut cfg = SourcesConfig::default();
        cfg.add("mylist", "https://example.com/a.txt").unwrap();
        let removed = cfg.remove("mylist").unwrap();
        assert_eq!(removed.name, "mylist");
        assert!(cfg.get("mylist").is_none());
    }
}
