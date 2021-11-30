use std::thread::{self, JoinHandle};

use glm::Vec2;
use legion::*;
use rand::Rng;

use super::{
    controls, external_event_handler, update_lives_system, update_positions_system,
    update_velocities_system, Asset, Friction, Position, Time, Velocity,
};
use crate::{
    channels::{LogicToWindowSender, WindowToLogicReceiver},
    graphics::DrawState,
    logic::TimedLife,
};

use super::state_input_event::*;

use std::time::SystemTime;

pub fn setup_world() -> (World, Entity) {
    println!("Hello, world!");

    let mut world = World::default();

    let player = world.push((
        Position { x: 0.0, y: 0.0 },
        Velocity { dx: 0.0, dy: 0.0 },
        Asset {
            name: "player".into(),
            animation: 0,
            animation_start_time: 0.0,
        },
        Friction {},
    ));

    world.push(
        // The background
        (
            Position { x: 0.0, y: 0.0 },
            Asset {
                name: "background".into(),
                animation: 0,
                animation_start_time: 0.0,
            },
        ),
    );

    world.push(
        // A bush
        (
            Position { x: 100.0, y: 100.0 },
            Asset {
                name: "bush".into(),
                animation: 0,
                animation_start_time: 0.0,
            },
            Friction {},
        ),
    );

    world.push(
        // A bush
        (
            Position {
                x: 480.0,
                y: -540.0,
            },
            Asset {
                name: "lamp post".into(),
                animation: 0,
                animation_start_time: 0.0,
            },
            Friction {},
        ),
    );

    (world, player)
}

pub fn setup_schedule() -> Schedule {
    Schedule::builder()
        .add_system(update_positions_system())
        .add_system(update_velocities_system())
        .add_system(update_lives_system())
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

#[derive(Default)]
///
/// TODO: BAD NAME
/// All the stuff around the player, like if they're looking somewhere or if the camera's shaking or whatever.
struct ExtraInfo {
    shake: f32,
    speed: f32,
    charge: u32,
}

impl ExtraInfo {
    pub fn new() -> Self {
        Self {
            shake: 0.0,
            speed: 16.0,
            ..Default::default()
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

        let first_time = SystemTime::now();

        let mut start_time = SystemTime::now();

        loop {
            resources.insert(Time {
                elapsed_seconds: start_time.elapsed().unwrap().as_secs_f32(),
            });
            start_time = SystemTime::now();

            evh.handle_inputs(&event_receiver);
            let events = evh.tick_events();

            // -----------------------------
            //      Handling user input
            // -----------------------------

            let (mut velocity, position) = if let Some(player_entry) = world.entry(player) {
                (
                    *player_entry.get_component::<Velocity>().unwrap(),
                    *player_entry.get_component::<Position>().unwrap(),
                )
            } else {
                panic!("The player has disappeared!");
            };

            for event in events {
                match event {
                    StateInputEvent::MovePlayerRelative { delta } => {
                        velocity.dx += delta.x * extra_info.speed;
                        velocity.dy += delta.y * extra_info.speed;
                    }
                    StateInputEvent::Jump => extra_info.shake += 5.0,
                    StateInputEvent::Charge(_) => {
                        extra_info.speed = 2.0;
                        if extra_info.charge < 30 {
                            extra_info.charge += 1;
                        }
                    }
                    StateInputEvent::Shoot(direction) => {
                        extra_info.speed = 16.0;
                        if extra_info.charge > 10 {
                            world.push((
                                Asset {
                                    name: "arrow".into(),
                                    animation: 0,
                                    animation_start_time: first_time
                                        .elapsed()
                                        .unwrap()
                                        .as_secs_f32(),
                                },
                                position,
                                Velocity::from(
                                    Vec2::from(direction) * (32.0 * (extra_info.charge as f32)),
                                ),
                                TimedLife { seconds_left: 1.0 },
                            ));
                        }
                        extra_info.charge = 0;
                    }
                }
            }

            if let Some(mut player_entry) = world.entry(player) {
                *player_entry.get_component_mut::<Velocity>().unwrap() = velocity;
                *player_entry.get_component_mut::<Position>().unwrap() = position;
            } else {
                panic!("The player has disappeared!");
            };

            // Do world step
            step(&mut world, &mut schedule, &mut resources);

            let mut q = <(Entity, &TimedLife)>::query();
            let removed_entities: Vec<Entity> = q
                .iter(&world)
                .flat_map(|(entity, time)| {
                    if time.seconds_left <= 0.0 {
                        Some(*entity)
                    } else {
                        None
                    }
                })
                .collect();

            for entity in removed_entities {
                world.remove(entity);
            }

            extra_info.update();

            //TODO: COLLISION

            let draw_positions: Vec<(Asset, Position)> = drawing_query
                .iter(&world)
                .map(|(asset, position)| {
                    (
                        asset.clone(),
                        Position {
                            x: position.x.floor(),
                            y: position.y.floor(),
                        },
                    )
                })
                .collect();
            let _ = graphics_sender.send(DrawState::new(
                draw_positions,
                [
                    rng.gen_range(-1.0..1.0) * extra_info.shake - position.x,
                    rng.gen_range(-1.0..1.0) * extra_info.shake - position.y,
                ],
                first_time.elapsed().unwrap().as_secs_f32(),
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
