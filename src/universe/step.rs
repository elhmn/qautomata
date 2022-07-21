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
                self.living_cells
                    .contains_key(&Coordinates { x: x_min, y: y_min }),
                self.living_cells
                    .contains_key(&Coordinates { x: x_min, y: y_min }),
                self.living_cells
                    .contains_key(&Coordinates { x: x_min, y: y_min }),
            ];

            compute_rule(rules, input_square_state);
            // To be continued when compute_rule() is implemented then tested
        }

        configurations
    }
}

fn compute_rule(_rules: Rules, _input_square_state: [bool; 4]) {}
