extern crate nalgebra_glm as glm;
mod logic;

mod channels;
mod graphics;

use std::sync::mpsc;

fn main() {
    let (game_event_sender, game_event_receiver) = mpsc::channel();
    let window_to_logic_sender = channels::WindowToLogicSender {
        channel_sender: game_event_sender,
    };
    let window_to_logic_receiver = channels::WindowToLogicReceiver {
        channel_receiver: game_event_receiver,
    };

    let (graphics_sender, graphics_receiver) = mpsc::sync_channel(1);
    let logic_to_window_sender = channels::LogicToWindowSender {
        render_pack: graphics_sender,
    };
    let logic_to_window_receiver = channels::LogicToWindowReceiver {
        render_pack: graphics_receiver,
    };

    let logic_join_handle =
        logic::start_logic_thread(window_to_logic_receiver, logic_to_window_sender);

    graphics::start_window(logic_to_window_receiver, window_to_logic_sender);

    logic_join_handle.join().expect("Panic in logic thread.");
}
