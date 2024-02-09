mod commands;
mod formatting;

use self::commands::{Active, Info, Install, List, Remove, RunnableCommand, Update, Use};
use crate::formatting::error_text;
use anyhow::Result;
use clap::Parser;
use nael_core::{dalamud::sources::GoatcorpReleaseSource, fs::storage::CompliantDiskStorage};
use std::{process::ExitCode, sync::Arc};

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
    Remove(Remove),
    Update(Update),
    List(List),
    Info(Info),
    Use(Use),
    Active(Active),
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
        eprintln!("{}: {:?}", error_text("Error"), err);
        return ExitCode::from(1);
    };
    ExitCode::SUCCESS
}
