use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap::Args;
use owo_colors::OwoColorize;
use tokio::process::Command;

use crate::{no_bun_for_you, npm, packagejson, PackageManager};

#[derive(Args, Debug)]
pub struct Options {
    /// The package manager to use
    package_manager: PackageManager,

    /// The version to use for the package manager (defaults to latest)
    #[arg(long, value_parser = npm::version_value_parser)]
    version: Option<String>,

    /// Path to the Node.js project
    #[arg(long)]
    folder: Option<PathBuf>,

    /// Disable re-installing after using the package manager
    #[arg(long)]
    no_install: bool,
}

#[async_trait]
impl super::OptionsWithAction for Options {
    async fn action(&self) -> Result<()> {
        if self.package_manager == PackageManager::Bun {
            no_bun_for_you();
        } else {
            let version = match &self.version {
                Some(version) => version.clone(),
                None => {
                    println!(
                        "Fetching latest version of {}...",
                        self.package_manager.cyan()
                    );
                    npm::fetch_latest(&self.package_manager).await?
                }
            };

            let package_json_path = match &self.folder {
                Some(folder) => folder.join("package.json"),
                None => std::env::current_dir()?.join("package.json"),
            };

            packagejson::patch_package_manager(&package_json_path, &self.package_manager, &version)
                .await?;

            println!(
                "Set package.json to use {}",
                self.package_manager.with_version(&version).green()
            );

            if !self.no_install {
                println!("{}", "Installing dependencies...".magenta());
                println!(
                    "{}",
                    format!("$ {} install", self.package_manager.package_name()).dimmed()
                );

                let mut install_command = Command::new(self.package_manager.package_name());
                install_command.arg("install");

                if let Some(dir) = &self.folder {
                    install_command.current_dir(dir);
                }

                install_command.status().await?;
            }
        }

        Ok(())
    }
}
