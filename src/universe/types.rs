use super::files;
use num::complex::Complex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Complex")]
pub struct ComplexDef<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}

// The HashMap bool value in the living_cells attribute
// is true if this cell has already been computed during the current step
// We decided to do that to optimize memory but it needs to be reviewed later
// use a struct would be better for readability
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(with = "ComplexDef")]
    pub amplitude: Complex<f64>,

    #[serde_as(as = "Vec<(_, _)>")]
    pub living_cells: HashMap<Coordinates, bool>,
}

pub type State = Vec<Configuration>;

// The Rules defines a 16x16 grid of complex number
pub type Rules = [[Complex<f64>; 16]; 16];

// The is_even_step attribute is used to determine the square in which
// the rules of the universe apply for a given living cell
// It is true if the universe is in an even step and false othrerwise
#[derive(Debug)]
pub struct Universe {
    pub state: State,
    pub is_even_step: bool,
    pub rules: Rules,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            is_even_step: true,
            rules: [[Complex::new(0.0, 0.0); 16]; 16],
        }
    }

    pub fn new_from_files(state_file: &str) -> Self {
        let state = files::get_state_from_file(state_file);
        let rules = get_test_rules();
        Self {
            state,
            rules,
            is_even_step: true,
        }
    }
}

// get_test_rules return an array of rules
// at term the rules should normally be extrated from a file
// but for now these rules are written in the code
// and used for testing purposes
pub fn get_test_rules() -> Rules {
    let c = |x: f64, y: f64| -> Complex<f64> { Complex::new(x, y) };

    [
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(1., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
        ],
    ]
}
