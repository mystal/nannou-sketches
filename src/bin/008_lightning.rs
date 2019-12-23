// Let's simulate and draw lightning!
// Inspired by: https://www.reddit.com/r/math/comments/eba6u8/generating_lightning_in_unity_game_engine_using/
// And: https://physics.stackexchange.com/questions/405834/what-determines-the-shape-of-lightning

// TODO: Leaders have a chance to die off. Higher they move the bigger the chance?
// TODO: Want it to look less like a tree. More like one central trunk with small bits coming off.
// Leaders with shorter paths are drawn dimmer and thinner.

use std::time::Instant;

use nannou::prelude::*;
use nannou::math::{cgmath, Basis2, Deg, MetricSpace, Rad, Rotation2};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

const MIN_LIGHTNING_SPEED: f32 = 100.0;
const MAX_LIGHTNING_SPEED: f32 = 250.0;
const MIN_TURN_DIST: f32 = 10.0;
const MAX_TURN_DIST: f32 = 40.0;
const MIN_TURN_DEGREES: f32 = -40.0;
const MAX_TURN_DEGREES: f32 = 40.0;
const GROUND_BIAS_ANGLE: f32 = 30.0;

const SHORT_PATH: usize = 10;
const LONG_PATH: usize = 25;

const LEADER_SPLIT_CHANCE: f32 = 0.02;

struct Leader {
    spawn_time: Instant,
    // Where we've been. The last value is our current position.
    path: Vec<Vector2>,
    // Our movement speed.
    speed: f32,
    // Normalized vector pointing in our current direction.
    dir: Vector2,
    // The distance to go before turning.
    turn_dist: f32,
    // Our parent's index, if we have one.
    parent: Option<usize>,
}

impl Leader {
    fn new(pos: Vector2, rot: f32, parent: Option<usize>) -> Self {
        let rotation = Basis2::from_angle(Deg(rot));
        let dir = rotation.rotate_vector(cgmath::Vector2::unit_x());

        Self {
            spawn_time: Instant::now(),
            path: vec![pos, pos],
            speed: random_range(MIN_LIGHTNING_SPEED, MAX_LIGHTNING_SPEED),
            dir: dir.into(),
            turn_dist: random_range(MIN_TURN_DIST, MAX_TURN_DIST),
            parent,
        }
    }

    fn step(&mut self, dt: f32) {
        let path_len = self.path.len();

        // Move in our current direction.
        let delta_pos = self.dir * (self.speed * dt);
        self.path[path_len - 1] += delta_pos;

        // If we reached our turn_dist, then pick a new direction!
        let last_pos = self.path[path_len - 2];
        if self.pos().distance(last_pos) >= self.turn_dist {
            // Extend the path with our current position.
            self.path.push(self.pos());

            // TODO: Could probably optimize this vector math but whatever.
            // Pick a new turn_dist and direction.
            self.turn_dist = random_range(MIN_TURN_DIST, MAX_TURN_DIST);
            let new_angle = {
                let mut current_angle = self.dir.angle().to_degrees();
                if current_angle < 0.0 {
                    current_angle += 360.0;
                }
                let mut angle_diff_to_down = 270.0 - current_angle;
                if angle_diff_to_down > 180.0 {
                    angle_diff_to_down = -(360.0 - angle_diff_to_down);
                }
                //println!("Current: {} Diff to down: {}", current_angle, angle_diff_to_down);
                // Bias the new angle down toward the ground.
                let bias = angle_diff_to_down.signum() * (angle_diff_to_down / 180.0).powi(2) * GROUND_BIAS_ANGLE;
                current_angle + bias + random_range(MIN_TURN_DEGREES, MAX_TURN_DEGREES)
            };
            let rotation = Basis2::from_angle(Deg(new_angle));
            let dir = rotation.rotate_vector(cgmath::Vector2::unit_x());
            self.dir = dir.into();
        }
    }

    fn pos(&self) -> Vector2 {
        *self.path.last().unwrap()
    }
}

struct Model {
    simulate: bool,

    leaders: Vec<Leader>,
    grounded_leader: Option<usize>,
}

impl Model {
    fn reset(&mut self) {
        self.leaders = vec![
            Leader::new(vec2(0.0, HEIGHT / 2.0), -90.0, None),
        ];
        self.grounded_leader = None;
    }
}

fn model(app: &App) -> Model {
    let window_builder = nannou::winit::WindowBuilder::new()
        .with_resizable(false);
    let _window = app.new_window()
        .window(window_builder)
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_title("Lightning")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {
        simulate: true,

        leaders: vec![
            Leader::new(vec2(0.0, HEIGHT / 2.0), -90.0, None),
        ],
        grounded_leader: None,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::KeyPressed(Key::Space) => {
            model.simulate = !model.simulate;
        }
        WindowEvent::KeyPressed(Key::R) => {
            model.reset();
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if !model.simulate || model.grounded_leader.is_some() {
        return;
    }

    // For each leader, see if we're gonna split.
    let n = model.leaders.len();
    for i in 0..n {
        // If we split, then create a new leader.
        if random_f32() < LEADER_SPLIT_CHANCE {
            // TODO: Save out current leader's path index so we can walk up for the grounded strike.
            let new_leader = Leader::new(
                model.leaders[i].pos(),
                // Branch off in a random direction.
                // TODO: Maybe enforece a minimum angle?
                model.leaders[i].dir.angle().to_degrees() + random_range(MIN_TURN_DEGREES, MAX_TURN_DEGREES),
                Some(i),
            );
            model.leaders.push(new_leader);
        }
    }

    // Step each leader. If one reached the ground (bottom of the screen), then we're done.
    for (i, leader) in model.leaders.iter_mut().enumerate() {
        leader.step(update.since_last.as_secs_f32());
        if leader.pos().y < -HEIGHT / 2.0 {
            model.grounded_leader = Some(i);
            println!("Done after {} iterations", leader.path.len());
            break;
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background()
        //.color(Rgb::new(230u8, 230, 250));
        .color(BLACK);

    // For each leader, draw all their line segments.
    for leader in &model.leaders {
        // TODO: Determine a measure based on spawn time (and path length? maybe total length
        // traveled?)
        let stroke_weight = map_range(clamp(leader.path.len(), SHORT_PATH, LONG_PATH), SHORT_PATH, LONG_PATH, 0.2, 3.0);
        let alpha = map_range(clamp(leader.path.len(), SHORT_PATH, LONG_PATH), SHORT_PATH, LONG_PATH, 0.3, 1.0);
        let color = Rgba::new(1.0, 1.0, 1.0, alpha);

        draw.polyline()
            .caps_square()
            .stroke_weight(stroke_weight)
            // TODO: Can we not clone everything?
            .points(leader.path.iter().cloned())
            .color(color);
    }

    // TODO: For the grounded leader, draw with a thicker stroke all the way back up.
    if let Some(_leader_id) = model.grounded_leader {
    }

    draw.to_frame(app, &frame)
        .unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
