use anyhow::Result;
use clap::{Parser, Subcommand};

use engage::commands;

#[derive(Parser)]
#[command(name = "engage", version, about = "Manage engage-il2cpp Cargo features for your project")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Features {
        #[command(subcommand)]
        action: Option<FeaturesAction>,
    },
}

#[derive(Subcommand)]
enum FeaturesAction {
    Check,
    Apply {
        #[arg(long)]
        yes: bool,
    },
    Add {
        name: String,
    },
    Explain {
        path: String,
    },
    Prune {
        #[arg(long)]
        yes: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Features { action } => {
            match action.unwrap_or(FeaturesAction::Check) {
                FeaturesAction::Check => commands::check(),
                FeaturesAction::Apply { yes } => commands::apply(yes),
                FeaturesAction::Add { name } => commands::add_feature(&name),
                FeaturesAction::Explain { path } => commands::explain(&path),
                FeaturesAction::Prune { yes } => commands::prune(yes),
            }
        },
    }
}
