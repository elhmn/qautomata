use crate::universe::types::*;
use num::complex::Complex;

pub fn run() {
    let universe = Universe{
        //This should be replaced by an State::new(state_file),
        state: State::new(),
        //This should be replaced by an Operator::new(operator_file),
        operator: [[Complex::new(0.0, 0.0); 16]; 16],
    };


    // The counter `n` is temporary until we implement
    // a cleaner way to stop the loop
    for _n in 0..10 {
        universe.step();
    };
}
