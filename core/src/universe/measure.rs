use num::complex::Complex;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

use super::types::*;

impl Universe {
    // This function measure the state of the universe
    // destroying all configurations except 1
    // This configuration is chosen randomly
    // according to a probability distribution
    // computed from the amplitudes of the configurations
    pub fn measure(&mut self) {
        if self.state.len() <= 1 {
            return;
        }

        let mut state_weights: Vec<f64> = Vec::new();

        for configuration in self.state.iter() {
            let weight = configuration.amplitude.norm_sqr();
            state_weights.push(weight);
        }

        let state_distribution = WeightedIndex::new(&state_weights).unwrap();
        let mut rng = thread_rng();
        let chosen_configuration_index = state_distribution.sample(&mut rng);
        let mut chosen_configuration = self.state.swap_remove(chosen_configuration_index);
        chosen_configuration.amplitude = Complex::new(1.0, 0.0);
        self.state = vec![chosen_configuration];
    }
}

// Need to implement tests
#[cfg(test)]
mod tests {

    #[test]
    fn test_measure() {}
}
