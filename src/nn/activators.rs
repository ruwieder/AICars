use nalgebra::DVector;
#[derive(Clone, Copy)]
pub struct Tanh<const INPUT: usize, const OUTPUT: usize> {
    
}

impl<const INPUT: usize, const OUTPUT: usize> Tanh<INPUT, OUTPUT> {
    pub fn forward(&self, x: &mut DVector<f32>) {
        for v in x.as_mut_slice() { *v = v.tanh() }
    }
}
