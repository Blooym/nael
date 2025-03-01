use super::RunnableCommand;
use crate::{
    AppState,
    formatting::{emphasis_text, warning_text},
};
use anyhow::{Result, anyhow};
use clap::Parser;
use nael_core::{dalamud::DalamudInstallation, fs::storage::AppStorage};

/// Update a local branch to the latest version.
#[derive(Debug, Parser)]
pub struct Update {
    /// The branch to update.
    ///
    /// Leave blank to imply the currently active branch.
    branch_name: Option<String>,

    /// Forcefully update regardless of the current local or remote version information.
    #[clap(
        short = 'f',
        long = "force",
        default_value_t = false,
        conflicts_with = "check"
    )]
    force: bool,

    /// Do not automatically apply updates, only check for them.
    #[clap(
        short = 'c',
        long = "check",
        default_value_t = false,
        conflicts_with = "force"
    )]
    check: bool,
}

impl RunnableCommand for Update {
    async fn run(&self, state: &AppState) -> Result<()> {
        let branch_name = match self.branch_name.clone() {
            Some(branch_name) => Ok(branch_name),
            None => {
                if let Some(active) = DalamudInstallation::get_active(&state.storage)? {
                    println!(
                        "Branch name not specified - inferring from active installation to be {}.",
                        emphasis_text(&active.branch_name)
                    );
                    Ok(active.branch_name)
                } else {
                    Err(anyhow!(
                        "No branch name specified. You must set a branch as active to use the update command without arguments.\nTip: run '{}' to update a specific branch.",
                        emphasis_text("nael update <branch>")
                    ))
                }
            }
        }?;

        let Some(installation) = DalamudInstallation::get(&branch_name, &state.storage)? else {
            return Err(anyhow!(
                "Branch '{}' is not installed.\nTip: run '{}' to try and install it.",
                branch_name,
                emphasis_text(&format!("nael install {}", branch_name))
            ));
        };

        // Handle check for update.
        if self.check {
            if is_up_to_date(&installation, state).await {
                println!("Branch is up to date.")
            } else {
                println!("Branch is out of date.");
            }
            return Ok(());
        }

        // Handle forceful update.
        if self.force {
            println!(
                "Forcefully updating branch '{}' to latest version.",
                branch_name
            );
            return update_branch(&branch_name, installation, state).await;
        }

        // Handle regular update.
        if is_up_to_date(&installation, state).await {
            println!("Branch is already up to date.");
            return Ok(());
        }

        update_branch(&branch_name, installation, state).await
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
    println!("Updated branch to the latest version.");
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
