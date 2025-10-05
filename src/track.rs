use std::f32::consts::PI;

use macroquad::prelude::*;

const POINT_COUNT: usize = 20;
const INIT_ANCHORS: [(f32, f32); POINT_COUNT] = [
    (350.0, 200.0),
    (450.0, 200.0),
    (550.0, 150.0),
    (700.0, 250.0),
    (830.0, 180.0),
    (990.0, 330.0),
    (1150.0, 100.0),
    (1330.0, 170.0),
    (1350.0, 420.0),
    (1200.0, 460.0),
    (1150.0, 650.0),
    (980.0, 480.0),
    (860.0, 600.0),
    (780.0, 470.0),
    (710.0, 650.0),
    (600.0, 470.0),
    (530.0, 650.0),
    (400.0, 450.0),
    (150.0, 470.0),
    (125.0, 140.0),
];

#[allow(dead_code)]
pub struct Track {
    anchors: [(f32, f32); POINT_COUNT],
    inner_points: [(f32, f32); POINT_COUNT],
    outer_points: [(f32, f32); POINT_COUNT],
    inner_segments: [((f32, f32), (f32, f32)); POINT_COUNT+1],
    outer_segments: [((f32, f32), (f32, f32)); POINT_COUNT+1],
    pub count: usize,
    pub start_pos: (f32, f32),
}

#[allow(dead_code)]
impl Track {
    pub fn init(track: Option<[(f32, f32); POINT_COUNT]>, width: f32) -> Self {
        let mut t = Track {
            anchors: track.unwrap_or(INIT_ANCHORS),
            inner_points: [(0.0, 0.0); POINT_COUNT],
            outer_points: [(0.0, 0.0); POINT_COUNT],
            inner_segments: [((0.0, 0.0), (0.0, 0.0)); POINT_COUNT+1],
            outer_segments: [((0.0, 0.0), (0.0, 0.0)); POINT_COUNT+1],
            count: POINT_COUNT,
            start_pos: (
                track.unwrap_or(INIT_ANCHORS)[0].0 - 30.0, 
                track.unwrap_or(INIT_ANCHORS)[0].1
            )
        };
        t.get_points(width);
        for i in 0..(POINT_COUNT+1) {
            let next = (i + 1) % POINT_COUNT;
            t.inner_segments[i] = (t.inner_points[i % POINT_COUNT], t.inner_points[next]);
            t.outer_segments[i] = (t.outer_points[i % POINT_COUNT], t.outer_points[next]);
        }
        t
    }
    
    fn get_points(&mut self, width: f32) {
        let mut i = 0;
        for _ in 0..(POINT_COUNT) {
            let prev = ((i as i32 - 1 + POINT_COUNT as i32) % POINT_COUNT as i32) as usize;
            let next = ((i as i32 + 1 + POINT_COUNT as i32) % POINT_COUNT as i32) as usize;
            let dx = (
                self.anchors[next].0 - self.anchors[prev].0,
                self.anchors[i].0 - self.anchors[prev].0,
                self.anchors[next].0 - self.anchors[i].0,
            );
            let dy = (
                self.anchors[next].1 - self.anchors[prev].1,
                self.anchors[i].1 - self.anchors[prev].1,
                self.anchors[next].1 - self.anchors[i].1,
            );
            let angle = dy.0.atan2(dx.0);
            let da = Self::diff_angle(dy.1.atan2(dx.1), dy.2.atan2(dx.2)).abs().powf(0.1);
            let da = 2.0 * da / (1.0 + (dx.0.abs() + dy.0.abs()).powf(0.1));
            self.inner_points[i] = (
                self.anchors[i].0 + width * da * (angle + PI/2.0).cos(),
                self.anchors[i].1 + width * da * (angle + PI/2.0).sin()
            );
            self.outer_points[i] = (
                self.anchors[i].0 + width * da * (angle - PI/2.0).cos(),
                self.anchors[i].1 + width * da * (angle - PI/2.0).sin()
            );
            i = (i + 1) % POINT_COUNT;
        }
    }
    
    pub fn draw(&self) {
        for i in 0..POINT_COUNT {
            draw_text(
                &i.to_string(), 
                self.anchors[i].0 + 10.0, 
                self.anchors[i].1, 20.0, WHITE);
            draw_line(
                self.inner_points[i].0, 
                self.inner_points[i].1, 
                self.inner_points[(i+1) % POINT_COUNT].0,
                self.inner_points[(i+1) % POINT_COUNT].1,
                1.0,
                WHITE
            );
            draw_line(
                self.outer_points[i].0, 
                self.outer_points[i].1, 
                self.outer_points[(i+1) % POINT_COUNT].0,
                self.outer_points[(i+1) % POINT_COUNT].1,
                1.0,
                WHITE
            );
            draw_line(
                self.inner_points[i].0, 
                self.inner_points[i].1, 
                self.outer_points[i].0, 
                self.outer_points[i].1, 
                1.0,
                DARKGRAY 
            );
        };
    }
    
    fn mean_angle(a1: f32, a2: f32) -> f32 {
        let x = (a1.cos() + a2.cos()) / 2.0;
        let y = (a1.sin() + a2.sin()) / 2.0;
        y.atan2(x)
    }
    
    fn diff_angle(a: f32, b: f32) -> f32{
        let diff = (a - b).rem_euclid(2.0 * PI);
        if diff > PI {
            diff - 2.0 * PI
        } else {
            diff
        }
    }
    
    pub fn raycast_boundaries(&self, start: (f32, f32), angle: f32) -> Option<f32> {
        let inner = self._raycast(start, angle, &self.inner_segments);
        let outer = self._raycast(start, angle, &self.outer_segments);
        if inner.is_none() {
            outer
        } else if outer.is_none() {
            inner
        } else {
            Some(inner.unwrap().min(outer.unwrap()))
        }
    }

    
    pub fn raycast_checkpoint(&self, start: (f32, f32), angle: f32, cp: usize) -> Option<f32> {
        let segments = vec![(self.inner_points[cp], self.outer_points[cp])];
        self._raycast(start, angle, &segments)
    }
    
    fn _raycast(
        &self, 
        start: (f32, f32), 
        angle: f32, 
        segments: &[((f32, f32), (f32, f32))]
    ) -> Option<f32>{
        let dir = (angle.cos(), angle.sin());
        let mut closest_t = f32::INFINITY;
        
        for &(a, b) in segments {
            let v = (b.0 - a.0, b.1 - a.1);
            let denom = dir.0 * v.1 - dir.1 * v.0;
            if denom.abs() < f32::EPSILON { continue; }
            let w = (a.0 - start.0, a.1 - start.1);
            let t_ray = (w.0 * v.1 - w.1 * v.0) / denom;
            let t_segment = (w.0 * dir.1 - w.1 * dir.0) / denom;
            if t_ray >= 0.0 && (0.0..=1.0).contains(&t_segment) && t_ray < closest_t {
                    closest_t = t_ray;
                }
        }
        if closest_t == f32::INFINITY { None } else {Some(closest_t)}
    }
}