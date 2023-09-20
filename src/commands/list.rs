use super::RunnableCommand;
use crate::{dalamud_version_manager::DalamudVersionManager, logger::info};
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use colored::Colorize;

/// List all installed versions of Dalamud.
#[derive(Debug, Parser)]
pub struct List {}

#[async_trait]
impl RunnableCommand for List {
    async fn run(&self) -> Result<()> {
        let versions = DalamudVersionManager::get_all_installed()?;
        if versions.is_empty() {
            info!("No versions of Dalamud are installed.");
            return Ok(());
        }

        // let current = DalamudVersionManager::get_current(), if none then return an error with a message
        let current = DalamudVersionManager::get_current()?;
        let msg = format!(
            "Installed versions of Dalamud:\n{}",
            versions
                .iter()
                .map(|v| {
                    if Some(v) == current.as_ref() {
                        format!(" - {}", v.bright_green())
                    } else {
                        format!(" - {}", v)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
        info!("{}", msg);

        Ok(())
    }
}
