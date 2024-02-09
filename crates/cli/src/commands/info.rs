use super::RunnableCommand;
use crate::{formatting::emphasis_text, AppState};
use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use nael_core::dalamud::{
    management::DalamudInstallation, sources::ReleaseSource, version_info::DalamudVersionInfo,
};
use std::str::FromStr;

/// Show information about the specified branch.
#[derive(Debug, Parser)]
pub struct Info {
    /// The name of the branch to show information for.
    branch_name: String,

    /// Output the information as JSON.
    #[clap(short = 'j', long = "json", default_value_t = false)]
    json: bool,

    /// Output information about a remote branch instead of the local one.
    #[clap(short = 'r', long = "remote", default_value_t = false)]
    remote: bool,
}

impl RunnableCommand for Info {
    async fn run(&self, state: &AppState) -> Result<()> {
        if self.remote {
            get_release_info_remote(self, state).await
        } else {
            get_release_info_local(self, state)
        }
    }
}

/// Get release information for a local branch.
fn get_release_info_local(args: &Info, state: &AppState) -> Result<()> {
    // Get the local Dalamud installation.
    let Some(installation) = DalamudInstallation::get(&args.branch_name, &state.storage)? else {
        return Err(anyhow!(
            "The branch '{}' is not installed locally.\nTip: You can use the '{}' flag to see remote information instead.",
            args.branch_name,
            emphasis_text("--remote")
        ));
    };

    if args.json {
        let Some(version_info) = installation.get_version_info_json()? else {
            return Err(anyhow!("No version information available for local branch"));
        };

        println!("{}", version_info);
        Ok(())
    } else {
        let Some(version_info) = installation.get_version_info()? else {
            return Err(anyhow!("No version information available for local branch"));
        };

        pretty_print_version_info(&args.branch_name, version_info, false);
        Ok(())
    }
}

/// Get release information for a remote branch.
async fn get_release_info_remote(args: &Info, state: &AppState) -> Result<()> {
    let raw_release_info: String = state
        .release_source
        .get_version_file_file(&args.branch_name)
        .read_into_string()
        .await?;

    if args.json {
        println!("{}", raw_release_info.trim());
        Ok(())
    } else {
        match DalamudVersionInfo::from_str(&raw_release_info) {
            Ok(data) => {
                pretty_print_version_info(&args.branch_name, data, true);
                Ok(())
            }
            Err(err) => Err(anyhow!("could not get remote version information: {}", err)),
        }
    }
}

/// Output the given [`DalamudVersionInfo`] to stdout with pretty formatting.
///
/// # Arguments:
/// * `branch_name` - The name of the branch linked to the verison information.
/// * `info` - The [`DalamudVersionInfo`] data to print.
/// * `remote` - If this information was fetched from a remote source (e.g. from GitHub)
fn pretty_print_version_info(branch_name: &str, info: DalamudVersionInfo, remote: bool) {
    let format = if remote {
        "Remote version information".yellow()
    } else {
        "Local version information".green()
    };

    println!(
        "{} for branch {}:\n\
        - Version: {}\n\
        - Git Sha: {}\n\
        - Revision: {}\n\
        - Key: {}\n\
        - Supported GameVer: {}\n\
        - Runtime version: {}\n\
        - Runtime required: {}",
        format,
        branch_name.bold(),
        info.version,
        info.git_sha.unwrap_or("unknown".to_owned()),
        info.revision.unwrap_or("unknown".to_owned()),
        info.key.unwrap_or("N/A".to_owned()),
        info.supported_game_ver.unwrap_or("unknown".to_owned()),
        info.runtime_version,
        info.runtime_required
    );
}
