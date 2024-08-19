use std::{env, ffi::OsString, path::Path};

use crate::errors::{AppError, AppResult};

pub fn get_home_path() -> AppResult<OsString> {
    match env::var_os("HOME") {
        Some(home_path) => Ok(home_path),
        None => Err(AppError::InvalidHomePath),
    }
}

pub fn setup_steam_paths() -> AppResult<(String, String)> {
    let home_path = get_home_path()?;
    
    // Define OS-specific base paths
    let base_path = match std::env::consts::OS {
        "macos" => Path::new(&home_path).join("Library/Application Support"),
        "linux" => Path::new(&home_path).join(".local/share"),
        _ => return Err(AppError::UnsupportedPlatform),
    };

    // Define relative paths
    let steam_workshop_path = "Steam/steamapps/workshop/content/107410";
    let steam_game_path = "Steam/steamapps/common/Arma 3";

    // Construct full paths
    let workshop_path = construct_path(&base_path, steam_workshop_path)?;
    let game_path = construct_path(&base_path, steam_game_path)?;

    Ok((workshop_path, game_path))
}

fn construct_path(base_path: &Path, relative_path: &str) -> AppResult<String> {
    let full_path = base_path.join(relative_path);
    full_path
        .to_str()
        .ok_or_else(|| AppError::PathConversionError(full_path.to_string_lossy().into()))
        .map(|s| s.to_string())
}
