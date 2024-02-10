use super::RunnableCommand;
use crate::AppState;
use anyhow::{Context, Result};
use clap::Parser;
use nael_core::fs::storage::AppStorage;

/// Get the path to the symlink that always points to the active branch.
#[derive(Debug, Clone, Parser)]
pub struct SymlinkPath;

impl RunnableCommand for SymlinkPath {
    async fn run(&self, state: &AppState) -> Result<()> {
        println!(
            "{}",
            state
                .storage
                .get_active_branch_symlink()
                .context("could not find active branch path")?
                .to_str()
                .context("could not convert path to string for output")?
        );

        Ok(())
    }
}
