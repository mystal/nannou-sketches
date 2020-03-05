// A nice pastel background.
// Pastel circles that appear and shrink over time.
// TODO: Try changing speed, transparency, delay in spawn?

use nannou::prelude::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const NUM_CIRCLES: u32 = 20;

struct PastelCircle {
    x: f32,
    y: f32,
    radius: f32,
    hue: f32,
    saturation: f32,
    value: f32,
    alpha: f32,
}

impl PastelCircle {
    fn new() -> Self {
        Self {
            x: random_range(-WIDTH / 2.0, WIDTH / 2.0),
            y: random_range(-HEIGHT / 2.0, HEIGHT / 2.0),
            radius: random_range(20.0, 80.0),
            hue: random(),
            saturation: random_range(0.2, 0.5),
            value: random_range(0.7, 1.0),
            alpha: random_range(0.5, 0.8),
        }
    }
}

struct Model {
    bg_hue: f32,
    bg_saturation: f32,
    bg_value: f32,
    //spawn_timer: f32,

    circles: Vec<PastelCircle>,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::window::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .size_pixels(WIDTH as u32, HEIGHT as u32)
        .title("Pastel Circles")
        .view(view)
        .build()
        .unwrap();

    Model {
        bg_hue: random(),
        bg_saturation: 0.1,
        bg_value: 0.95,
        //spawn_timer: app.time + 1.0 / CIRCLES_SPAWNED_PER_SECOND,

        circles: (0..NUM_CIRCLES).map(|_| PastelCircle::new()).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for circle in &mut model.circles {
        circle.radius -= 0.2;
        if circle.radius < 1.0 {
            // TODO: Delay respawning the circle some random amount of time?
            *circle = PastelCircle::new();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background()
        .hsv(model.bg_hue, model.bg_saturation, model.bg_value);

    for circle in &model.circles {
        draw.ellipse()
            .radius(circle.radius)
            .hsva(circle.hue, circle.saturation, circle.value, circle.alpha)
            .x_y(circle.x, circle.y);
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
