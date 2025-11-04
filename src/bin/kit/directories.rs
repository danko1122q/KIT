use std::env;
use std::path::{Path, PathBuf};

use etcetera::BaseStrategy;
use once_cell::sync::Lazy;

/// Wrapper for 'etcetera' that checks KIT_CACHE_PATH and KIT_CONFIG_DIR and falls back to the
/// Windows known folder locations on Windows & the XDG Base Directory Specification everywhere else.
pub struct KitProjectDirs {
    cache_dir: PathBuf,
    config_dir: PathBuf,
}

impl KitProjectDirs {
    fn new() -> Option<KitProjectDirs> {
        let basedirs = etcetera::choose_base_strategy().ok()?;

        // Checks whether or not `$KIT_CACHE_PATH` exists. If it doesn't, set the cache dir to our
        // system's default cache home.
        let cache_dir = if let Some(cache_dir) = env::var_os("KIT_CACHE_PATH").map(PathBuf::from) {
            cache_dir
        } else {
            basedirs.cache_dir().join("kit")
        };

        // Checks whether or not `$KIT_CONFIG_DIR` exists. If it doesn't, set the config dir to our
        // system's default configuration home.
        let config_dir = if let Some(config_dir) = env::var_os("KIT_CONFIG_DIR").map(PathBuf::from)
        {
            config_dir
        } else {
            basedirs.config_dir().join("kit")
        };

        Some(KitProjectDirs {
            cache_dir,
            config_dir,
        })
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
}

pub static PROJECT_DIRS: Lazy<KitProjectDirs> =
    Lazy::new(|| KitProjectDirs::new().expect("Could not get home directory"));
