use std::{
    env, fs,
    path::{Path, PathBuf},
};

use regex::Regex;

use crate::errors::{AppError, AppResult};

use self::{config::Config, paginator::Paginator, terminal::Terminal};

mod config;
mod file_handler;
mod paginator;
mod terminal;

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

            Err(AppError::IoError { .. }) => {
                // Initialize save state
                let home_path = env::var_os("HOME").unwrap();
                let workshop_path = Path::new(&home_path)
                    .join("Library/Application Support/Steam/steamapps/workshop/content/107410")
                    .into_os_string()
                    .into_string()
                    .unwrap();

                let game_path = Path::new(&home_path)
                    .join("Library/Application Support/Steam/steamapps/common/Arma 3")
                    .into_os_string()
                    .into_string()
                    .unwrap();

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
                    let entry = entry.unwrap();
                    let path = entry.path();

                    let metadata = fs::metadata(&path).unwrap();
                    if metadata.is_dir() {
                        let mod_id: u64 =
                            path.file_name().unwrap().to_str().unwrap().parse().unwrap();

                        // If the meta.cpp file is not present, skip
                        let mod_path = Path::new(&path).join("meta.cpp");
                        if !mod_path.exists() {
                            continue;
                        }

                        let mod_content = fs::read(mod_path)
                            .expect(&format!("Unable to read mod.cpp for mod {}", &mod_id));

                        let content_str = String::from_utf8_lossy(&mod_content);

                        let name = Regex::new(r#"name\s*=\s*"([^"]+)""#)
                            .unwrap()
                            .captures(&content_str)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str();

                        mods.push(Mod::new(mod_id, name.to_string()));
                    }
                }
            }
            Err(e) => println!("{}\n{:?}", e, workshop_path),
        }

        Ok(mods)
    }
}
