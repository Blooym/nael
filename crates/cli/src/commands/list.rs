use super::RunnableCommand;
use crate::{formatting::selected_value, AppState};
use anyhow::Result;
use clap::Parser;
use nael_core::dalamud::management::DalamudInstallation;

/// List all installed branches of Dalamud.
#[derive(Debug, Parser)]
pub struct List;

impl RunnableCommand for List {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(branches) = DalamudInstallation::get_all(&state.storage)? else {
            println!("No Dalamud branches are currently installed.");
            return Ok(());
        };

        if branches.is_empty() {
            println!("No Dalamud branches are currently installed.");
            return Ok(());
        }

        let msg = format!(
            "Installed Dalamud branches:\n{}",
            branches
                .iter()
                .map(|v| {
                    if v.is_active().unwrap_or(false) {
                        format!(" - {}", selected_value(&v.branch_name))
                    } else {
                        format!(" - {}", v.branch_name)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
        println!("{}", msg);

        Ok(())
    }
}
