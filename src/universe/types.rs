use std::collections::HashMap;
use num::complex::Complex;

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

// The HashMap bool value in the living_cells attribute
// is true if this cell has already been computed during the current step
// We decided to do that to optimize memory but it needs to be reviewed later
// use a struct would be better for readability
pub struct Configuration {
    pub amplitude: Complex<f64>,
    pub living_cells: HashMap<Coordinates, bool>,
}

pub type State = Vec<Configuration>;

// The Operator defines a 16x16 grid of complex number
pub type Operator = [[Complex<f64>; 16]; 16];

pub struct Universe {
    pub state: State,
    pub operator: Operator,
}
