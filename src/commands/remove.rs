use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, logger::info};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

/// Remove a Dalamud version from this system.
#[derive(Debug, Parser)]
pub struct Remove {
    /// The branch to remove.
    branch: String,
}

#[async_trait]
impl RunnableCommand for Remove {
    async fn run(&self) -> Result<()> {
        match DalamudVersionManager::remove(&self.branch) {
            Err(e) => Err(anyhow!("Failed to remove Dalamud@{}: {}", self.branch, e)),
            Ok(_) => {
                info!("Successfully removed Dalamud@{}.", self.branch);
                Ok(())
            }
        }
    }
}
