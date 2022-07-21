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
}
