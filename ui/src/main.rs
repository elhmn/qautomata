use clap::Parser;

mod sketch;

/// Quantum cellular automata
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the file containing the initial state of the universe
    #[clap(value_name = "STATE_FILE", index = 1)]
    state_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let state_file = match args.state_file {
        Some(path) => path,
        None => {
            println!(
                "Please provide the path to the file containing the initial state of the universe"
            );
            return;
        }
    };

    sketch::run(state_file);
}
