use anyhow::Result;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

pub mod clean;
pub mod completions;
pub mod update;
pub mod use_;

#[cfg(feature = "self_update")]
pub mod self_update;

#[async_trait]
#[enum_dispatch]
pub trait OptionsWithAction {
    async fn action(&self) -> Result<()>;
}
