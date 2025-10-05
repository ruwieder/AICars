use nalgebra::{DMatrix, DVector};
use rand::Rng;
use rand_distr::Distribution;

#[derive(Clone)]
pub struct Linear<const INPUT: usize, const OUTPUT: usize> {
    weights: DMatrix<f32>,
    bias:    DVector<f32>,
    shape: (usize, usize),
}

#[allow(dead_code)]
impl<const INPUT: usize, const OUTPUT: usize> Linear<INPUT, OUTPUT> {
    pub fn zeros() -> Self {
        Self {
            weights: DMatrix::<f32>::zeros(OUTPUT, INPUT),
            bias: DVector::<f32>::zeros(INPUT),
            shape: (INPUT, OUTPUT)
        }
    }
    
    pub fn normal(mean: f32, std_dev: f32) -> Self{
        let mut rng = rand::rng();
        let normal = rand_distr::Normal::new(mean, std_dev).unwrap();
        Self { 
            weights: DMatrix::<f32>::from_fn(OUTPUT, INPUT,
                |_, _| { normal.sample(&mut rng) }
            ),
            bias: DVector::<f32>::from_fn(OUTPUT, 
                |_, _|{ normal.sample(&mut rng) }
            ),
            shape: (INPUT, OUTPUT)
        }
    }
    
    pub fn forward(&self, x: &DVector<f32>) -> DVector<f32> {
        &self.weights * x + &self.bias
    }
    
    pub fn crossover(&mut self, other: &Self, rate: f64) {
        debug_assert_eq!(self.shape, other.shape);
        let mut rng = rand::rng();
        for (a, b) in self.weights.as_mut_slice().iter_mut().zip(other.weights.as_slice()) {
            if rng.random_bool(rate) {
                *a = *b;
            }
        }
        for (a, b) in self.bias.as_mut_slice().iter_mut().zip(other.bias.as_slice()) {
            if rng.random_bool(rate) {
                *a = *b;
            }
        }
    }
    
    pub fn mutate(&mut self, rate: f32) {
        let mut rng = rand::rng();
        let normal = rand_distr::Normal::new(0.0, rate).unwrap();
        for v in self.weights.as_mut_slice() {
            *v += normal.sample(&mut rng);
        }
        for v in self.bias.as_mut_slice() {
            *v += normal.sample(&mut rng);
        }
    }
    
    pub fn apply(&mut self, func: fn(&mut f32)) {
        for v in self.weights.as_mut_slice() { func(v); }
        for v in self.bias.as_mut_slice() { func(v); }
    }
}
