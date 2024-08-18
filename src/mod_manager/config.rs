use std::
    path::{Path, PathBuf}
;

use serde::{Deserialize, Serialize};

use crate::errors::{AppError, AppResult};

const SAVE_PATH: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    game_path: String,
    workshop_path: String,
    enabled_mods: Vec<u64>,
}

impl Config {
    pub fn new(game_path: String, workshop_path: String) -> AppResult<Self> {
        let new_config = Config {
            game_path,
            workshop_path,
            enabled_mods: Vec::new(),
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

    pub fn save(&self) -> AppResult<()> {
        super::file_handler::write_json(&PathBuf::from(SAVE_PATH), &self)?;
        Ok(())
    }

    pub fn read() -> AppResult<Self> {
        let config: Config = super::file_handler::read_json(&PathBuf::from(SAVE_PATH))?;

        config.valid()?;

        Ok(config)
    }
}
