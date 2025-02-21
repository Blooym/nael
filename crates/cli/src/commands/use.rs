use crate::{AppState, formatting::emphasis_text};

use super::RunnableCommand;
use anyhow::{Result, anyhow};
use clap::Parser;
use nael_core::dalamud::DalamudInstallation;

/// Switch the currently active branch.
#[derive(Debug, Parser)]
pub struct Use {
    /// The branch name to use.
    branch_name: String,
}

impl RunnableCommand for Use {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(installation) = DalamudInstallation::get(&self.branch_name, &state.storage)?
        else {
            return Err(anyhow!(
                "Branch '{}' is not installed.\nTip: run '{}' to install it.",
                self.branch_name,
                emphasis_text(&format!("nael install {}", self.branch_name))
            ));
        };

        match installation.set_active() {
            Err(err) => Err(anyhow!(
                "Failed to use switch to branch '{}': {}",
                &self.branch_name,
                err
            )),
            _ => {
                println!("Successfully set branch '{}' as active.", &self.branch_name);
                Ok(())
            }
        }
    }
}
