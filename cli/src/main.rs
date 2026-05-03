use anyhow::Result;
use clap::{Parser, Subcommand};

mod cargo_check;
mod commands;
mod cover;
mod manifest;
mod scan;
mod toml_writer;
mod workspace;

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
                FeaturesAction::Explain { path } => commands::explain(&path),
                FeaturesAction::Prune { yes } => commands::prune(yes),
            }
        },
    }
}
