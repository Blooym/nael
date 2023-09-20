use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, logger::info};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

/// Output the currently in-use Dalamud version.
#[derive(Debug, Parser)]
pub struct Current {}

#[async_trait]
impl RunnableCommand for Current {
    async fn run(&self) -> Result<()> {
        match DalamudVersionManager::get_current() {
            Err(e) => Err(anyhow!("Failed to get current Dalamud version: {}", e)),
            Ok(ver) => {
                if let Some(ver) = ver {
                    info!("Currently using Dalamud@{}.", ver);
                } else {
                    info!("No version of Dalamud is currently in use.");
                }
                Ok(())
            }
        }
    }
}
