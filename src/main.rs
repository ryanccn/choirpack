use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use owo_colors::OwoColorize;

mod cmd;
mod corepack;
mod npm;
mod package_manager;
mod packagejson;

pub use package_manager::PackageManager;

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
        #[arg(long, value_parser = npm::version_value_parser)]
        version: Option<String>,

        /// Path to the Node.js project
        #[arg(long)]
        folder: Option<PathBuf>,

        /// Disable re-installing after using the package manager
        #[arg(long)]
        no_install: bool,
    },

    /// Update the package manager for a Node.js project
    Update {
        /// The version to use for the package manager (defaults to latest)
        #[arg(long, value_parser = npm::version_value_parser)]
        version: Option<String>,

        /// Path to the Node.js project
        #[arg(long)]
        folder: Option<PathBuf>,
    },

    /// Clean the Corepack cache
    Clean {
        /// Timeframe for keeping package managers (ones not used within this timeframe are deleted)
        #[arg(default_value = "7d")]
        duration: String,
    },

    /// Generate shell completions
    Completions {
        /// Shell
        #[arg(value_enum)]
        shell: Shell,
    },
}

pub fn no_bun_for_you() {
    println!("Bun is currently {} by Corepack!", "not supported".red());
    println!(
        "Refer to {} for details.",
        "https://github.com/nodejs/corepack/issues/295".blue()
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = Cmd::parse();

    match cmd.command {
        Commands::Use {
            package_manager,
            version,
            folder,
            no_install,
        } => {
            cmd::use_::action(cmd::use_::Options {
                package_manager,
                version,
                folder,
                no_install,
            })
            .await?;
        }

        Commands::Update { version, folder } => {
            cmd::update::action(cmd::update::Options { version, folder }).await?;
        }

        Commands::Clean { duration } => {
            cmd::clean::action(cmd::clean::Options { duration }).await?;
        }

        Commands::Completions { shell } => {
            let cli = &mut Cmd::command();
            generate(shell, cli, cli.get_name().to_string(), &mut stdout());
        }
    };

    Ok(())
}
