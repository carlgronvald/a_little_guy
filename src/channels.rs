use crate::{
    graphics::{DrawState, ExternalEvent},
};

use std::sync::{mpsc};

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
