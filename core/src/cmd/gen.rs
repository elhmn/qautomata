use crate::universe::types;
use clap::Args;
use num::complex::Complex;
use rand::Rng;
use std::collections::HashMap;

#[derive(Args, Debug)]
pub struct GenCmd {
    /// generate a state, which consist of a list of configurations
    #[clap(short, long, value_parser)]
    state: bool,

    /// store generated data in a Json file
    #[clap(short, long, value_parser)]
    out: Option<String>,
}

pub fn generate(_cmd: &GenCmd) {
    let universe = types::Universe::new();
    let mut state = universe.state;

    let mut configuration = types::Configuration {
        //The module of the amplitude should be equal to 1
        amplitude: Complex::new(1., 0.),
        living_cells: HashMap::new(),
    };

    //Generate a set of living cells
    let max_x = 100;
    let max_y = 100;
    let max_number_of_living_cells = 100;
    let mut rng = rand::thread_rng();
    //Get the number of cells we would like to have on the map
    let number_of_cells: i32 = rng.gen_range(0..max_number_of_living_cells);
    for _i in 0..number_of_cells {
        configuration.living_cells.insert(
            types::Coordinates {
                x: rng.gen_range(0..max_x),
                y: rng.gen_range(0..max_y),
            },
            false,
        );
    }

    state.push(configuration);
    let serialized_state = serde_json::to_string(&state).unwrap();
    println!("{}", serialized_state);
}
