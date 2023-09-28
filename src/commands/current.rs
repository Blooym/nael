use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, fs::directory, logger::info};
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
                    let current_dir = directory::get_current_version_dir();
                    info!(
                        "Currently using Dalamud@{} linked at {}",
                        ver,
                        current_dir.unwrap().display()
                    );
                } else {
                    info!("No version of Dalamud is currently in use.");
                }
                Ok(())
            }
        }
    }
}
