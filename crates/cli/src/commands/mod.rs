mod active;
mod info;
mod install;
mod list;
mod remove;
mod symlink_path;
mod update;
mod update_all;
mod r#use;

pub use {
    active::Active, info::Info, install::Install, list::List, r#use::Use, remove::Remove,
    symlink_path::SymlinkPath, update::Update, update_all::UpdateAll,
};

use crate::AppState;
use anyhow::Result;

pub trait RunnableCommand {
    async fn run(&self, state: &AppState) -> Result<()>;
}
