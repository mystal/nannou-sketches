// Based on The Coding Train's Coding Challenge #1: Starfield
// https://www.youtube.com/watch?v=17WoOqgXsRM

use nannou::prelude::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
}

impl Star {
    fn new(x_range: f32, y_range: f32) -> Self {
        let z = random_range(0.0, x_range);
        Self {
            x: random_range(-x_range, x_range),
            y: random_range(-y_range, y_range),
            z,
            pz: z,
        }
    }
}

struct Model {
    stars: Vec<Star>,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::window::WindowBuilder::new()
        .with_resizable(false);
    let window = app.new_window()
        .window(window_builder)
        .size_pixels(WIDTH as u32, HEIGHT as u32)
        .title("Starfield")
        .view(view)
        .build()
        .unwrap();

    Model {
        stars: (0..400).map(|_| Star::new(WIDTH, HEIGHT)).collect(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let speed = map_range(app.mouse.x, -WIDTH / 2.0, WIDTH / 2.0, 0.0, 40.0);
    for star in &mut model.stars {
        star.pz = star.z;
        star.z -= speed;
        if star.z < 1.0 {
            star.z = WIDTH as f32;
            star.pz = star.z;
            star.x = random_range(-WIDTH / 2.0, WIDTH / 2.0);
            star.y = random_range(-HEIGHT / 2.0, HEIGHT / 2.0);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let win = app.window_rect();

    for star in &model.stars {
        let sx = map_range(star.x / star.z, 0.0, 1.0, 0.0, WIDTH);
        let sy = map_range(star.y / star.z, 0.0, 1.0, 0.0, HEIGHT);
        let r = map_range(star.z, 0.0, WIDTH, 8.0, 0.0);
        draw.ellipse()
            .x_y(sx, sy)
            .color(WHITE)
            .radius(r);

        let px = map_range(star.x / star.pz, 0.0, 1.0, 0.0, WIDTH);
        let py = map_range(star.y / star.pz, 0.0, 1.0, 0.0, HEIGHT);

        draw.line()
            .color(WHITE)
            .start((px, py).into())
            .end((sx, sy).into());
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
