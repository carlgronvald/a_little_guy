mod components;
use std::thread::{self, JoinHandle};

pub use components::{Player, Position, Velocity};

mod resources;
pub use resources::Time;

mod systems;
pub use systems::{update_positions_system, update_velocities_system};

use legion::*;

use crate::channels::{LogicToWindowSender, WindowToLogicReceiver};

pub fn setup_world() -> World {
    println!("Hello, world!");

    let mut world = World::default();

    world.extend(vec![(
        Position { x: 0.0, y: 0.0 },
        Velocity { dx: 0.02, dy: 0.0 },
        Player {},
    )]);

    let mut query = <&Position>::query();

    for position in query.iter(&world) {
        println!("{:?}", position)
    }

    world
}

pub fn setup_schedule() -> Schedule {
    Schedule::builder()
        .add_system(update_positions_system())
        .add_system(update_velocities_system())
        .build()
}

pub fn setup_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(Time {
        elapsed_seconds: 0.0f32,
    });

    resources
}

pub fn step(world: &mut World, schedule: &mut Schedule, resources: &mut Resources) {
    resources.insert(Time {
        elapsed_seconds: 0.05,
    });
    schedule.execute(world, resources)
}

pub fn start_logic_thread(rx: WindowToLogicReceiver, tx: LogicToWindowSender) -> JoinHandle<()> {
    thread::spawn(move || {
        let event_receiver = rx.channel_receiver;
        let graphics_sender_mutex = tx.render_pack;

        let mut world = setup_world();
        let mut schedule = setup_schedule();
        let mut resources = setup_resources();
        let mut player_query = <(&mut Velocity, &mut Position, &Player)>::query();

        loop {
            for (velocity, position, _) in player_query.iter_mut(&mut world) {
                velocity.dx += 0.1 * resources.get::<Time>().unwrap().elapsed_seconds;
                //println!("Player velocity: {}", velocity.dx);
            }

            step(&mut world, &mut schedule, &mut resources)
        }
    })
}
