use std::{
    env::{self, consts::OS},
    path::PathBuf,
};

use anyhow::Result;

pub fn get_folder() -> Result<PathBuf> {
    if let Ok(home_override) = env::var("COREPACK_HOME") {
        return Ok(PathBuf::from(home_override));
    };

    let cache_home = env::var("XDG_CACHE_HOME").or_else(|_| env::var("LOCALAPPDATA"));

    let cache_home = match cache_home {
        Ok(cache_home) => PathBuf::from(cache_home),
        Err(_) => PathBuf::from(env::var("HOME")?).join(if OS == "windows" {
            "AppData/Local"
        } else {
            ".cache"
        }),
    };

    Ok(cache_home.join("node/corepack"))
}
