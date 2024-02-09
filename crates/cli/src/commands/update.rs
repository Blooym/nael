use super::RunnableCommand;
use crate::{
    formatting::{emphasis_text, warning_text},
    AppState,
};
use anyhow::{anyhow, Result};
use clap::Parser;
use nael_core::dalamud::{
    management::DalamudInstallation, sources::ReleaseSource, version_info::DalamudVersionInfo,
};

/// Update an existing local Dalamud branch installation to the latest version.
#[derive(Debug, Parser)]
pub struct Update {
    /// The branch to install from.
    branch_name: String,
}

impl RunnableCommand for Update {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(installation) = DalamudInstallation::get(&self.branch_name, &state.storage)?
        else {
            return Err(anyhow!(
                "Branch '{}' is not installed.\nTip: Run '{}' to try and install it.",
                self.branch_name,
                emphasis_text(&format!("nael install {}", self.branch_name))
            ));
        };

        // Just update without version check if no local info is available.
        let version_info = match installation.get_version_info() {
            Ok(version_info) => version_info,
            Err(err) => {
                eprintln!(
                    "{}",
                    warning_text(&format!(
                        "Warning: Failed to obtain version information: {:?}\n",
                        err
                    ))
                );
                None
            }
        };

        let Some(version_info) = version_info else {
            println!("No local information available for branch, skipping up-to-date check and performing update anyway...");
            return match installation.update(&state.release_source).await {
                Err(_) => Err(anyhow!("Failed to update branch '{}'", &self.branch_name,)),
                Ok(_) => {
                    println!(
                        "Updated branch '{}' to the latest remote version.",
                        self.branch_name
                    );
                    Ok(())
                }
            };
        };

        // Check if we're up to date with remote release source.
        let file = state
            .release_source
            .get_version_file_file(&self.branch_name);
        let remote_version_info = DalamudVersionInfo::from_remote_file(&file).await?;

        if version_info == remote_version_info {
            println!("Branch is already up to date.");
            return Ok(());
        }

        match installation.update(&state.release_source).await {
            Err(_) => Err(anyhow!("Failed to update branch '{}'", &self.branch_name,)),
            Ok(_) => {
                println!(
                    "Updated branch '{}' to the latest version.",
                    self.branch_name
                );
                Ok(())
            }
        }
    }
}
