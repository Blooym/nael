mod active;
mod info;
mod install;
mod list;
mod remove;
mod update;
mod r#use;

pub use {
    active::Active, info::Info, install::Install, list::List, r#use::Use, remove::Remove,
    update::Update,
};

use crate::AppState;
use anyhow::Result;

pub trait RunnableCommand {
    async fn run(&self, state: &AppState) -> Result<()>;
}
