use parking_lot::RwLock;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use crossbeam::channel::{Receiver, Sender};
use utils::{log, sync::Mutex};
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
    message::{Envelope, GameStatus, Message},
    ui::{
        grenades::{Grenade, GrenadeList},
        gui::{Tab, aimbot::AimbotTab},
        trail::Trail,
        window_context::WindowContext,
    },
};

const FRAME_RATE: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FRAME_RATE);

pub struct App {
    pub gui: Option<WindowContext>,
    pub overlay: Option<WindowContext>,
    next_frame_time: Instant,

    pub tx: Sender<Envelope>,
    pub rx: Receiver<Message>,
    pub data: Arc<RwLock<Data>>,

    pub game_status: GameStatus,
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
        data: Arc<RwLock<Data>>,
        grenades: Arc<Mutex<GrenadeList>>,
    ) -> Self {
        // read config
        let config = parse_config(&CONFIG_PATH.join(DEFAULT_CONFIG_NAME));
        // override config if invalid
        write_config(&config, &CONFIG_PATH.join(DEFAULT_CONFIG_NAME));

        let mut ret = Self {
            gui: None,
            overlay: None,

            next_frame_time: Instant::now() + FRAME_DURATION,

            tx,
            rx,
            data,
            config,
            current_config: CONFIG_PATH.join(DEFAULT_CONFIG_NAME),
            available_configs: available_configs(),
            new_config_name: String::new(),

            game_status: GameStatus::NotStarted,
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
        ret
    }

    fn create_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(gui) = WindowContext::new(event_loop, false, self.config.accent_color) else {
            log::error!("failed to create gui window context");
            return;
        };
        let Some(overlay) = WindowContext::new(event_loop, true, self.config.accent_color) else {
            log::error!("failed to create overlay window context");
            return;
        };

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
            if let Message::GameStatus(status) = message {
                self.game_status = status;
            }
        }

        let Some(gui_id) = self.gui.as_ref().map(|w| w.window().id()) else {
            return;
        };
        let Some(overlay_id) = self.overlay.as_ref().map(|w| w.window().id()) else {
            return;
        };

        enum WindowTarget {
            Gui,
            Overlay,
        }

        let target = if gui_id == window_id {
            WindowTarget::Gui
        } else if overlay_id == window_id {
            WindowTarget::Overlay
        } else {
            return;
        };

        match &window_event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                match target {
                    WindowTarget::Gui => {
                        if let Some(gui) = &self.gui {
                            gui.resize(*new_size);
                        }
                    }
                    WindowTarget::Overlay => {
                        if let Some(overlay) = &self.overlay {
                            overlay.resize(*new_size);
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                use winit::{event::ElementState, keyboard::Key};
                match key_event.logical_key {
                    Key::Named(winit::keyboard::NamedKey::Control)
                    | Key::Named(winit::keyboard::NamedKey::Shift)
                    | Key::Named(winit::keyboard::NamedKey::Alt) => {
                        let modifiers = self.gui.as_ref().map(|gui| gui.modifiers());
                        if let (Some(modifiers), Some(gui)) = (modifiers, self.gui.as_mut()) {
                            gui.process_modifier(
                                modifiers,
                                key_event.state == ElementState::Pressed,
                                key_event.repeat,
                            );
                        }
                    }
                    _ => {}
                }

                if let Some(gui) = self.gui.as_mut() {
                    let event_response = gui.process_event(&window_event);
                    if event_response.repaint {
                        gui.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                    self.next_frame_time,
                ));
                match target {
                    WindowTarget::Gui => self.render_gui(),
                    WindowTarget::Overlay => self.render_overlay(),
                }
            }
            _ => {
                if let Some(gui) = self.gui.as_mut() {
                    let event_response = gui.process_event(&window_event);
                    if event_response.repaint {
                        gui.request_redraw();
                    }
                }
            }
        }
    }
}
