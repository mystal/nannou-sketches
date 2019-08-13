// Based on The Coding Train's Coding Challenge #4: Purple Rain
// https://www.youtube.com/watch?v=KkyIDI6rQJI

// TODO: Consider adding some (rainbow) splashes when they hit the ground?

use nannou::prelude::*;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;
const NUM_DROPS: u32 = 500;

struct Drop {
    x: f32,
    y: f32,
    z: f32,
    yspeed: f32,
    length: f32,
}

impl Drop {
    fn new() -> Self {
        let z = random_range(0.0, 20.0);
        Self {
            x: random_range(-WIDTH / 2.0, WIDTH / 2.0),
            y: random_range(HEIGHT / 2.0 + 50.0, HEIGHT / 2.0 + 500.0),
            z,
            yspeed: map_range(z, 0.0, 20.0, 1.0, 20.0),
            length: map_range(z, 0.0, 20.0, 10.0, 20.0),
        }
    }

    fn fall(&mut self) {
        self.y -= self.yspeed;
        self.yspeed += map_range(self.z, 0.0, 20.0, 0.01, 0.2);

        if self.y < -HEIGHT / 2.0 {
            self.y = random_range(HEIGHT / 2.0 + 100.0, HEIGHT / 2.0 + 200.0);
            self.yspeed = map_range(self.z, 0.0, 20.0, 4.0, 10.0);
        }
    }
}

struct Model {
    drops: Vec<Drop>,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_title("Purple Rain")
        .view(view)
        .build()
        .unwrap();

    Model {
        drops: (0..NUM_DROPS).map(|_| Drop::new()).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for drop in &mut model.drops {
        drop.fall();
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(Rgb::new(230u8, 230, 250));

    for drop in &model.drops {
        draw.line()
            .color(Rgb::new(138u8, 43, 226))
            .stroke_weight(map_range(drop.z, 0.0, 20.0, 1.0, 3.0))
            .start((drop.x, drop.y).into())
            .end((drop.x, drop.y - drop.length).into());
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
