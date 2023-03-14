use crate::gui;
use crate::universe::types::*;

pub fn run(state_file: &Option<String>) {
    // If there is a state file, use it as the starting state
    // Use an empty universe otherwise (unique configuration with no living cell)
    let universe = match state_file {
        None => Universe::new(),
        Some(s) => match Universe::new_from_files(s) {
            Ok(u) => u,
            Err(err) => {
                println!("Error: failed to load state file: {err}");
                return;
            }
        },
    };

    gui::run(universe);
}
