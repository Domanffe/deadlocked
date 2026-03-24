use crossbeam::channel::{Receiver, Sender};

use crate::message::{Envelope, Message, Target};

pub fn router(rx: Receiver<Envelope>, tx_gui: Sender<Message>, tx_game: Sender<Message>) {
    while let Ok(envelope) = rx.recv() {
        if match envelope.target {
            Target::Gui => tx_gui.send(envelope.message),
            Target::Game => tx_game.send(envelope.message),
        }
        .is_err()
        {
            utils::log::warn!("router channel closed; stopping router thread");
            break;
        }
    }
}
