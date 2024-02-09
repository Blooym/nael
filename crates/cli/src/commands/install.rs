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
        match DalamudInstallation::create(&self.branch_name, &state.storage, &state.release_source)
            .await
        {
            Ok(branch) => {
                if let Some(version_info) = branch.get_version_info()? {
                    println!(
                        "Successfully installed branch '{}' with version '{}'.",
                        &branch.branch_name, &version_info.version
                    );
                } else {
                    println!("Successfully installed branch '{}'", &branch.branch_name);
                }
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
