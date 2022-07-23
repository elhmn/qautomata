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
        self.is_even_step = !self.is_even_step;
    }
}

impl Configuration {
    pub fn step(&mut self, rules: Rules, is_even_step: bool) -> Vec<Configuration> {
        let mut new_configurations: Vec<Configuration> = vec![Configuration {
            amplitude: self.amplitude,
            living_cells: HashMap::new(),
        }];

        for coordinates in self
            .living_cells
            .keys()
            .cloned()
            .collect::<Vec<Coordinates>>()
        {
            if self.living_cells.get(&coordinates) == Some(&true) {
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

            // The square_state contains 4 bool, one for each cell of the square
            // The boolean is true if the cell is alive and false otherwise
            // Cell order in the configuration compared to the square_state array:
            // [0] [1] -> configuration
            // [2] [3]
            //
            // [0 1 2 3] -> square_state
            //
            // The values are initialized at false and updated to true below
            // when checking if the correspondings cells are alive
            let mut square_state: [bool; 4] = [false, false, false, false];

            // Get the 4 living cells of the square [0] [1]
            //                                      [2] [3]
            //
            // - Mark the living cells as computed during this step
            //    in order to not compute them again in the loop
            // - Mark the cells alive in the `square_state` array
            //    if we find them in the list of living cells (see comment above)
            {
                let living_cell_0 = self
                    .living_cells
                    .get_mut(&Coordinates { x: x_min, y: y_min });
                if let Some(lc0) = living_cell_0 {
                    *lc0 = true;
                    square_state[0] = true;
                }
            }
            {
                let living_cell_1 = self.living_cells.get_mut(&Coordinates {
                    x: x_min,
                    y: y_min + 1,
                });
                if let Some(lc1) = living_cell_1 {
                    *lc1 = true;
                    square_state[1] = true;
                }
            }
            {
                let living_cell_2 = self.living_cells.get_mut(&Coordinates {
                    x: x_min + 1,
                    y: y_min,
                });
                if let Some(lc2) = living_cell_2 {
                    *lc2 = true;
                    square_state[2] = true;
                }
            }
            {
                let living_cell_3 = self.living_cells.get_mut(&Coordinates {
                    x: x_min + 1,
                    y: y_min + 1,
                });
                if let Some(lc3) = living_cell_3 {
                    *lc3 = true;
                    square_state[3] = true;
                }
            }

            let new_square_states: Vec<(Complex<f64>, [bool; 4])> =
                compute_rules(rules, square_state);

            // Think about what to do here, probably an error
            if new_square_states.is_empty() {
                continue;
            }

            // For each new_configuration:
            //     - For each new_square_state except the first one:
            //         - We create a copy of the new_configuration, apply new_square_state and
            //           add it to new_configurations (we won't iter on it on the main
            //           loop)
            //     - Then we apply the first new_square_state to the new_configuation
            //
            // Apply a square_state to a configuration:
            //     - multiply the amplitude of the configuration with the one of the square_state
            //     - add each living_cell of the square_state in the living_cells of the
            //       configuration
            for i in 0..new_configurations.len() {
                for new_square_state in new_square_states.iter().skip(1) {
                    let mut new_configuration: Configuration = new_configurations[i].clone();

                    new_configuration.amplitude *= new_square_state.0;
                    if new_square_state.1[0] {
                        new_configuration
                            .living_cells
                            .insert(Coordinates { x: x_min, y: y_min }, false);
                    }
                    if new_square_state.1[1] {
                        new_configuration.living_cells.insert(
                            Coordinates {
                                x: x_min,
                                y: y_min + 1,
                            },
                            false,
                        );
                    }
                    if new_square_state.1[2] {
                        new_configuration.living_cells.insert(
                            Coordinates {
                                x: x_min + 1,
                                y: y_min,
                            },
                            false,
                        );
                    }
                    if new_square_state.1[3] {
                        new_configuration.living_cells.insert(
                            Coordinates {
                                x: x_min + 1,
                                y: y_min + 1,
                            },
                            false,
                        );
                    }

                    new_configurations.push(new_configuration);
                }

                new_configurations[i].amplitude *= new_square_states[0].0;
                if new_square_states[0].1[0] {
                    new_configurations[i]
                        .living_cells
                        .insert(Coordinates { x: x_min, y: y_min }, false);
                }
                if new_square_states[0].1[1] {
                    new_configurations[i].living_cells.insert(
                        Coordinates {
                            x: x_min,
                            y: y_min + 1,
                        },
                        false,
                    );
                }
                if new_square_states[0].1[2] {
                    new_configurations[i].living_cells.insert(
                        Coordinates {
                            x: x_min + 1,
                            y: y_min,
                        },
                        false,
                    );
                }
                if new_square_states[0].1[3] {
                    new_configurations[i].living_cells.insert(
                        Coordinates {
                            x: x_min + 1,
                            y: y_min + 1,
                        },
                        false,
                    );
                }
            }
        }

        new_configurations
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
                exp: vec![(Complex::new(1.0, 0.0), [false, false, false, true])],
            },
            Test {
                rules,
                ss: [false, false, false, true],
                exp: vec![(Complex::new(1.0, 0.0), [false, false, true, false])],
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
