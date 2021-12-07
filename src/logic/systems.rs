use super::components::*;
use super::Time;
use legion::system;
use rand::prelude::StdRng;
use rand::prelude::ThreadRng;
use rand::Rng;

#[system(for_each)]
pub fn update_positions(pos: &mut Position, vel: &Velocity, #[resource] time: &Time) {
    pos.x += vel.dx * time.elapsed_seconds;
    pos.y += vel.dy * time.elapsed_seconds;
    //println!("{}, {}", pos.x, pos.y);
}

#[system(for_each)]
pub fn update_velocities(vel: &mut Velocity, _friction: &Friction, #[resource] time: &Time) {
    vel.dx *= 0.1f32.powf(time.elapsed_seconds);
    vel.dy *= 0.1f32.powf(time.elapsed_seconds);
}

#[system(for_each)]
pub fn update_lives(life: &mut TimedLife, #[resource] time: &Time) {
    life.seconds_left -= time.elapsed_seconds;
}

#[system(for_each)]
pub fn random_walk_ai(
    velocity: &mut Velocity,
    ai_random_walk: &AiRandomWalk,
    #[resource] time: &Time,
    #[resource] rng: &mut StdRng,
) {
    velocity.dx += rng.gen_range(-1.0..1.0) * time.elapsed_seconds * ai_random_walk.speed;
    velocity.dy += rng.gen_range(-1.0..1.0) * time.elapsed_seconds * ai_random_walk.speed;
}
