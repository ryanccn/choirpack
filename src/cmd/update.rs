use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap::Args;
use owo_colors::OwoColorize;

use crate::{no_bun_for_you, npm, packagejson, PackageManager};

#[derive(Args, Debug)]
pub struct Options {
    /// The version to use for the package manager (defaults to latest)
    #[arg(long, value_parser = npm::version_value_parser)]
    version: Option<String>,

    /// Path to the Node.js project
    #[arg(long)]
    folder: Option<PathBuf>,
}

#[async_trait]
impl super::OptionsWithAction for Options {
    async fn action(&self) -> Result<()> {
        let folder = self
            .folder
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap());

        let package_manager = packagejson::get_package_manager(&folder).await?;

        if package_manager == PackageManager::Bun {
            no_bun_for_you();
        } else {
            let version = match &self.version {
                Some(version) => version.to_owned(),
                None => {
                    println!("Fetching latest version of {}...", package_manager.cyan());
                    npm::fetch_latest(&package_manager).await?
                }
            };

            let package_json_path = folder.join("package.json");

            packagejson::patch_package_manager(&package_json_path, &package_manager, &version)
                .await?;

            println!(
                "Updated package.json to use {}",
                format!("{}@{}", package_manager.to_package_name(), version).green()
            );
        };

        Ok(())
    }
}
