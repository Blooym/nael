use super::RunnableCommand;
use crate::{formatting::emphasis_text, AppState};
use anyhow::{anyhow, Result};
use clap::Parser;
use nael_core::dalamud::management::DalamudInstallation;

/// Install a Dalamud release from the specified branch.
#[derive(Debug, Parser)]
pub struct Install {
    /// The name of the branch to fetch the release from.
    branch_name: String,
}

impl RunnableCommand for Install {
    async fn run(&self, state: &AppState) -> Result<()> {
        match DalamudInstallation::create(&self.branch_name, &state.storage, &state.release_source)
            .await
        {
            Ok(installation) => {
                println!(
                    "Successfully installed branch '{}' with Dalamud assembly version '{}'",
                    &installation.branch_name,
                    &installation
                        .get_version_info()
                        .unwrap_or(None)
                        .map_or("unknown".to_owned(), |v| v.assembly_version),
                );
                println!(
                    "Tip: run `{}` to select it as the active branch.",
                    emphasis_text(&format!("nael use {}", self.branch_name))
                );
                Ok(())
            }
            Err(e) => Err(anyhow!(
                "Failed to install the branch '{}': {}",
                self.branch_name,
                e
            )),
        }
    }
}
