pub mod gen;
pub mod run;

use clap::{Parser, Subcommand};

/// Quantum cellular automata
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// generate fixture data for qautomata
    Gen(gen::GenCmd),

    /// run the quantum cellular automata
    Run(run::RunCmd),
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen(cmd) => {
            gen::generate(cmd);
        }
        Commands::Run(cmd) => {
            run::run(cmd);
        }
    }
}
