use std::{
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

use crate::errors::{AppError, AppResult};

use self::{config::Config, paginator::Paginator, terminal::Terminal};

mod config;
mod file_handler;
mod paginator;
mod terminal;
mod utils;

#[derive(Debug, Clone)]
pub struct Mod {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

impl Mod {
    fn new(id: u64, name: String) -> Mod {
        Mod {
            id,
            name,
            enabled: false,
        }
    }

    pub fn get_path(&self, path: &Path) -> PathBuf {
        path.join(self.id.to_string())
    }
}

#[derive(Debug)]
pub struct ModManager {
    config: Config,
    loaded_mods: Paginator<Mod>,
}

impl ModManager {
    pub fn new(page_size: usize) -> AppResult<Self> {
        match Config::read() {
            Ok(config) => {
                let mut loaded_mods = ModManager::get_installed_mods(config.get_workshop_path())?;

                for i_mod in &mut loaded_mods {
                    if config
                        .get_enabled_mods()
                        .iter()
                        .find(|&&e_mod_id| e_mod_id == i_mod.id)
                        .is_some()
                    {
                        i_mod.enabled = true;
                    }
                }

                Ok(ModManager {
                    config,
                    loaded_mods: Paginator::new(loaded_mods, page_size),
                })
            }

            Err(AppError::IoError(io_error)) if io_error.kind() == std::io::ErrorKind::NotFound => {
                let (workshop_path, game_path) = utils::setup_steam_paths()?;

                let config = Config::new(game_path, workshop_path)?;

                let loaded_mods = ModManager::get_installed_mods(config.get_workshop_path())?;

                Ok(ModManager {
                    config,
                    loaded_mods: Paginator::new(loaded_mods, page_size),
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn start(&mut self) -> AppResult<()> {
        let mut term = Terminal::new(self);

        term.run()?;

        Ok(())
    }

    pub fn refresh_mods(&mut self) -> AppResult<()> {
        let installed_mods = ModManager::get_installed_mods(self.config.get_workshop_path())?;
        self.loaded_mods = Paginator::new(installed_mods, self.loaded_mods.page_size);

        Ok(())
    }

    fn get_installed_mods(workshop_path: &Path) -> AppResult<Vec<Mod>> {
        let mut mods: Vec<Mod> = Vec::new();

        match fs::read_dir(&workshop_path) {
            Ok(installed_mods) => {
                for entry in installed_mods {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(_) => continue,
                    };

                    let path = entry.path();

                    if !path.is_dir() {
                        continue;
                    }

                    let mod_id: u64 = match path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .and_then(|s| s.parse().ok())
                    {
                        Some(id) => id,
                        None => continue,
                    };

                    // If the meta.cpp file is not present, skip
                    let mod_path = Path::new(&path).join("meta.cpp");
                    if !mod_path.exists() {
                        continue;
                    }

                    let mod_content =
                        fs::read(&mod_path).map_err(|_| AppError::MissingMeta(mod_id))?;

                    let content_str = String::from_utf8_lossy(&mod_content);

                    let mut name = match Regex::new(r#"name\s*=\s*"([^"]+)""#)
                        .unwrap()
                        .captures(&content_str)
                        .and_then(|caps| caps.get(1))
                        .map(|m| m.as_str().to_string())
                    {
                        Some(name) => name,
                        None => continue,
                    };

                    // Uppercase the first letter of the name
                    let mut chars = name.chars();
                    name = match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    };

                    mods.push(Mod::new(mod_id, name.to_string()));
                }
            }
            Err(e) => println!("{}\n{:?}", e, workshop_path),
        }

        mods.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(mods)
    }
}
