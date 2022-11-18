use crate::gui;
use crate::universe::types::*;

pub fn run() {
    let universe = Universe::new_from_files("./fixtures/state_2_adjacent_cells.json");
    gui::run(universe);
}
