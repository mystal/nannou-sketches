// Based on the Getting Started Processing tutorial:
// https://py.processing.org/tutorials/gettingstarted/

use nannou::prelude::*;

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Draw an ellipse to follow the mouse.
    let color = if app.mouse.buttons.left().is_down() {
        BLACK
    } else {
        WHITE
    };
    // TODO: Is there any way to draw an outline on the ellipse? Like the default ellipse call in
    // Processing does.
    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(80.0)
        .color(color);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::sketch(view);
}
