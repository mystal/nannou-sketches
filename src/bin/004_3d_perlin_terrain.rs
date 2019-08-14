// Based on The Coding Train's Coding Challenge #11: 3D Terrain Generation with Perlin Noise
// https://www.youtube.com/watch?v=IKB1hWWedMk

//use itertools::iproduct;
use nannou::prelude::*;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 600.0;

struct Model {
    cols: u32,
    rows: u32,
    scale: u32,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_title("Perlin Noise Terrain")
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

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let rect_offset = (-WIDTH / 2.0 + model.scale as f32 / 2.0, -HEIGHT / 2.0 + model.scale as f32 / 2.0);

    //for j in 0..model.rows {
    //    for i in 0..model.cols {
    //        let x = (i * model.scale) as f32 + rect_offset.0;
    //        let y = (j * model.scale) as f32 + rect_offset.1;
    //        let w = model.scale as f32;
    //        let h = model.scale as f32;
    //        draw.rect()
    //            .x_y(x, y)
    //            .w_h(w, h)
    //            .no_fill()
    //            .stroke(WHITE);
    //    }
    //}

    for j in 0..model.rows {
        // TODO: Switch back to mesh and just draw a solid mesh in 3D?
        let tris = (0..model.cols)
            .flat_map(|i| {
                let x = (i * model.scale) as f32 + rect_offset.0;
                let y = (j * model.scale) as f32 + rect_offset.1;
                let w = model.scale as f32;
                let h = model.scale as f32;
                geom::Rect::from_x_y_w_h(x, y, w, h).triangles_iter()
            }).map(|tri| tri.map_vertices(|v| v.extend(0.0)));
            //}).flat_map(|tri| tri.vertices().map(|v| (v, srgba(1.0, 1.0, 1.0, 1.0))));
        // Create these rectangles using a mesh of triangles instead?
        // NOTE: Bahhh... Can't draw rotated 3D vertices.
        for tri in tris {
            draw.tri()
                .stroke(WHITE)
                .no_fill()
                .points(tri[0], tri[1], tri[2]);
        }
        // Polyline isn't quite right... I really wanna draw the triangles. Well, let's just
        // do that for now.
        //draw.polyline()
        //    .colored_points(verts);
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
