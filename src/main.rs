#![warn(clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
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
#[enum_dispatch(OptionsWithAction)]
enum Commands {
    /// Use a package manager for a Node.js project
    Use(cmd::use_::Options),
    /// Update the package manager for a Node.js project
    Update(cmd::update::Options),
    /// Clean the Corepack cache
    Clean(cmd::clean::Options),
    /// Generate shell completions
    Completions(cmd::completions::Options),
    /// Update the CLI itself from GitHub Releases
    #[cfg(feature = "self_update")]
    UpdateSelf(cmd::self_update::Options),
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
    cmd.command.action().await?;
    Ok(())
}
