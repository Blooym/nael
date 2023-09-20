use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, logger::info};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

/// Switch to a different Dalamud version.
#[derive(Debug, Parser)]
pub struct Use {
    /// The branch to use.
    branch: String,
}

#[async_trait]
impl RunnableCommand for Use {
    async fn run(&self) -> Result<()> {
        if !DalamudVersionManager::exists(&self.branch)? {
            return Err(anyhow!(
                "Dalamud@{} does not exist. Run `nael install {}` to try and install it.",
                self.branch,
                self.branch
            ));
        }

        match DalamudVersionManager::set_current(&self.branch) {
            Err(e) => Err(anyhow!("Failed to use Dalamud@{}: {}", self.branch, e)),
            Ok(_) => {
                info!("Successfully switched to Dalamud@{}.", self.branch);
                Ok(())
            }
        }
    }
}
