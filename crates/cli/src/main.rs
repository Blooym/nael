mod commands;
mod formatting;

use self::commands::{Active, Info, Install, List, Remove, RunnableCommand, Update, Use};
use crate::formatting::error_text;
use anyhow::Result;
use clap::Parser;
use commands::SymlinkPath;
use nael_core::{dalamud::GoatcorpReleaseSource, fs::storage::CompliantDiskStorage};
use std::{process::ExitCode, sync::Arc};

#[cfg(target_os = "windows")]
use colored::control;

const APP_QUALIFIER: &str = "dev";
const APP_ORGANIZATION: &str = "Blooym";
const APP_NAME: &str = "Nael";

struct AppState {
    storage: Arc<CompliantDiskStorage>,
    release_source: GoatcorpReleaseSource,
}

#[derive(Debug, Parser)]
enum NaelCommand {
    Install(Install),
    Update(Update),
    Remove(Remove),
    List(List),
    Use(Use),
    Active(Active),
    SymlinkPath(SymlinkPath),
    Info(Info),
}

impl RunnableCommand for NaelCommand {
    async fn run(&self, state: &AppState) -> Result<()> {
        match self {
            NaelCommand::Install(cmd) => cmd.run(state).await,
            NaelCommand::Remove(cmd) => cmd.run(state).await,
            NaelCommand::Update(cmd) => cmd.run(state).await,
            NaelCommand::List(cmd) => cmd.run(state).await,
            NaelCommand::Info(cmd) => cmd.run(state).await,
            NaelCommand::Use(cmd) => cmd.run(state).await,
            NaelCommand::Active(cmd) => cmd.run(state).await,
            NaelCommand::SymlinkPath(cmd) => cmd.run(state).await,
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Opts {
    #[clap(subcommand)]
    cmd: NaelCommand,
}

#[tokio::main]
async fn main() -> ExitCode {
    #[cfg(target_os = "windows")]
    control::set_virtual_terminal(true).expect("Failed to set virtual terminal");

    let opts = Opts::parse();

    if let Err(err) = opts
        .cmd
        .run(&AppState {
            release_source: GoatcorpReleaseSource,
            storage: Arc::from(CompliantDiskStorage::new(
                APP_QUALIFIER,
                APP_ORGANIZATION,
                APP_NAME,
            )),
        })
        .await
    {
        eprintln!("{}: {:?}", error_text("error"), err);
        return ExitCode::from(1);
    };
    ExitCode::SUCCESS
}
