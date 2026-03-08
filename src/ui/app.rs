use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::utils::{log, sync::Mutex};
use arboard::Clipboard;
use crossbeam::channel::{Receiver, Sender};
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
};

use crate::{
    config::{
        CONFIG_PATH, Config, DEFAULT_CONFIG_NAME, available_configs, parse_config, write_config,
    },
    cs2::entity::weapon::Weapon,
    data::{Data, SoundType},
    message::{Envelope, GameStatus, Message, RadarStatus, Target},
    ui::{
        grenades::{Grenade, GrenadeList},
        gui::{Tab, aimbot::AimbotTab},
        trail::Trail,
        window_context::WindowContext,
    },
};

const FRAME_RATE: u64 = 120;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FRAME_RATE);

pub struct App {
    pub gui: Option<WindowContext>,
    pub overlay: Option<WindowContext>,
    pub clipboard: Clipboard,
    next_frame_time: Instant,

    pub tx: Sender<Envelope>,
    pub rx: Receiver<Message>,
    pub data: Arc<Mutex<Data>>,

    pub game_status: GameStatus,
    pub radar_status: RadarStatus,
    pub display_scale: f32,
    pub trails: HashMap<u64, Trail>,
    pub player_sounds: HashMap<u64, (Instant, SoundType)>,

    pub grenades: Arc<Mutex<GrenadeList>>,
    pub new_grenade: Grenade,
    pub current_grenade: Option<(String, usize)>,

    pub config: Config,
    pub current_config: PathBuf,
    pub available_configs: Vec<PathBuf>,
    pub new_config_name: String,

    pub active_preset: Option<u32>,
    pub current_tab: Tab,
    pub aimbot_tab: AimbotTab,
    pub aimbot_weapon: Weapon,
}

impl App {
    pub fn new(
        tx: Sender<Envelope>,
        rx: Receiver<Message>,
        data: Arc<Mutex<Data>>,
        grenades: Arc<Mutex<GrenadeList>>,
    ) -> Self {
        // read config
        let config = parse_config(&CONFIG_PATH.join(DEFAULT_CONFIG_NAME));
        // override config if invalid
        write_config(&config, &CONFIG_PATH.join(DEFAULT_CONFIG_NAME));

        let mut ret = Self {
            gui: None,
            overlay: None,

            clipboard: Clipboard::new().unwrap(),
            next_frame_time: Instant::now() + FRAME_DURATION,

            tx,
            rx,
            data,
            config,
            current_config: CONFIG_PATH.join(DEFAULT_CONFIG_NAME),
            available_configs: available_configs(),
            new_config_name: String::new(),

            game_status: GameStatus::NotStarted,
            radar_status: RadarStatus::Disconnected,
            display_scale: 1.0,
            trails: HashMap::new(),
            player_sounds: HashMap::new(),

            grenades,
            new_grenade: Grenade::new(),
            current_grenade: None,

            active_preset: None,
            current_tab: Tab::Aimbot,
            aimbot_tab: AimbotTab::Global,
            aimbot_weapon: Weapon::Ak47,
        };
        ret.send_config();
        ret.send_radar_config();
        ret
    }

    fn send_radar_config(&self) {
        self.send_message(
            Message::RadarSetEnabled(self.config.radar.enabled),
            Target::Radar,
        );
        self.send_message(
            Message::ChangeRadarUrl(self.config.radar.url.clone()),
            Target::Radar,
        );
    }

    fn create_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let gui = WindowContext::new(event_loop, false, self.config.accent_color);
        let overlay = WindowContext::new(event_loop, true, self.config.accent_color);

        self.display_scale = gui.window().scale_factor() as f32;
        log::info!("detected display scale: {}", self.display_scale);

        self.gui = Some(gui);
        self.overlay = Some(overlay);
    }
}

impl ApplicationHandler for App {
    fn new_events(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, cause: StartCause) {
        if let StartCause::ResumeTimeReached { .. } = cause {
            if let Some(window) = &self.gui {
                window.window().request_redraw();
            }
            if let Some(window) = &self.overlay {
                window.window().request_redraw();
            }
            self.next_frame_time += FRAME_DURATION;
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop);

        self.next_frame_time = Instant::now() + FRAME_DURATION;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
            self.next_frame_time,
        ));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        window_event: WindowEvent,
    ) {
        while let Ok(message) = self.rx.try_recv() {
            match message {
                Message::GameStatus(status) => self.game_status = status,
                Message::RadarStatus(status) => self.radar_status = status,
                _ => {}
            }
        }

        let Some(gui) = &self.gui else {
            return;
        };
        let Some(overlay) = &self.overlay else {
            return;
        };

        let window = if gui.window().id() == window_id {
            gui
        } else if overlay.window().id() == window_id {
            overlay
        } else {
            return;
        };

        match &window_event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                window.resize(*new_size);
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                use winit::keyboard::{Key, NamedKey};
                match key_event.logical_key {
                    Key::Named(NamedKey::Control)
                    | Key::Named(NamedKey::Shift)
                    | Key::Named(NamedKey::Alt) => {
                        self.gui.as_mut().unwrap().process_modifier();
                    }
                    _ => {}
                }

                let event_response = self.gui.as_mut().unwrap().process_event(&window_event);
                if event_response.repaint {
                    self.gui.as_ref().unwrap().request_redraw();
                    self.overlay.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                    self.next_frame_time,
                ));
                gui.request_redraw();
                overlay.request_redraw();
                self.render();
            }
            _ => {
                let event_response = self.gui.as_mut().unwrap().process_event(&window_event);

                if event_response.repaint {
                    self.gui.as_ref().unwrap().request_redraw();
                    self.overlay.as_ref().unwrap().request_redraw();
                }
            }
        }
    }
}
