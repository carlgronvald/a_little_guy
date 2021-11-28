use super::{Friction, Position, Time, TimedLife, Velocity};
use legion::system;

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
