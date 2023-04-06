use crate::engine;
use clap::Args;

#[derive(Args, Debug)]
pub struct RunCmd {
    // the starting state file
    #[clap(value_name = "STATE_FILE", index = 1)]
    state_file: Option<String>,
}

pub fn run(args: &RunCmd) {
    engine::run(&args.state_file);
}
