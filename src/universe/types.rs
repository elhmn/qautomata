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

pub type Universe = Vec<Configuration>;
