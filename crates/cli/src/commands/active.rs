use super::RunnableCommand;
use crate::{formatting::emphasis_text, AppState};
use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use nael_core::{dalamud::management::DalamudInstallation, fs::storage::AppStorage};

#[derive(Debug, Default, Clone, ValueEnum)]
enum OutputFormat {
    /// Output only the name of the active branch.
    #[default]
    Name,

    /// Output the real path to the active branch.
    RealPath,

    /// Output the path to the symlink that points to the active branch.
    SymlinkPath,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Output information about the active Dalamud branch.
#[derive(Debug, Clone, Parser)]
pub struct Active {
    /// The format used for outputting the active branch.
    #[clap(short = 'f', long = "format", default_value_t, value_enum)]
    format: OutputFormat,
}

impl RunnableCommand for Active {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(active_branch) = DalamudInstallation::get_active(&state.storage)? else {
            return Err(anyhow!(
                "No active branch set, or last active branch was removed improperly.\nTip: Run '{}' to set an active branch.",
                emphasis_text("nael use <branch>")
            ));
        };

        match self.format {
            OutputFormat::Name => {
                println!("{}", active_branch.branch_name);
            }
            OutputFormat::SymlinkPath => {
                println!(
                    "{}",
                    state
                        .storage
                        .get_active_branch_symlink()
                        .context("could not find active branch path")?
                        .to_str()
                        .context("could not convert path to string for output")?
                )
            }
            OutputFormat::RealPath => {
                println!(
                    "{}",
                    active_branch
                        .get_location()?
                        .context("could not determine active branch location on disk")?
                        .to_str()
                        .context("could not parse location to str")?
                )
            }
        };

        Ok(())
    }
}
