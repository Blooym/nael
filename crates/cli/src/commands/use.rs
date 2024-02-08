use crate::{formatting::emphasis_text, AppState};

use super::RunnableCommand;
use anyhow::{anyhow, Result};
use clap::Parser;
use nael_core::dalamud::management::DalamudInstallation;

/// Switch the currently active Dalamud branch.
#[derive(Debug, Parser)]
pub struct Use {
    /// The branch name to use.
    branch_name: String,
}

impl RunnableCommand for Use {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(install) = DalamudInstallation::get(&self.branch_name, &state.storage)? else {
            return Err(anyhow!(
                "Branch '{}' is not installed\nTip: Run '{}' to try and install it",
                self.branch_name,
                emphasis_text(&format!("nael install {}", self.branch_name))
            ));
        };

        match install.set_active() {
            Err(e) => Err(anyhow!(
                "Failed to use switch to branch '{}': {}",
                &self.branch_name,
                e
            )),
            Ok(_) => {
                println!("Successfully switched to branch '{}'.", &self.branch_name);
                Ok(())
            }
        }
    }
}