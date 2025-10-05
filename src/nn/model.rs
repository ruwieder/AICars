use nalgebra::DVector;

use crate::nn::{linear::Linear, activators::Tanh};
#[derive(Clone)]
pub struct Model<const INPUT: usize, const OUTPUT: usize> {
    l1: Linear<INPUT, 10>,
    a1: Tanh<10, 10>,
    // l2: Linear<15, 10>,
    // a2: Tanh<10, 10>,
    l3: Linear<10, OUTPUT>,
    a3: Tanh<OUTPUT, OUTPUT>,
}

#[allow(dead_code)]
impl<const INPUT: usize, const OUTPUT: usize> Model<INPUT, OUTPUT> {
    pub fn init() -> Self {
        Self { 
            l1: Linear::normal(0.0, 1.0),
            a1: Tanh{},
            // l2: Linear::normal(0.0, 1.0),
            // a2: Tanh{},
            l3: Linear::normal(0.0, 1.0),
            a3: Tanh{},
        }
    }
    
    pub fn forward(&self, x: &DVector<f32>) -> DVector<f32> {
        let mut x = self.l1.forward(x);
        self.a1.forward(&mut x);
        // let mut x = self.l2.forward(&x);
        // self.a2.forward(&mut x);
        let mut x = self.l3.forward(&x);
        self.a3.forward(&mut x);
        x
    }
    
    pub fn crossover(&mut self, other: &Self, rate: f64) {
        self.l1.crossover(&other.l1, rate);
        // self.l2.crossover(&other.l2, rate);
        self.l3.crossover(&other.l3, rate);
    }
    
    pub fn mutate(&mut self, rate: f32) {
        self.l1.mutate(rate);
        // self.l2.mutate(rate);
        self.l3.mutate(rate);
    }
}