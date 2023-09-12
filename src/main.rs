use std::io::stdout;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use owo_colors::OwoColorize;

mod cmd;
mod corepack;
mod npm;
mod package_manager;
mod packagejson;

use cmd::OptionsWithAction;
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
    Use(cmd::use_::Options),
    /// Update the package manager for a Node.js project
    Update(cmd::update::Options),
    /// Clean the Corepack cache
    Clean(cmd::clean::Options),

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
        Commands::Use(options) => {
            options.action().await?;
        }
        Commands::Update(options) => {
            options.action().await?;
        }
        Commands::Clean(options) => {
            options.action().await?;
        }

        Commands::Completions { shell } => {
            let cli = &mut Cmd::command();
            generate(shell, cli, cli.get_name().to_string(), &mut stdout());
        }
    };

    Ok(())
}
