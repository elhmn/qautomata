use crate::universe::types::*;

pub fn run() {
    let mut universe = Universe::new_from_files("./fixtures/state1.json");

    // The counter `n` is temporary until we implement
    // a cleaner way to stop the loop
    for _n in 0..10 {
        universe.step();
        if universe.state.len() > 128 {
            universe.measure();
        }
    }
}
