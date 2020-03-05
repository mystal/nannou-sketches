// Based on:
// https://www.reddit.com/r/proceduralgeneration/comments/f16ml3/isometric_random_walkers/
// https://github.com/ShriRambo/p5Sketches/blob/master/Iso%20random%20walker/sketch.js

use nannou::prelude::*;
use nannou::math::{cgmath, Basis2, Deg, Rad, Rotation2};
use nannou::rand::rand::{
    self,
    seq::SliceRandom,
};
use nannou::winit;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

const NUM_WALKERS: u32 = 500;
const BG_COLOR: u8 = 20;
const WALKER_PALETTE: &[(u8, u8, u8)] = &[
    (0xf3, 0xd6, 0xe9),
    (0xfd, 0xaf, 0x2c),
    (0xff, 0x48, 0x3e),
    (0xe7, 0x05, 0xbe),
    (0x03, 0xa4, 0xff),
];
const TURN_CHANCE: f32 = 0.05;

// Position and direction are in pixel coordinates.
struct IsoWalker {
    pos: Vector2,
    dir: Vector2,
    color: Srgb<u8>,
}

impl IsoWalker {
    fn new() -> Self {
        let x = random_range(-WIDTH / 2.0, WIDTH / 2.0);
        let y = random_range(-HEIGHT / 2.0, HEIGHT / 2.0);
        let first_rot = Basis2::from_angle(Rad(PI / 6.0));
        let second_rot = Basis2::from_angle(Rad(random_range(0, 3) as f32 * TAU / 3.0));
        let dir = second_rot.rotate_vector(first_rot.rotate_vector(cgmath::vec2(2.0, 0.0)));
        let color = WALKER_PALETTE.choose(&mut rand::thread_rng()).unwrap();
        Self {
            pos: vec2(x, y),
            dir: Vector2::from(dir),
            color: Srgb::from_components(color.clone()),
        }
    }
}

struct Model {
    walkers: Vec<IsoWalker>,
}

fn model(app: &App) -> Model {
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .size_pixels(WIDTH as u32, HEIGHT as u32)
        .title("Iso Walkers")
        .view(view)
        .build()
        .unwrap();

    Model {
        walkers: (0..NUM_WALKERS).map(|_| IsoWalker::new()).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for walker in &mut model.walkers {
        walker.pos += walker.dir;

        if random_f32() < TURN_CHANCE {
            let rotation = Basis2::from_angle(Rad(random_range(0, 3) as f32 * TAU / 3.0));
            walker.dir = Vector2::from(rotation.rotate_vector(walker.dir.into()));
        }

        if walker.pos.x < -WIDTH / 2.0 {
            walker.pos.x += WIDTH;
        } else if walker.pos.x > WIDTH / 2.0 {
            walker.pos.x -= WIDTH;
        }
        if walker.pos.y < -HEIGHT / 2.0 {
            walker.pos.y += HEIGHT;
        } else if walker.pos.y > HEIGHT / 2.0 {
            walker.pos.y -= HEIGHT;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() < 1 {
        // Only set background color on the first frame. Afterwards use a transparent rectangle.
        let color = Srgb::new(BG_COLOR, BG_COLOR, BG_COLOR);
        draw.background().color(color);
    } else {
        draw.rect()
            .rgba8(BG_COLOR, BG_COLOR, BG_COLOR, BG_COLOR)
            .w_h(WIDTH, HEIGHT);
    }

    for walker in &model.walkers {
        draw.line()
            .color(walker.color)
            .stroke_weight(3.0)
            .caps_round()
            .start(walker.pos)
            .end(walker.pos + walker.dir);
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
