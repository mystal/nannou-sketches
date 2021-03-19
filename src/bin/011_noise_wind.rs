// Inspired by: https://www.reddit.com/r/generative/comments/f6vlg5/how_to_generate_a_wind_field_using_perlin_noise/
// And: https://codepen.io/Mamboleoo/pen/xxGEVXM

use std::f32::consts::TAU;

use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 60;

struct Model {
    noise_grid: Vec<f32>,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title("Noise Wind")
        .view(view)
        .build()
        .unwrap();

    let noise = Perlin::new();
    let grid_size = GRID_WIDTH * GRID_HEIGHT;
    Model {
        noise_grid: vec![1.0; grid_size],
        noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for (j, row) in model.noise_grid.chunks_mut(GRID_WIDTH).enumerate() {
        for (i, value) in row.iter_mut().enumerate() {
            *value = model.noise.get([i as f64 * 0.02, j as f64 * 0.02, app.time as f64 * 0.2]) as f32;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let (w, h) = app.window_rect().w_h();
    let cell_width = w / GRID_WIDTH as f32;
    let cell_height = h / GRID_HEIGHT as f32;
    let left = -w / 2.0 + cell_width / 2.0;
    let top = h / 2.0 - cell_height / 2.0;

    let draw = app.draw();

    draw.background()
        .color(BLACK);

    for (j, row) in model.noise_grid.chunks(GRID_WIDTH).enumerate() {
        for (i, value) in row.iter().enumerate() {
            let cell_center = Vector2::new(left + i as f32 * cell_width, top - j as f32 * cell_height);
            let start = -Vector2::new(cell_width * 0.4, 0.0);
            let end = Vector2::new(cell_width * 0.4, 0.0);

            let alpha = map_range(*value, -1.0, 1.0, 0.0, 1.0);
            let color = Rgba::new(1.0, 1.0, 1.0, alpha);
            let angle = map_range(*value, -1.0, 1.0, 0.0, TAU);
            let draw = draw.xy(cell_center);
            draw.line()
                .stroke_weight(1.5)
                .start(start)
                .end(end)
                .rotate(angle)
                .color(color);
        }
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
