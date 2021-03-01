// Based on the Processing Flocking example: https://processing.org/examples/flocking.html

use nannou::prelude::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

const INITIAL_BOID_COUNT: u32 = 150;
const BOID_RADIUS: f32 = 2.0;
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

struct Model {
    boids: Vec<Boid>,
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::window::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .size_pixels(WIDTH as u32, HEIGHT as u32)
        .title("Processing Boids")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {
        boids: (0..INITIAL_BOID_COUNT).map(|_| Boid::new(0.0, 0.0)).collect(),
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MousePressed(MouseButton::Left) => {
            let pos = app.mouse.position();
            model.boids.push(Boid::new(pos.x, pos.y));
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.boids.len() {
        let boid = &model.boids[i];

        // Compute flocking accelerations.
        let sepration = {
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

        // Update our physics.
        let boid = &mut model.boids[i];
        let accel = sepration * 1.5 + alignment + cohesion;
        boid.vel += accel;
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

    // Draw boids.
    let (v1, v2, v3) = (
        vec2(2.0 * BOID_RADIUS, 0.0),
        vec2(-2.0 * BOID_RADIUS, -BOID_RADIUS),
        vec2(-2.0 * BOID_RADIUS, BOID_RADIUS),
    );
    for boid in &model.boids {
        draw.tri()
            .color(Rgba::new(200u8, 200, 200, 100))
            .stroke(WHITE)
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
