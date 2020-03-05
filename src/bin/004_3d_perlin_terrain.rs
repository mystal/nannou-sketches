// Based on The Coding Train's Coding Challenge #11: 3D Terrain Generation with Perlin Noise
// https://www.youtube.com/watch?v=IKB1hWWedMk

use nannou::prelude::*;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 600.0;

struct Model {
    cols: u32,
    rows: u32,
    scale: u32,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::window::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .size_pixels(WIDTH as u32, HEIGHT as u32)
        .title("Perlin Noise Terrain")
        .view(view)
        .build()
        .unwrap();

    let scale = 20;

    Model {
        cols: 600 / scale,
        rows: 600 / scale,
        scale,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let rect_offset = (-WIDTH / 2.0 + model.scale as f32 / 2.0, -HEIGHT / 2.0 + model.scale as f32 / 2.0);

    for j in 0..model.rows {
        for i in 0..model.cols {
            let x = (i * model.scale) as f32 + rect_offset.0;
            let y = (j * model.scale) as f32 + rect_offset.1;
            draw.rect()
                .x_y(x, y)
                .w_h(model.scale as f32, model.scale as f32)
                .no_fill()
                .stroke(WHITE);
            // TODO: Create these rectangles using a mesh of triangles instead.
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
