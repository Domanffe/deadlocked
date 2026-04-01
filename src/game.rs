use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

use crossbeam::channel::{Receiver, Sender};
use parking_lot::RwLock;
use utils::{log, sync::Mutex};

use crate::{
    config::{CONFIG_PATH, Config, DEFAULT_CONFIG_NAME, SLEEP_DURATION, parse_config},
    cs2::CS2,
    data::Data,
    message::{Envelope, GameStatus, Message, Target},
    os::mouse::Mouse,
    ui::grenades::GrenadeList,
};

pub trait Game {
    fn is_valid(&self) -> bool;

    fn setup(&mut self);
    fn run(&mut self, config: &Config, mouse: &mut Mouse);
    fn data(&self, config: &Config, data: &mut Data);
}

pub struct GameManager {
    tx: Sender<Envelope>,
    rx: Receiver<Message>,
    data: Arc<RwLock<Data>>,
    config: Config,
    mouse: Mouse,
    game: Box<dyn Game>,
}

impl GameManager {
    pub fn new(
        tx: Sender<Envelope>,
        rx: Receiver<Message>,
        data: Arc<RwLock<Data>>,
        grenades: Arc<Mutex<GrenadeList>>,
    ) -> Self {
        let mouse = match Mouse::open() {
            Ok(mouse) => mouse,
            Err(err) => {
                log::error!("failed to open mouse: {}", err);
                std::process::exit(1);
            }
        };

        let mut game = Self {
            tx,
            rx,
            data,
            config: Config::default(),
            mouse,
            game: Box::new(CS2::new(grenades)),
        };

        let config_path = CONFIG_PATH.join(DEFAULT_CONFIG_NAME);
        if config_path.exists() {
            game.config = parse_config(&config_path);
        }

        game
    }

    fn send_game_message(&self, message: Message) -> bool {
        let envelope = Envelope {
            target: Target::Gui,
            message,
        };
        if self.tx.send(envelope).is_err() {
            log::warn!("failed to send game message to gui; stopping game loop");
            return false;
        }
        true
    }

    pub fn run(&mut self) {
        if !self.send_game_message(Message::GameStatus(GameStatus::NotStarted)) {
            return;
        }
        let mut previous_status = GameStatus::NotStarted;
        loop {
            let start = Instant::now();
            while let Ok(message) = self.rx.try_recv() {
                self.parse_message(message);
            }

            let mut is_valid = self.game.is_valid();
            if !is_valid {
                if previous_status == GameStatus::Working {
                    if !self.send_game_message(Message::GameStatus(GameStatus::NotStarted)) {
                        break;
                    }
                    previous_status = GameStatus::NotStarted;
                }
                self.game.setup();
                is_valid = self.game.is_valid();
            }

            if is_valid {
                if previous_status == GameStatus::NotStarted {
                    if !self.send_game_message(Message::GameStatus(GameStatus::Working)) {
                        break;
                    }
                    previous_status = GameStatus::Working;
                }
                self.game.run(&self.config, &mut self.mouse);
                let mut data = self.data.write();
                self.game.data(&self.config, &mut data);
            } else {
                *self.data.write() = Data::default();
            }

            if is_valid {
                let elapsed = start.elapsed();
                let loop_duration = self.loop_duration();
                if elapsed < loop_duration {
                    sleep(loop_duration - elapsed);
                } else {
                    log::debug!(
                        "game loop took {} ms (max {} ms)",
                        elapsed.as_millis(),
                        loop_duration.as_millis()
                    );
                }
            } else {
                sleep(SLEEP_DURATION);
            }
        }
    }

    fn parse_message(&mut self, message: Message) {
        if let Message::Config(config) = message {
            self.config = *config;
        }
    }

    fn loop_duration(&self) -> Duration {
        let fps = self.config.fps.clamp(30, 500) as f32;
        Duration::from_secs_f32(1.0 / fps)
    }
}
