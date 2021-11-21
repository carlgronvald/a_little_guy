use std::thread::{self, JoinHandle};

use legion::*;

use super::{
    controls, external_event_handler, update_positions_system, update_velocities_system, Asset,
    Player, Position, Time, Velocity,
};
use crate::channels::{LogicToWindowSender, WindowToLogicReceiver};

use std::time::SystemTime;

pub fn setup_world() -> (World, Entity) {
    println!("Hello, world!");

    let mut world = World::default();

    let player = world.push((
        Position { x: 0.0, y: 0.0 },
        Velocity { dx: 0.02, dy: 0.0 },
        Player {},
        Asset {},
    ));

    let mut query = <&Position>::query();

    for position in query.iter(&world) {
        println!("{:?}", position)
    }

    (world, player)
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
        let graphics_sender = tx.render_pack;

        let (mut world, player) = setup_world();
        let mut schedule = setup_schedule();
        let mut resources = setup_resources();

        let mut drawing_query = <(&Asset, &Position)>::query();

        let mut evh =
            external_event_handler::ExternalEventHandler::new(controls::ControlConfig::default());

        loop {
            let start_time = SystemTime::now();

            evh.handle_inputs(&event_receiver);
            let events = evh.tick_events();

            if let Some(mut player_entry) = world.entry(player) {
                for event in events {
                    match event {
                        super::state_input_event::StateInputEvent::MovePlayerRelative { delta } => {
                            let velocity = player_entry.get_component_mut::<Velocity>().unwrap();
                            velocity.dx += delta.x;
                            velocity.dy += delta.y;
                        }
                        _ => {}
                    }
                }
                println!(
                    "Player Position: {:?}",
                    player_entry.get_component::<Position>().unwrap()
                );
            } else {
                panic!("The player has disappeared!");
            }

            step(&mut world, &mut schedule, &mut resources);

            //TODO: COLLISION

            let render_state: Vec<Position> = drawing_query
                .iter(&world)
                .map(|(asset, position)| *position)
                .collect();
            let _ = graphics_sender.send(render_state);

            //TODO: RENDERING

            let end_time = SystemTime::now();
            let tick_duration = end_time.duration_since(start_time).unwrap().as_millis();
            if tick_duration < 33 {
                std::thread::sleep(std::time::Duration::from_millis(
                    (33 - tick_duration) as u64,
                ))
            } else {
                println!("Tick took longer than 33ms!");
            }
        }
    })
}
