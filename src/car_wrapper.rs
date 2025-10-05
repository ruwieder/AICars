use std::f32::consts::PI;

use nalgebra::DVector;
use macroquad::{prelude::*, rand::rand};
use crate::{car::{Car, DRAW_SCALE}, nn::model::Model, track::Track, RAY_COUNT, RAY_WIDTH};

const MODEL_INPUT: usize = RAY_COUNT + 2;
const MODEL_OUTPUT: usize = 4;
pub struct AICar {
    pub car: Car,
    pub model: Model<MODEL_INPUT, MODEL_OUTPUT>,
    rays: DVector<f32>,
    since_last_raycast: f32,
    pub fitness: f32,
    next_cp: usize,
    pub is_dead: bool,
    to_live: f32,
    time_per_cp: f32,
}

#[allow(dead_code)]
impl AICar {
    pub fn init() -> Self {
        Self::init_with_model(Model::init())
    }
    pub fn init_with_model(model: Model<MODEL_INPUT, MODEL_OUTPUT>) -> Self {
        Self { 
            car: Car::new(
                (
                    300.0 + 50.0 * (rand() as f32 / u32::MAX as f32), 
                    170.0 + 50.0 * (rand() as f32 / u32::MAX as f32)
                ),
                (rand() as f32 / u32::MAX as f32 - 0.5) * PI / 3.0
            ),
            model, 
            rays: DVector::zeros(RAY_COUNT),
            since_last_raycast: 3.0,
            fitness: 5.0,
            next_cp: 1,
            is_dead: false,
            to_live: 8.0,
            time_per_cp: 4.0,
        }
    }
    
    // pub fn respawn(&mut self) {
    //     self.car = Car::new((300.0, 200.0));
    //     self.next_cp = 0;
    //     self.since_last_raycast = 3.0;
    // }
    
    pub fn update(&mut self, track: &Track, dt: f32) {
        if self.is_dead { return; };
        if self.car.check_collision(track) { 
            self.is_dead = true;
            self.fitness -= self.car.vel * 10.0;
            return;
        }
        if self.to_live <= 0.0 {
            self.is_dead = true;
            return;
        }
        self.to_live -= dt;
        self.fitness += self.car.vel * dt / 100.0;
        self.check_checkpoint(track);
        let mut data: DVector<f32> = DVector::zeros(MODEL_INPUT);
        for i in 0..RAY_COUNT { data[i] = self.rays[i]; };
        data[RAY_COUNT] = self.car.vel / 200.0;
        data[RAY_COUNT + 1] = self.since_last_raycast;
        let result = self.model.forward(&data);
        if result[2] > 0.0 {
            // let width = (result[3] / 2.0 + 0.5).clamp(0.3, 1.5);
            let width = PI;
            self.rays = self.car.raycast(track, width);
            // self.fitness -= 0.05;
            self.to_live -= 0.05;
            self.since_last_raycast = 0.0;
        };
        self.since_last_raycast += dt;
        self.car.update(result[0], result[1], dt);
    }
    
    pub fn draw_rays(&self) {
        const D_ANGLE: f32 = RAY_WIDTH / RAY_COUNT as f32;
        for i in 0..RAY_COUNT {
            let angle = self.car.angle - RAY_WIDTH / 2.0 + i as f32 * D_ANGLE;
            draw_line(
                self.car.pos.0, 
                self.car.pos.1, 
                self.car.pos.0 + self.rays[i] * angle.cos(), 
                self.car.pos.1 + self.rays[i] * angle.sin(), 
                1.0, 
                Color { r: 0.8, g: 0.0, b: 0.0, a: 1.0 }
            );
        }
    }
    
    fn check_checkpoint(&mut self, track: &Track) {
        let back = (
            self.car.pos.0 - DRAW_SCALE * 10.0 * self.car.angle.cos(),
            self.car.pos.1 - DRAW_SCALE * 10.0 * self.car.angle.sin(),
        );
        if track.raycast_checkpoint(back, self.car.angle, self.next_cp).unwrap_or(100.0) < 30.0 {
            self.next_cp = (self.next_cp + 1) % track.count;
            if self.next_cp == 0 {
                self.fitness += 2000.0 * self.to_live / self.time_per_cp;
            }
            if self.next_cp == 1 {
                self.time_per_cp *= 0.75;
            }
            self.to_live += self.time_per_cp;
            // self.fitness += 100.0 * (1.0 + self.to_live).powf(0.1);
            self.fitness += 100.0;
        }
    }
    
    pub fn draw(&self) {
        self.car.draw();
        // draw_text(&self.next_cp.to_string(), self.car.pos.0, self.car.pos.1 - 15.0, 20.0, PINK);
    }
    
    // pub fn new_from_parents(parent1: &Self, parent2: &Self) -> Self {
    pub fn new_from_parents(parent1: &Self) -> Self {
        let mut model = parent1.model.clone();
        // model.crossover(&parent2.model, 0.5);
        model.mutate(0.03);
        Self::init_with_model(model)
    }
}
