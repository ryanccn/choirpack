use std::{io::stdout, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use owo_colors::OwoColorize;

mod npm;
mod package_manager;
mod packagejson;

pub use package_manager::PackageManager;
use regex::Regex;
use tokio::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cmd {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Use a package manager for a Node.js project
    Use {
        /// The package manager to use
        package_manager: PackageManager,

        /// The version to use for the package manager (defaults to latest)
        #[arg(long, value_parser = version_value_parser)]
        version: Option<String>,

        /// Path to the target package.json to modify
        #[arg(long)]
        package_json: Option<PathBuf>,
    },

    /// Generate shell completions
    Completions {
        /// Shell
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn version_value_parser(str: &str) -> Result<String> {
    let validator = Regex::new("[\\d.]+")?;

    if !validator.is_match(str) {
        return Err(anyhow!("not a valid version"));
    }

    Ok(str.to_owned())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = Cmd::parse();

    match cmd.command {
        Commands::Use {
            package_manager,
            version,
            package_json,
        } => {
            if package_manager == PackageManager::Bun {
                println!("Bun is currently {} by Corepack!", "not supported".red());
                println!(
                    "Refer to {} for details.",
                    "https://github.com/nodejs/corepack/issues/295".blue()
                );
            } else {
                let version = match version {
                    Some(version) => version,
                    None => {
                        println!("Fetching latest version of {}", package_manager.cyan());
                        npm::fetch_latest(&package_manager).await?
                    }
                };

                let package_json_path = package_json
                    .unwrap_or_else(|| std::env::current_dir().unwrap().join("package.json"));

                packagejson::patch_package_manager(&package_json_path, &package_manager, &version)
                    .await?;

                println!(
                    "Set package.json to use {}",
                    format!("{}@{}", package_manager.to_package_name(), version).green()
                );

                println!("{}", "Installing dependencies".magenta());
                println!(
                    "{}",
                    format!("$ {} install", package_manager.to_package_name()).dimmed()
                );

                Command::new(package_manager.to_package_name())
                    .arg("install")
                    .status()
                    .await?;
            }
        }

        Commands::Completions { shell } => {
            let cli = &mut Cmd::command();
            generate(shell, cli, cli.get_name().to_string(), &mut stdout());
        }
    };

    Ok(())
}
