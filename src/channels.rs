use crate::{
    graphics::{DrawState, ExternalEvent},
    logic::Position,
};
//use graphics::RenderMessages;
//use graphics::GraphicsCapabilities;
//use game::GraphicsStateModel;
use std::sync::{mpsc, Arc, Mutex};

pub struct WindowToLogicSender {
    pub channel_sender: mpsc::Sender<ExternalEvent>,
}

pub struct WindowToLogicReceiver {
    pub channel_receiver: mpsc::Receiver<ExternalEvent>,
}

pub struct LogicToWindowSender {
    pub render_pack: mpsc::SyncSender<DrawState>,
}

pub struct LogicToWindowReceiver {
    pub render_pack: mpsc::Receiver<DrawState>,
}
