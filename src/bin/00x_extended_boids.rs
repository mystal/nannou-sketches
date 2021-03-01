// Based on the Processing Flocking example: https://processing.org/examples/flocking.html
// Trying to mimic the look and feel of: https://www.youtube.com/watch?v=QbUPfMXXQIY

use nannou::prelude::*;
use nannou::color::Gradient;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

const INITIAL_BOID_COUNT: u32 = 150;
const BOID_RADIUS: f32 = 4.0;
const MAX_SPEED: f32 = 2.0;
const MAX_FORCE: f32 = 0.03;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

struct Boid {
    pos: Vector2,
    vel: Vector2,
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        let angle = random_range(0.0, TWO_PI);
        Self {
            pos: vec2(x, y),
            vel: vec2(angle.cos(), angle.sin()),
        }
    }
}

struct Repel {
    pos: Vector2,
}

impl Repel {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
        }
    }
}

struct Model {
    boids: Vec<Boid>,
    repels: Vec<Repel>,

    enable_separation: bool,
    enable_alignment: bool,
    enable_cohesion: bool,
    enable_repulsion: bool,

    separation_factor: f32,
    alignment_factor: f32,
    cohesion_factor: f32,
    repulsion_factor: f32,
}

impl Model {
    fn new() -> Self {
        Self {
            boids: (0..INITIAL_BOID_COUNT).map(|_| Boid::new(0.0, 0.0)).collect(),
            repels: Vec::new(),

            enable_separation: true,
            enable_alignment: true,
            enable_cohesion: true,
            enable_repulsion: true,

            separation_factor: 1.5,
            alignment_factor: 1.0,
            cohesion_factor: 1.0,
            repulsion_factor: 1.5,
        }
    }
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::window::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .size_pixels(WIDTH as u32, HEIGHT as u32)
        .title("More Boids Fun!")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model::new()
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MousePressed(MouseButton::Left) => {
            let pos = app.mouse.position();
            model.boids.push(Boid::new(pos.x, pos.y));
        }
        WindowEvent::MousePressed(MouseButton::Right) => {
            let pos = app.mouse.position();
            model.repels.push(Repel::new(pos.x, pos.y));
        }
        WindowEvent::KeyPressed(Key::R) => {
            model.boids = (0..INITIAL_BOID_COUNT).map(|_| Boid::new(0.0, 0.0)).collect();
            model.repels = Vec::new();
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Collect forces and apply them after computing them.
    let mut accels = vec![Vector2::zero(); model.boids.len()];

    if model.enable_separation && model.separation_factor > 0.0 {
        for (boid, accel) in model.boids.iter().zip(accels.iter_mut()) {
            let separation = {
                // Try to steer away from nearby boids.
                let desired_separation = 25.0;

                let mut steer = Vector2::zero();
                let mut count = 0;

                // Check if we're too close to all other boids.
                for other in &model.boids {
                    let dist = boid.pos.distance(other.pos);

                    // If we're too close, modify our steering vector.
                    if dist > 0.0 && dist < desired_separation {
                        let diff = (boid.pos - other.pos).normalize() / dist;
                        steer += diff;
                        count += 1;
                    }
                }

                // Average out the steering.
                if count > 0 {
                    steer /= count as f32;
                }

                if !steer.is_zero() {
                    steer = steer.with_magnitude(MAX_SPEED);
                    steer -= boid.vel;
                    steer = steer.limit_magnitude(MAX_FORCE);
                }

                steer
            };
            *accel += separation * model.separation_factor;
        }
    }

    if model.enable_alignment && model.alignment_factor > 0.0 {
        for (boid, accel) in model.boids.iter().zip(accels.iter_mut()) {
            let alignment = {
                // Try to align with nearby boids.
                let neighbor_dist = 50.0;

                let mut sum = Vector2::zero();
                let mut count = 0;

                for other in &model.boids {
                    let dist = boid.pos.distance(other.pos);
                    if dist > 0.0 && dist < neighbor_dist {
                        sum += other.vel;
                        count += 1;
                    }
                }

                if count > 0 {
                    let avg_vel = sum / count as f32;
                    let desired_vel = avg_vel.with_magnitude(MAX_SPEED);
                    (desired_vel - boid.vel).limit_magnitude(MAX_FORCE)
                } else {
                    vec2(0.0, 0.0)
                }
            };
            *accel += alignment * model.alignment_factor;
        }
    }

    if model.enable_cohesion && model.cohesion_factor > 0.0 {
        for (boid, accel) in model.boids.iter().zip(accels.iter_mut()) {
            let cohesion = {
                // Try to move to the center of nearby boids.
                let neighbor_dist = 50.0;

                let mut sum = Vector2::zero();
                let mut count = 0;

                for other in &model.boids {
                    let dist = boid.pos.distance(other.pos);
                    if dist > 0.0 && dist < neighbor_dist {
                        sum += other.pos;
                        count += 1;
                    }
                }

                if count > 0 {
                    let avg_pos = sum / count as f32;
                    let desired = (avg_pos - boid.pos).with_magnitude(MAX_SPEED);
                    (desired - boid.vel).limit_magnitude(MAX_FORCE)
                } else {
                    vec2(0.0, 0.0)
                }
            };
            *accel += cohesion * model.cohesion_factor;
        }
    }

    if model.enable_repulsion && model.repulsion_factor > 0.0 {
        for (boid, accel) in model.boids.iter().zip(accels.iter_mut()) {
            let repulsion = {
                // Try to move away from repel nodes.
                let repel_dist = 50.0;

                let mut sum = Vector2::zero();
                let mut count = 0;

                for repel in &model.repels {
                    let dist = boid.pos.distance(repel.pos);
                    if dist > 0.0 && dist < repel_dist {
                        sum += repel.pos;
                        count += 1;
                    }
                }

                if count > 0 {
                    let avg_pos = sum / count as f32;
                    // Move away from the average position.
                    let desired = -(avg_pos - boid.pos).with_magnitude(MAX_SPEED);
                    (desired - boid.vel).limit_magnitude(MAX_FORCE)
                } else {
                    vec2(0.0, 0.0)
                }
            };
            *accel += repulsion * model.repulsion_factor;
        }
    }

    for (boid, accel) in model.boids.iter_mut().zip(accels.iter()) {
        // Update our physics.
        boid.vel += *accel;
        boid.vel = boid.vel.limit_magnitude(MAX_SPEED);
        boid.pos += boid.vel;

        // Wrap around if we left the window border.
        // Horizontal check.
        if boid.pos.x < -(WIDTH / 2.0) - BOID_RADIUS {
            boid.pos.x = (WIDTH / 2.0) + BOID_RADIUS
        } else if boid.pos.x > (WIDTH / 2.0) + BOID_RADIUS {
            boid.pos.x = -(WIDTH / 2.0) - BOID_RADIUS
        }
        // Vertical check.
        if boid.pos.y < -(HEIGHT / 2.0) - BOID_RADIUS {
            boid.pos.y = (HEIGHT / 2.0) + BOID_RADIUS
        } else if boid.pos.y > (HEIGHT / 2.0) + BOID_RADIUS {
            boid.pos.y = -(HEIGHT / 2.0) - BOID_RADIUS
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(Rgb::new(50u8, 50, 50));

    let neighbor_dist = 50.0;

    let happy_boid_color = Rgba::new(95.0 / 255.0, 219.0 / 255.0, 0.0 / 255.0, 200.0 / 255.0)
        .into_linear();
    let sad_boid_color = Rgba::new(0.0 / 255.0, 146.0 / 255.0, 219.0 / 255.0, 200.0 / 255.0)
        .into_linear();
    let gradient = Gradient::new(vec![sad_boid_color, happy_boid_color]);

    // Draw repel nodes first.
    for repel in &model.repels {
        draw.ellipse()
            .radius(BOID_RADIUS)
            .color(nannou::color::RED)
            .xy(repel.pos);
    }

    // Draw boids.
    let (v1, v2, v3) = (
        vec2(2.0 * BOID_RADIUS, 0.0),
        vec2(-2.0 * BOID_RADIUS, -BOID_RADIUS),
        vec2(-2.0 * BOID_RADIUS, BOID_RADIUS),
    );
    for boid in &model.boids {
        let neighbor_count = model.boids.iter()
            .filter(|other| boid.pos.distance(other.pos) < neighbor_dist)
            .count();
        let gradient_color = map_range(neighbor_count as f32, 0.0, 8.0, 0.0, 1.0);
        draw.tri()
            .color(gradient.get(gradient_color))
            .points(v1, v2, v3)
            .xy(boid.pos)
            .rotate(boid.vel.angle());
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
