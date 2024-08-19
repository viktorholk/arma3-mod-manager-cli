use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::{AppError, AppResult};

use super::utils;

const SAVE_FILE: &str = "arma3-mod-manager-cli-config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    game_path: String,
    workshop_path: String,
    enabled_mods: Vec<u64>,
    default_args: String,
}

impl Config {
    fn get_save_path() -> AppResult<PathBuf> {
        let home_path = utils::get_home_path()?;

        Ok(Path::new(&home_path).join(SAVE_FILE))
    }

    pub fn new(game_path: String, workshop_path: String) -> AppResult<Self> {
        let new_config = Config {
            game_path,
            workshop_path,
            enabled_mods: Vec::new(),
            default_args: "-noSplash -skipIntro -world=empty".to_string(),
        };

        new_config.valid()?;

        Ok(new_config)
    }

    fn valid(&self) -> AppResult<()> {
        if !Path::new(&self.workshop_path).exists() {
            return Err(AppError::InvalidPath(self.workshop_path.to_owned()));
        }

        if !Path::new(&self.game_path).exists() {
            return Err(AppError::InvalidPath(self.game_path.to_owned()));
        }

        Ok(())
    }

    pub fn get_enabled_mods(&self) -> Vec<u64> {
        self.enabled_mods.clone()
    }

    pub fn update_mods(&mut self, mods: Vec<u64>) {
        self.enabled_mods = mods;
    }

    pub fn get_game_path(&self) -> &Path {
        Path::new(&self.game_path)
    }

    pub fn get_workshop_path(&self) -> &Path {
        Path::new(&self.workshop_path)
    }

    pub fn get_default_args(&self) -> &str {
        &self.default_args
    }

    pub fn set_default_args(&mut self, args: String) {
        self.default_args = args;
    }

    pub fn save(&self) -> AppResult<()> {
        super::file_handler::write_json(&Config::get_save_path()?, &self)?;
        Ok(())
    }

    pub fn read() -> AppResult<Self> {
        let config: Config = super::file_handler::read_json(&Config::get_save_path()?)?;

        config.valid()?;

        Ok(config)
    }
}
