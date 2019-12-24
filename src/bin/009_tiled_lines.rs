// Based on Tiled Lines from Generative Artistry:
// https://generativeartistry.com/tutorials/tiled-lines/

use nannou::prelude::*;
use nannou::winit::VirtualKeyCode;
use nannou::Draw;

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 320.0;

const STEP: usize = 20;

struct Model {
    draw_frame: u64,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(1));

    let window_builder = nannou::winit::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_title("Tiled Lines")
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    Model {
        draw_frame: 1,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: VirtualKeyCode) {
    // TODO: Ignore repeat key presses. Ughhh.
    match key {
        VirtualKeyCode::Space => {
            // Set to next frame since it will increment before view is called.
            model.draw_frame = app.elapsed_frames() + 1;
        }
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn draw_line(draw: &Draw, x: f32, y: f32, width: f32, height: f32) {
    // Always draw top down.
    let (start, end) = if random::<bool>() {
        // Go left to right.
        (vec2(x, y), vec2(x + width, y + height))
    } else {
        // Go right to left.
        (vec2(x + width, y), vec2(x, y + height))
    };
    draw.line()
        .start(start)
        .end(end)
        .stroke_weight(3.0)
        .color(BLACK);
}

fn view(app: &App, model: &Model, frame: &Frame) {
    if model.draw_frame != app.elapsed_frames() {
        return;
    }

    let draw = app.draw();

    draw.background()
        .color(WHITE);

    for x in (-WIDTH as i32 / 2..WIDTH as i32 / 2).step_by(STEP) {
        for y in (-HEIGHT as i32 / 2..HEIGHT as i32 / 2).rev().step_by(STEP) {
            draw_line(&draw, x as f32, y as f32, STEP as f32, -(STEP as f32));
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
