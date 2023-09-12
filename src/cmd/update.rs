use std::path::PathBuf;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{no_bun_for_you, npm, packagejson, PackageManager};

pub struct Options {
    pub version: Option<String>,
    pub folder: Option<PathBuf>,
}

pub async fn action(options: Options) -> Result<()> {
    let folder = options
        .folder
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    let package_manager = packagejson::get_package_manager(&folder).await?;

    if package_manager == PackageManager::Bun {
        no_bun_for_you();
    } else {
        let version = match options.version {
            Some(version) => version,
            None => {
                println!("Fetching latest version of {}", package_manager.cyan());
                npm::fetch_latest(&package_manager).await?
            }
        };

        let package_json_path = folder.join("package.json");

        packagejson::patch_package_manager(&package_json_path, &package_manager, &version).await?;

        println!(
            "Updated package.json to use {}",
            format!("{}@{}", package_manager.to_package_name(), version).green()
        );
    };

    Ok(())
}
