mod current;
mod install;
mod list;
mod remove;
mod update;
mod r#use;

use anyhow::Result;
use async_trait::async_trait;
pub use {
    current::Current, install::Install, list::List, r#use::Use, remove::Remove, update::Update,
};

#[async_trait]
/// A command that can be run.
pub trait RunnableCommand {
    /// Run the command.
    async fn run(&self) -> Result<()>;
}
