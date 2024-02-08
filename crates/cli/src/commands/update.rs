use super::RunnableCommand;
use crate::{formatting::emphasis_text, AppState};
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
                "Branch '{}' is not installed\nTip: Run '{}' to try and install it",
                self.branch_name,
                emphasis_text(&format!("nael install {}", self.branch_name))
            ));
        };

        let Some(version_info) = installation.get_version_info().unwrap_or(None) else {
            eprintln!("{} did not have version info so it could not be automatically checked to be the latest version", &self.branch_name);
            installation.update(&state.release_source).await?;
            println!(
                "Updated branch {} to the latest version available",
                &self.branch_name
            );
            return Ok(());
        };

        let file = state
            .release_source
            .get_version_file_file(&self.branch_name);
        let remote_version_info = DalamudVersionInfo::from_remote_file(&file).await?;

        if version_info == remote_version_info {
            println!("Already up to date.");
            return Ok(());
        }

        match installation.update(&state.release_source).await {
            Err(e) => Err(anyhow!(
                "Failed to update branch '{}': {}",
                &self.branch_name,
                e
            )),
            Ok(_) => {
                println!("Successfully updated branch '{}'", self.branch_name);
                Ok(())
            }
        }
    }
}
