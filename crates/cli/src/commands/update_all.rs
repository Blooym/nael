use super::RunnableCommand;
use crate::{AppState, formatting::warning_text};
use anyhow::{Result, anyhow};
use clap::Parser;
use nael_core::{dalamud::DalamudInstallation, fs::storage::AppStorage};

/// Update all local branches to the latest version.
#[derive(Debug, Parser)]
pub struct UpdateAll;

impl RunnableCommand for UpdateAll {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(installations) = DalamudInstallation::get_all(&state.storage)? else {
            return Err(anyhow!("No dalamud installations detected."));
        };

        let mut has_updated = false;
        for install in installations {
            if is_up_to_date(&install, state).await {
                continue;
            }

            update_branch(&install.branch_name, install.clone(), state).await?;
            println!("Updated {} successfully", &install.branch_name);
            has_updated = true;
        }

        if has_updated {
            println!("One or more installations were successfully updated");
        } else {
            println!("All installations are up to date.")
        }
        Ok(())
    }
}

/// Handle updating the given installation to the latest version and printing messages to Stdout and Stderr accordingly.
async fn update_branch<S: AppStorage>(
    branch_name: &str,
    installation: DalamudInstallation<S>,
    state: &AppState,
) -> Result<()> {
    if installation.update(&state.release_source).await.is_err() {
        return Err(anyhow!("Failed to update branch '{}'", &branch_name));
    }
    Ok(())
}

/// Check for whether or not the given installation/branch is up to date or not.
///
/// When any part of the checking for remote/local version information fails, this function will
/// output a warning to Stderr and indicate the release is out of date.
async fn is_up_to_date<S: AppStorage>(
    installation: &DalamudInstallation<S>,
    state: &AppState,
) -> bool {
    let version_info = match installation.get_version_info() {
        Ok(version_info) => version_info,
        Err(err) => {
            eprintln!(
                "{}",
                warning_text(&format!(
                    "Warning: Failed to obtain version information: {err:?}\n"
                ))
            );
            None
        }
    };

    let remote_version_info = match installation
        .get_remote_version_info(&state.release_source)
        .await
    {
        Ok(remote_version_info) => remote_version_info,
        Err(err) => {
            eprintln!(
                "{}",
                warning_text(&format!(
                    "Warning: Failed to obtain remote version information: {err:?}\n"
                ))
            );
            None
        }
    };

    let Some(version_info) = version_info else {
        println!(
            "No local version information was found for branch, it will be assumed out of date..."
        );
        return false;
    };

    let Some(remote_version_info) = remote_version_info else {
        println!(
            "No remote version information was found for branch, it will be assumed out of date..."
        );
        return false;
    };

    version_info == remote_version_info
}
