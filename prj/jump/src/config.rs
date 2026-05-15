use std::fs;
use std::path::PathBuf;

use crate::{Error, Result};

#[derive(Default, serde::Deserialize)]
pub struct Config {
    pub default_target: Option<String>,
}

impl Config {
    pub fn from_dirs(dirs: &[PathBuf]) -> Result<Self> {
        dirs.iter().try_fold(Config::default(), |config, dir| {
            let file = dir.join("config.yaml");
            let Ok(yaml) = fs::read_to_string(&file) else {
                // Fine, this directory contains no (readable) config file.
                return Ok(config);
            };
            serde_saphyr::from_str(&yaml)
                .map(|c| config.merge(c))
                .map_err(|e| Error::Config(file, Box::new(e)))
        })
    }

    #[must_use]
    fn merge(self, that: Config) -> Self {
        let default_target = that.default_target.or(self.default_target);
        Self { default_target }
    }
}
