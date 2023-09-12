use std::path::PathBuf;

use anyhow::Result;
use owo_colors::OwoColorize;
use tokio::process::Command;

use crate::{no_bun_for_you, npm, packagejson, PackageManager};

pub struct Options {
    pub package_manager: PackageManager,
    pub version: Option<String>,
    pub folder: Option<PathBuf>,
    pub no_install: bool,
}

pub async fn action(options: Options) -> Result<()> {
    if options.package_manager == PackageManager::Bun {
        no_bun_for_you();
    } else {
        let version = match options.version {
            Some(version) => version,
            None => {
                println!(
                    "Fetching latest version of {}",
                    options.package_manager.cyan()
                );
                npm::fetch_latest(&options.package_manager).await?
            }
        };

        let package_json_path = match options.folder {
            Some(folder) => folder.join("package.json"),
            None => std::env::current_dir()?.join("package.json"),
        };

        packagejson::patch_package_manager(&package_json_path, &options.package_manager, &version)
            .await?;

        println!(
            "Set package.json to use {}",
            format!("{}@{}", options.package_manager.to_package_name(), version).green()
        );

        if !options.no_install {
            println!("{}", "Installing dependencies".magenta());
            println!(
                "{}",
                format!("$ {} install", options.package_manager.to_package_name()).dimmed()
            );

            Command::new(options.package_manager.to_package_name())
                .arg("install")
                .status()
                .await?;
        }
    }

    Ok(())
}
