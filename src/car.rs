use std::f32::consts::PI;
use macroquad::prelude::*;
use nalgebra::DVector;

use crate::track::Track;
use crate::RAY_COUNT;

pub const DRAW_SCALE: f32 = 1.0;
pub const DRAW_RAYS: bool = false;

#[derive(Debug, Clone, Copy)]
pub struct Car {
    pub pos: (f32, f32),
    pub vel: f32,
    pub angle: f32,
    pub color: Color,
}

impl Car {
    pub fn new(pos: (f32, f32), angle: f32) -> Self {
        Self { pos, vel: 0.0, angle, color: LIME }
    }
    pub fn draw(&self) {
        let back = (
            self.pos.0 - DRAW_SCALE * 10.0 * self.angle.cos(),
            self.pos.1 - DRAW_SCALE * 10.0 * self.angle.sin(),
        );
        let back = (
            (
                back.0 + DRAW_SCALE * 5.0 * (self.angle + PI/2.0).cos(),
                back.1 + DRAW_SCALE * 5.0 * (self.angle + PI/2.0).sin(),
            ),
            (
                back.0 + DRAW_SCALE * 5.0 * (self.angle +- PI/2.0).cos(),
                back.1 + DRAW_SCALE * 5.0 * (self.angle +- PI/2.0).sin(),
            )
        );
        let front = (
            self.pos.0 + 10.0 * self.angle.cos(),
            self.pos.1 + 10.0 * self.angle.sin()
        );
        draw_line(front.0, front.1, back.0.0, back.0.1, 1.0, self.color);
        draw_line(front.0, front.1, back.1.0, back.1.1, 1.0, self.color);
        draw_line(back.0.0, back.0.1, back.1.0, back.1.1, 1.0, self.color);
    }
    
    pub fn update(&mut self, forward: f32, steering: f32, dt: f32) {
        self.vel += 100.0 * forward * dt;
        self.angle += 5.0 * steering * dt;
        self.pos.0 += self.vel * self.angle.cos() * dt;
        self.pos.1 += self.vel * self.angle.sin() * dt;
    }
    
    pub fn raycast(&self, track: &Track, width: f32) -> DVector<f32>{
        let d_angle: f32 = width / RAY_COUNT as f32;
        let mut res: DVector<f32> = DVector::zeros(RAY_COUNT);
        for i in 0..RAY_COUNT {
            let angle = self.angle - width / 2.0 + i as f32 * d_angle;
            let t = track.raycast_boundaries(self.pos, angle);
            if t.is_some() {
                let t = t.unwrap();
                res[i] = t;
                if DRAW_RAYS{
                    draw_line(
                        self.pos.0, self.pos.1, 
                        self.pos.0 + t * angle.cos(), 
                        self.pos.1 + t * angle.sin(), 
                        1.0, 
                        Color { r: 0.5, g: 0.0, b: 0.0, a: 1.0 }
                    );
                }
            }
        }
        res
    }
    
    pub fn check_collision(&self, track: &Track) -> bool {
        let back = (
            self.pos.0 - DRAW_SCALE * 15.0 * self.angle.cos(),
            self.pos.1 - DRAW_SCALE * 15.0 * self.angle.sin(),
        );
        let left = (
            back.0 + DRAW_SCALE * 5.0 * (self.angle + PI/2.0).cos(),
            back.1 + DRAW_SCALE * 5.0 * (self.angle + PI/2.0).sin(),
        );
        track.raycast_boundaries(left, self.angle - PI/2.0).unwrap_or(1e6) < DRAW_SCALE * 10.0
        || track.raycast_boundaries(back, self.angle).unwrap_or(1e6) < DRAW_SCALE * 30.0
    }
}