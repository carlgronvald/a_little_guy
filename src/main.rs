mod logic;

mod channels;
mod graphics;

use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let (game_event_sender, game_event_receiver) = mpsc::channel();
    let window_to_logic_sender = channels::WindowToLogicSender {
        channel_sender: game_event_sender,
    };
    let window_to_logic_receiver = channels::WindowToLogicReceiver {
        channel_receiver: game_event_receiver,
    };

    let render_mutex = Arc::new(Mutex::new(None));
    let logic_to_window_sender = channels::LogicToWindowSender {
        render_pack: render_mutex.clone(),
    };
    let logic_to_window_receiver = channels::LogicToWindowReceiver {
        render_pack: render_mutex,
    };

    let logic_join_handle =
        logic::start_logic_thread(window_to_logic_receiver, logic_to_window_sender);

    graphics::start_window(logic_to_window_receiver, window_to_logic_sender);

    logic_join_handle.join().expect("Panic in logic thread.");
}
