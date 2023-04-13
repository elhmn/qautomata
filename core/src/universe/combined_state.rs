use super::types::*;

impl Universe {
    // This function computes the combined state
    // of a Universe from its state
    //
    // The combined state contains the probability
    // of each cell being alive
    //
    // This probability is the sum of probabilities
    // of each configuration (associated with their amplitude)
    // in which a given cell is alive
    //
    // We don't call this function
    // after computing a step
    // because it's faster to compute
    // the combined state while
    // computing the step
    pub fn compute_combined_state(&mut self) {
        self.combined_state.clear();

        for configuration in self.state.iter() {
            let probability = configuration.amplitude.norm_sqr();

            for coordinates in configuration.living_cells.keys() {
                *self
                    .combined_state
                    .entry(Coordinates {
                        x: coordinates.x,
                        y: coordinates.y,
                    })
                    .or_insert(0.0) += probability;
            }
        }
    }
}
