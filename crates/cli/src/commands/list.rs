use super::RunnableCommand;
use crate::{formatting::selected_value, AppState};
use anyhow::Result;
use clap::Parser;
use nael_core::dalamud::management::DalamudInstallation;

const LIST_SYMBOL_ACTIVE: &str = "*";
const LIST_SYMBOL_INACTIVE: &str = "-";

/// List all installed branches of Dalamud.
#[derive(Debug, Parser)]
pub struct List;

impl RunnableCommand for List {
    async fn run(&self, state: &AppState) -> Result<()> {
        let Some(installations) = DalamudInstallation::get_all(&state.storage)? else {
            println!("No branches are currently installed.");
            return Ok(());
        };

        if installations.is_empty() {
            println!("No branches are currently installed.");
            return Ok(());
        }

        let msg = format!(
            "Installed branches:\n{}",
            installations
                .iter()
                .map(|v| {
                    if v.is_active().unwrap_or(false) {
                        format!(" {LIST_SYMBOL_ACTIVE} {}", selected_value(&v.branch_name))
                    } else {
                        format!(" {LIST_SYMBOL_INACTIVE} {}", v.branch_name)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
        println!("{msg}");

        Ok(())
    }
}
