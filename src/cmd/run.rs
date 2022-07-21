use crate::engine;
use clap::Args;

#[derive(Args, Debug)]
pub struct RunCmd {}

pub fn run(_cmd: &RunCmd) {
    engine::run();
}
