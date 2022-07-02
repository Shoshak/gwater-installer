use crate::consts::constants;
use std::path::Path;

pub fn remove_modules(gmod_installation_directory: &Path) {
    for f in constants::GARRYSMOD_MODULES {
        let full_path = gmod_installation_directory.join("garrysmod").join(f);
        if full_path.exists() {
            std::fs::remove_file(&full_path)
                .expect(&format!("Could not remove file at {}", full_path.display()));
        }
    }
    for f in constants::LUA_BIN_MODULES {
        let full_path = gmod_installation_directory
            .join("garrysmod/lua/bin")
            .join(f);
        if full_path.exists() {
            std::fs::remove_file(&full_path)
                .expect(&format!("Could not remove file at {}", full_path.display()));
        }
    }
}
