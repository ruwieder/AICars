mod track;
mod car;
mod nn;
mod car_wrapper;

use std::process::exit;
use macroquad::prelude::*;
use rayon::prelude::*;
use crate::car_wrapper::AICar;
use crate::nn::genetic::select_parent;

pub const RAY_COUNT: usize =11;
const RAY_WIDTH: f32 = std::f32::consts::PI * 1.2;
const CAR_COUNT: u16 = 20_000;

fn conf() -> Conf {
    Conf {
        window_title: "AICars".to_string(),
        fullscreen: false,
        window_width: 1920,
        window_height: 1080,
        sample_count: 4,
        high_dpi: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf = "conf")]
async fn main() {
    let track = track::Track::init(None, 100.0);
    let mut aicars = Vec::new();
    for _ in 0..CAR_COUNT { 
        aicars.push(AICar::init()); 
    }
    let mut dt: f32;
    let mut dead_pickup: u8 = 10;
    let mut avg_fitness = 0.0;
    loop {
        dt = get_frame_time();
        clear_background(BLACK);
        aicars.par_iter_mut().for_each(|aicar| {
                aicar.update(&track, dt);
        });
        for i in 0..(CAR_COUNT/2) {
            // aicars[i as usize].update(&track, dt);
            if aicars[i as usize].fitness < avg_fitness {continue;}
            aicars[i as usize].draw();
        }
        avg_fitness = evolution_step(&mut aicars, &mut dead_pickup).unwrap_or(avg_fitness);
        track.draw();
        draw_fps();
        draw_text(&avg_fitness.to_string(), 50.0, 50.0, 20.0, WHITE);
        if is_key_pressed(KeyCode::Escape) { exit(0); }
        next_frame().await
    }
}

fn evolution_step(cars: &mut Vec<AICar>, dead_pickup: &mut u8) -> Option<f32> {
    if *dead_pickup != 0 {
        *dead_pickup -= 1;
        return None;
    };
    let dead_indices: Vec<usize> = cars.par_iter()
            .enumerate()
            .filter_map(|(i, aicar)| {
                if aicar.is_dead { Some(i) } else { None }
            })
            .collect();
    
    let mut total_fitness = 0.0;
    let mut count = 0;
    for i in 0..cars.len() {
        if !cars[i].is_dead{
            count += 1;
            total_fitness += cars[i].fitness;
        }
    }
    let avg_fitness = total_fitness / count as f32;
    for i in dead_indices {
        if cars[i].fitness <= avg_fitness + 800.0 {
            let parent1 = select_parent(cars);
            // let parent2 = select_parent(&cars);
            cars[i] = AICar::new_from_parents(parent1);
        } else {
            cars[i] = AICar::init_with_model(cars[i].model.clone());
        }
    };
    
    Some(avg_fitness)
}