// Based on a JavaScript implementation:
// http://slicker.me/javascript/particles.htm

use nannou::prelude::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

const PARTICLE_SIZE: f32 = 4.0;
const MAX_PARTICLES: u32 = 40;
const THRESHOLD: f32 = 100.0;
const THRESHOLD_SQUARE: f32 = THRESHOLD * THRESHOLD;

struct Particle {
    pos: Vector2,
    vel: Vector2,
}

impl Particle {
    fn new() -> Self {
        let x = random_range(-WIDTH / 2.0, WIDTH / 2.0);
        let y = random_range(-HEIGHT / 2.0, HEIGHT / 2.0);
        Self {
            pos: vec2(x, y),
            vel: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
        }
    }
}

struct Model {
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_title("Particle Constellations")
        .view(view)
        .build()
        .unwrap();

    Model {
        particles: (0..MAX_PARTICLES).map(|_| Particle::new()).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for particle in &mut model.particles {
        particle.pos += particle.vel;
        if (particle.pos.x > WIDTH / 2.0) || (particle.pos.x < -WIDTH / 2.0){
            particle.vel.x *= -1.0;
        }
        if (particle.pos.y > HEIGHT / 2.0) || (particle.pos.y < -HEIGHT / 2.0){
            particle.vel.y *= -1.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // Draw lines between particles that are close to each other.
    for (i, particle) in model.particles.iter().enumerate() {
        for other in &model.particles[i + 1..] {
            let dist = particle.pos.distance2(other.pos);
            if dist < THRESHOLD_SQUARE {
                let width = map_range(dist, 0.0, THRESHOLD_SQUARE, 0.0, 3.0);
                let color: u8 = map_range(dist, 50.0, THRESHOLD_SQUARE, 200, 30);
                draw.line()
                    .color(Rgb::new(color, color, color))
                    .stroke_weight(width)
                    .start(particle.pos)
                    .end(other.pos);
            }
        }
    }

    // Draw particles.
    for particle in &model.particles {
        draw.rect()
            .color(WHITE)
            .w_h(PARTICLE_SIZE, PARTICLE_SIZE)
            .x_y(particle.pos.x, particle.pos.y);
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
