use std::thread::{self, JoinHandle};

use legion::*;
use rand::Rng;

use super::{
    controls, external_event_handler, update_positions_system, update_velocities_system, Asset,
    Player, Position, Time, Velocity,
};
use crate::{
    channels::{LogicToWindowSender, WindowToLogicReceiver},
    graphics::DrawState,
};

use std::time::SystemTime;

pub fn setup_world() -> (World, Entity) {
    println!("Hello, world!");

    let mut world = World::default();

    let player = world.push((
        Position { x: 0.0, y: 0.0 },
        Velocity { dx: 0.02, dy: 0.0 },
        Player {},
        Asset { name : "player".into() },
    ));

    world.push( // A bush
        (
            Position { x: 100.0, y: 100.0},
            Asset { name : "bush".into() }
        )
    );

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
        elapsed_seconds: 0.033,
    });
    schedule.execute(world, resources)
}

///
/// TODO: BAD NAME
/// All the stuff around the player, like if they're looking somewhere or if the camera's shaking or whatever.
struct ExtraInfo {
    shake: f32,
    speed: f32,
}

impl ExtraInfo {
    pub fn new() -> Self {
        Self {
            shake: 0.0,
            speed: 16.0,
        }
    }
    pub fn update(&mut self) {
        self.shake *= 0.98;
    }
}

pub fn start_logic_thread(rx: WindowToLogicReceiver, tx: LogicToWindowSender) -> JoinHandle<()> {
    thread::spawn(move || {
        let event_receiver = rx.channel_receiver;
        let graphics_sender = tx.render_pack;

        let (mut world, player) = setup_world();
        let mut schedule = setup_schedule();
        let mut resources = setup_resources();

        let mut drawing_query = <(&Asset, &Position)>::query();

        let mut extra_info = ExtraInfo::new();

        let mut evh =
            external_event_handler::ExternalEventHandler::new(controls::ControlConfig::default());

        let mut rng = rand::thread_rng();

        loop {
            let start_time = SystemTime::now();

            evh.handle_inputs(&event_receiver);
            let events = evh.tick_events();

            // -----------------------------
            //      Handling user input
            // -----------------------------
            if let Some(mut player_entry) = world.entry(player) {
                for event in events {
                    match event {
                        super::state_input_event::StateInputEvent::MovePlayerRelative { delta } => {
                            let velocity = player_entry.get_component_mut::<Velocity>().unwrap();
                            velocity.dx += delta.x * extra_info.speed;
                            velocity.dy += delta.y * extra_info.speed;
                        }
                        super::state_input_event::StateInputEvent::Jump => extra_info.shake += 5.0,
                        super::state_input_event::StateInputEvent::Charge(_) => {
                            extra_info.speed = 2.0;
                        }
                        super::state_input_event::StateInputEvent::Shoot(_) => {
                            extra_info.speed = 16.0;
                        }
                        _ => {}
                    }
                }
            } else {
                panic!("The player has disappeared!");
            }

            // Do world step
            step(&mut world, &mut schedule, &mut resources);
            extra_info.update();

            //TODO: COLLISION

            let draw_positions: Vec<(Asset, Position)> = drawing_query
                .iter(&world)
                .map(|(asset, position)| (asset.clone(), *position))
                .collect();
            let _ = graphics_sender.send(DrawState::new(
                draw_positions,
                [
                    rng.gen_range(-1.0..1.0) * extra_info.shake,
                    rng.gen_range(-1.0..1.0) * extra_info.shake,
                ],
            ));

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
