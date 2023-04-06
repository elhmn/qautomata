use super::files;
use num::complex::Complex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::Error;

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Universe {
    pub state: State,
    pub combined_state: HashMap<Coordinates, f64>,
    pub is_even_step: bool,
    pub rules: Rules,
}

impl Universe {
    pub fn new() -> Self {
        // Unique configuration with no living cell
        let configuration = Configuration {
            amplitude: Complex::new(1., 0.),
            living_cells: HashMap::new(),
        };
        let state = vec![configuration];
        let rules = get_test_rules();
        Self {
            state,
            combined_state: HashMap::new(),
            is_even_step: true,
            rules,
        }
    }

    pub fn new_from_files(state_file: &str) -> Result<Self, Error> {
        let state = files::get_state_from_file(state_file)?;
        let rules = get_test_rules();
        Ok(Self {
            state,
            combined_state: HashMap::new(),
            is_even_step: true,
            rules,
        })
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
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1., 1.),
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
            c(1., 0.),
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
            c(0., PI / 4.0).exp(),
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
            c(1. / 2.0_f64.sqrt(), 0.),
            c(0., 0.),
            c(0., 0.),
            c(1. / 2.0_f64.sqrt(), 0.),
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
        ],
        [
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(0., 0.),
            c(1.0 / 2.0_f64.sqrt(), 0.),
            c(0., 0.),
            c(0., 0.),
            c(-1.0 / 2.0_f64.sqrt(), 0.),
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
            c(0., PI / 8.0).exp(),
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
            c(1., 0.),
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
            c(1., 0.),
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
            c(1., 0.),
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
            c(0., PI / 2.0).exp(),
        ],
    ]
}
