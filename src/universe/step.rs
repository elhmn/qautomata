use num::complex::Complex;
use std::collections::HashMap;

use super::types::*;

impl Universe {
    pub fn step(&mut self) {
        let mut new_state: State = State::new();

        for configuration in self.state.iter_mut() {
            new_state.append(&mut configuration.step(self.rules, self.is_even_step));
        }

        self.state = new_state;
    }
}

impl Configuration {
    pub fn step(&mut self, rules: Rules, is_even_step: bool) -> Vec<Configuration> {
        let configurations: Vec<Configuration> = vec![Configuration {
            amplitude: self.amplitude,
            living_cells: HashMap::new(),
        }];

        for (coordinates, is_already_computed) in self.living_cells.iter() {
            if *is_already_computed {
                continue;
            }

            let x_min: i32;
            let y_min: i32;

            // The 2 * 2 squares in which rules locally apply are alternating on each step
            //
            // Example:
            //      Below is an example of a 2 * 2 square in which rules will
            //      locally apply in two consecutive steps, the pair of numbers
            //      inside [] represents the coordinates of a cell
            //
            //             Step 0           Step 1
            //          [0, 2] [0, 3] -> [1, 3] [1, 4]
            //          [1, 2] [1, 3]    [2, 3] [2, 4]
            //
            //
            // On even steps (Step 0 above as an example), to calculate x_min (same for y_min)
            // given one of the cells [x, y] of a 2 * 2 square in which rules locally apply
            // we take the value of x if x is even or x - 1 if x is odd
            //
            // On odd steps (Step 1 above as an example), to calculate x_min (same for y_min)
            // given one of the cells [x, y] of a 2 * 2 square in which rules locally apply
            // we take the value of x if x is odd or x - 1 if x is even
            if is_even_step {
                x_min = coordinates.x - if coordinates.x % 2 == 0 { 0 } else { 1 };
                y_min = coordinates.y - if coordinates.y % 2 == 0 { 0 } else { 1 };
            } else {
                x_min = coordinates.x - if coordinates.x % 2 == 1 { 0 } else { 1 };
                y_min = coordinates.y - if coordinates.y % 2 == 1 { 0 } else { 1 };
            }

            let input_square_state: [bool; 4] = [
                self.living_cells
                    .contains_key(&Coordinates { x: x_min, y: y_min }),
                self.living_cells.contains_key(&Coordinates {
                    x: x_min,
                    y: y_min + 1,
                }),
                self.living_cells.contains_key(&Coordinates {
                    x: x_min + 1,
                    y: y_min,
                }),
                self.living_cells.contains_key(&Coordinates {
                    x: x_min + 1,
                    y: y_min + 1,
                }),
            ];

            compute_rules(rules, input_square_state);
            // To be continued when compute_rule() is implemented then tested
        }

        configurations
    }
}

pub fn compute_rules(rules: Rules, square_state: [bool; 4]) -> Vec<(Complex<f64>, [bool; 4])> {
    let mut ret: Vec<(Complex<f64>, [bool; 4])> = Vec::new();
    let index = square_state_to_index(square_state) as usize;
    let len = rules[0].len();

    for (ri, row) in rules.iter().enumerate().take(len) {
        let amplitude = row[index];
        if amplitude.norm() != 0.0 {
            let new_square_state = index_to_square_state(ri as i32);
            ret.push((amplitude, new_square_state));
        }
    }

    ret
}

//square_state_to_index converts a square_state in index in the 16x16 gird of rules
//
//To convert the array of boolean in a number that represents an index
//in a 16x16 grid of rules.
//That grid contains the 16x16 possible combinations of booleans
//we can have in a [b_0, b_1, b_2, b_3] array of booleans
//with b_i = (0 | 1)
//
// Example:
// Given an array [false, true, false, false]
// wich in binary is [0000, 0001, 0000, 0000]
//
// We first shift these binary bits, to position them approriately in an i32 sized number
// 0000 << 3 = 0000
// 0001 << 2 = 0100
// 0000 << 1 = 0000
// 0000 << 0 = 0000
//
// Then we combine them using the OR bitwise operator to get an i32 number
// number = 0000 | 0100 | 0000 | 0000;
//
//The resulted number is number = 4 = 0100
fn square_state_to_index(square_state: [bool; 4]) -> i32 {
    let s = [
        (square_state[0] as i32) << 3,
        (square_state[1] as i32) << 2,
        (square_state[2] as i32) << 1,
        square_state[3] as i32,
    ];

    // return the index
    s[0] | s[1] | s[2] | s[3]
}

fn index_to_square_state(index: i32) -> [bool; 4] {
    [
        ((index >> 3) & 1) == 1,
        ((index >> 2) & 1) == 1,
        ((index >> 1) & 1) == 1,
        (index & 1) == 1,
    ]
}

#[cfg(test)]
mod tests {
    use crate::universe::{step, types};
    use num::complex::Complex;

    #[test]
    fn test_compute_rule() {
        let rules = types::get_test_rules();

        struct Test {
            rules: types::Rules,
            ss: [bool; 4],
            exp: Vec<(Complex<f64>, [bool; 4])>,
        }

        let tests = [
            Test {
                rules,
                ss: [false, true, false, false],
                exp: vec![(Complex::new(1.0, 0.0), [true, false, false, true])],
            },
            Test {
                rules,
                ss: [false, false, false, true],
                exp: vec![(Complex::new(1.0, 0.0), [false, false, false, true])],
            },
        ];

        for t in tests {
            let got = step::compute_rules(t.rules, t.ss);
            assert_eq!(got, t.exp);
        }
    }

    #[test]
    fn test_square_state_to_index() {
        struct Test {
            ss: [bool; 4],
            exp: i32,
        }

        let tests = [
            Test {
                ss: [false, false, false, false],
                exp: 0,
            },
            Test {
                ss: [true, false, false, true],
                exp: 9,
            },
            Test {
                ss: [false, true, false, false],
                exp: 4,
            },
            Test {
                ss: [false, true, false, true],
                exp: 5,
            },
            Test {
                ss: [true, true, false, true],
                exp: 13,
            },
            Test {
                ss: [true, true, true, false],
                exp: 14,
            },
            Test {
                ss: [true, true, true, true],
                exp: 15,
            },
        ];

        for t in tests {
            let got = step::square_state_to_index(t.ss);
            assert_eq!(got, t.exp);
        }
    }

    #[test]
    fn test_index_square_state() {
        struct Test {
            exp: [bool; 4],
            index: i32,
        }

        let tests = [
            Test {
                index: 0,
                exp: [false, false, false, false],
            },
            Test {
                index: 9,
                exp: [true, false, false, true],
            },
            Test {
                index: 4,
                exp: [false, true, false, false],
            },
            Test {
                index: 5,
                exp: [false, true, false, true],
            },
            Test {
                index: 13,
                exp: [true, true, false, true],
            },
            Test {
                index: 14,
                exp: [true, true, true, false],
            },
            Test {
                index: 15,
                exp: [true, true, true, true],
            },
        ];

        for t in tests {
            let got = step::index_to_square_state(t.index);
            assert_eq!(got, t.exp);
        }
    }
}
