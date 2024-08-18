use errors::AppResult;

use mod_manager::ModManager;

mod errors;
mod mod_manager;

fn main() -> AppResult<()> {
    let mut manager = ModManager::new(15)?;

    manager.start()?;

    Ok(())
}
