use super::RunnableCommand;
use crate::{AppState, formatting::emphasis_text};
use anyhow::{Context, Result, anyhow};
use clap::{Parser, ValueEnum};
use nael_core::dalamud::DalamudInstallation;

#[derive(Debug, Default, Clone, ValueEnum)]
enum OutputFormat {
    /// Output only the name of the active branch.
    #[default]
    Name,

    /// Output the path to the active branch.
    Path,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Get information about the active branch.
#[derive(Debug, Clone, Parser)]
pub struct Active {
    /// The format used for outputting the active branch.
    #[clap(short = 'f', long = "format", default_value_t, value_enum)]
    format: OutputFormat,

    /// Return an empty value with a successful exit code if no branch is set, useful for various tools.
    #[clap(short = 'e', long = "empty-if-none", default_value_t = false)]
    empty_if_none: bool,
}

impl RunnableCommand for Active {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(active_installation) = DalamudInstallation::get_active(&state.storage)? else {
            if self.empty_if_none {
                return Ok(());
            }
            return Err(anyhow!(
                "No active branch set, or last active branch was removed improperly.\nTip: run '{}' to set an active branch.",
                emphasis_text("nael use <branch>")
            ));
        };

        match self.format {
            OutputFormat::Name => {
                println!("{}", active_installation.branch_name);
            }
            OutputFormat::Path => {
                println!(
                    "{}",
                    active_installation
                        .get_location()?
                        .context("could not determine active branch location on disk")?
                        .to_str()
                        .context("could not parse location to str")?
                );
            }
        };

        Ok(())
    }
}
