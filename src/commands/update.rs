use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, logger::info};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

/// Update an existing Dalamud branch installation to the latest version.
#[derive(Debug, Parser)]
pub struct Update {
    /// The branch to install from.
    branch: String,
}

#[async_trait]
impl RunnableCommand for Update {
    async fn run(&self) -> Result<()> {
        if !DalamudVersionManager::exists(&self.branch)? {
            return Err(anyhow!(
                "Dalamud@{} is not installed. Run `nael install {}` to install it.",
                self.branch,
                self.branch
            ));
        }

        match DalamudVersionManager::create(&self.branch, true).await {
            Err(e) => Err(anyhow!("Failed to update Dalamud@{}: {}", self.branch, e)),
            Ok(_) => {
                info!("Successfully updated Dalamud@{}.", self.branch);
                Ok(())
            }
        }
    }
}
