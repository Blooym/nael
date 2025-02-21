use super::RunnableCommand;
use crate::AppState;
use anyhow::{Result, anyhow};
use clap::Parser;
use nael_core::dalamud::DalamudInstallation;

/// Remove a branch from this system.
#[derive(Debug, Parser)]
pub struct Remove {
    /// The name of the branch to remove.
    branch_name: String,
}

impl RunnableCommand for Remove {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(installation) = DalamudInstallation::get(&self.branch_name, &state.storage)?
        else {
            return Err(anyhow!("Branch '{}' is not installed", &self.branch_name));
        };

        installation.remove()?;
        println!("Successfully removed the branch '{}'.", self.branch_name);
        Ok(())
    }
}
