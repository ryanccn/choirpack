use anyhow::Result;
use async_trait::async_trait;

pub mod clean;
pub mod update;
pub mod use_;

#[async_trait]
pub trait OptionsWithAction {
    async fn action(&self) -> Result<()>;
}
