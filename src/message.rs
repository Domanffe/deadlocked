use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    Working,
    NotStarted,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatus::Working => write!(f, "Working"),
            GameStatus::NotStarted => write!(f, "Not Started"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Config(Box<Config>),
    GameStatus(GameStatus),
}

#[derive(Clone, Debug)]
pub enum Target {
    Gui,
    Game,
}

#[derive(Clone, Debug)]
pub struct Envelope {
    pub target: Target,
    pub message: Message,
}
