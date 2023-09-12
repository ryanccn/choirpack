use std::io::stdout;

use anyhow::Result;
use async_trait::async_trait;

use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

use crate::Cmd;

#[derive(Args, Debug)]
pub struct Options {
    /// Shell
    #[arg(value_enum)]
    shell: Shell,
}

#[async_trait]
impl super::OptionsWithAction for Options {
    async fn action(&self) -> Result<()> {
        let cli = &mut Cmd::command();
        generate(self.shell, cli, cli.get_name().to_string(), &mut stdout());
        Ok(())
    }
}
