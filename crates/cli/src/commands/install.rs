use super::RunnableCommand;
use crate::{formatting::emphasis_text, AppState};
use anyhow::{anyhow, Result};
use clap::Parser;
use nael_core::dalamud::management::DalamudInstallation;

/// Install a Dalamud release from the specified branch.
#[derive(Debug, Parser)]
pub struct Install {
    /// The name of the branch to install.
    branch_name: String,
}

impl RunnableCommand for Install {
    async fn run(&self, state: &AppState) -> Result<()> {
        if DalamudInstallation::exists(&self.branch_name, &state.storage)? {
            return Err(anyhow!(
                "Branch '{}' is already installed.\nTip: run '{}' to update it.",
                self.branch_name,
                emphasis_text(&format!("nael update {}", self.branch_name))
            ));
        }

        match DalamudInstallation::create(&self.branch_name, &state.storage, &state.release_source)
            .await
        {
            Ok(installation) => {
                if let Some(version_info) = installation.get_version_info()? {
                    println!(
                        "Successfully installed branch '{}' with version '{}'.",
                        &installation.branch_name, &version_info.assembly_version
                    );
                } else {
                    println!(
                        "Successfully installed branch '{}'",
                        &installation.branch_name
                    );
                }
                println!(
                    "Tip: run `{}` to select it as the active branch.",
                    emphasis_text(&format!("nael use {}", self.branch_name))
                );
                Ok(())
            }
            Err(_) => Err(anyhow!(
                "Failed to install the branch '{}'",
                self.branch_name
            )),
        }
    }
}
