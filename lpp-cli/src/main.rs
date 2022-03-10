mod commands;

use crate::commands::GenerateCst;

use lpp::Failable;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    GenerateCst(GenerateCst),
}

fn main() -> Failable<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Command::GenerateCst(args) => {
                commands::generate_cst(args)?;
            },
        }
    }

    Ok(())
}
