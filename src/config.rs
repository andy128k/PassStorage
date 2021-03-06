use crate::error::*;
use glib::get_user_config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{read, write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub search_in_secrets: bool,
    pub show_secrets_on_preview: bool,
}

fn config_path() -> Result<PathBuf> {
    if let Some(mut path) = get_user_config_dir() {
        path.push("password-storage.toml");
        Ok(path)
    } else {
        Err("Path to config is not detected".into())
    }
}

impl Config {
    fn from_file(filename: &Path) -> Result<Self> {
        let buf = read(filename)?;
        let config = toml::from_slice(&buf)?;
        Ok(config)
    }

    pub fn load() -> Self {
        config_path()
            .and_then(|filename| Self::from_file(&filename))
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                Default::default()
            })
    }

    pub fn save(&self) -> Result<()> {
        let filename = config_path()?;
        let dump = toml::to_vec(self)?;
        write(&filename, &dump)?;
        Ok(())
    }
}
