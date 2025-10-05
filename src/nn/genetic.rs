use rand::prelude::*;
use crate::car_wrapper::AICar;

pub fn select_parent(aicars: &[AICar]) -> &AICar {
    let mut rng = rand::rng();
    let mut best_idx = rng.random_range(0..aicars.len());
    for _ in 0..100 {
        let idx = rng.random_range(0..aicars.len());
        if aicars[idx].fitness > aicars[best_idx].fitness {
            best_idx = idx;
        }
    }
    &aicars[best_idx]
}