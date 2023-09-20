use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, logger::info};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

/// Install a new version of Dalamud from the specified branch.
#[derive(Debug, Parser)]
pub struct Install {
    /// The branch to install from.
    branch: String,

    /// Whether to replace an existing installation if it exists.
    #[clap(short, long, default_value = "false")]
    replace: bool,
}

#[async_trait]
impl RunnableCommand for Install {
    async fn run(&self) -> Result<()> {
        match DalamudVersionManager::create(&self.branch, self.replace).await {
            Err(e) => Err(anyhow!("Failed to install Dalamud@{}: {}", self.branch, e)),
            Ok(_) => {
                info!("Successfully installed Dalamud@{}.", self.branch);
                info!(
                    "Run `nael use {}` to select it as the current version.",
                    self.branch
                );
                Ok(())
            }
        }
    }
}
