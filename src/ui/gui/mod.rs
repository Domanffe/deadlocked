use egui::{Align, Context};
use crate::utils::log;

use crate::{
    config::{WeaponConfig, write_config},
    message::{Envelope, GameStatus, Message, Target},
    ui::{app::App, color::Colors, gui::aimbot::AimbotTab},
};

pub mod aimbot;
mod config;
mod grenade;
mod helpers;
mod hud;
mod player;
mod radar;
mod r#unsafe;

#[derive(PartialEq)]
pub enum Tab {
    Aimbot,
    Player,
    Hud,
    Radar,
    Grenades,
    Unsafe,
    Config,
}

impl App {
    pub fn send_config(&self) {
        self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
        self.save();
    }

    pub fn send_message(&self, message: Message, target: Target) {
        if self.tx.send(Envelope { target, message }).is_err() {
            std::process::exit(1);
        }
    }

    fn save(&self) {
        write_config(&self.config, &self.current_config);
    }

    fn gui(&mut self, ctx: &Context) {
        ctx.set_pixels_per_point(self.display_scale);

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(160.0)
            .frame(egui::Frame::NONE.fill(Colors::BACKDROP).inner_margin(10.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading(egui::RichText::new("DEADLOCKED").strong().color(Colors::ACCENT).size(20.0));
                    ui.add_space(20.0);
                });

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    self.sidebar_button(ui, Tab::Aimbot, "\u{f04fe}", "Aimbot");
                    self.sidebar_button(ui, Tab::Player, "\u{f0013}", "Player");
                    self.sidebar_button(ui, Tab::Hud, "\u{f0379}", "Hud");
                    self.sidebar_button(ui, Tab::Radar, "\u{f0437}", "Radar");
                    self.sidebar_button(ui, Tab::Grenades, "\u{f0691}", "Grenades");
                    self.sidebar_button(ui, Tab::Unsafe, "\u{f0ce6}", "Unsafe");
                    self.sidebar_button(ui, Tab::Config, "\u{f168b}", "Config");
                });

                ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.button(egui::RichText::new("Report Issue").small()).clicked() {
                        let _ = std::process::Command::new("xdg-open")
                            .arg("https://github.com/Domanffe/deadlocked/issues")
                            .status();
                    }

                    ui.add_space(10.0);
                    let (status_text, status_color) = match self.game_status {
                        GameStatus::Working => ("System Live", Colors::GREEN),
                        GameStatus::NotStarted => ("Waiting for CS2", Colors::YELLOW),
                    };

                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("●").color(status_color)));
                        ui.label(egui::RichText::new(status_text).small().color(Colors::SUBTEXT));
                    });
                    ui.separator();
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Colors::BASE).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    match self.current_tab {
                        Tab::Aimbot => self.aimbot_settings(ui),
                        Tab::Player => self.player_settings(ui),
                        Tab::Hud => self.hud_settings(ui),
                        Tab::Radar => self.radar_settings(ui),
                        Tab::Grenades => self.grenade_settings(ui),
                        Tab::Unsafe => self.unsafe_settings(ui),
                        Tab::Config => self.config_settings(ui, ctx),
                    }
                });
            });
    }

    fn sidebar_button(&mut self, ui: &mut egui::Ui, tab: Tab, icon: &str, label: &str) {
        let is_selected = self.current_tab == tab;
        let color = if is_selected { Colors::WHITE } else { Colors::GRAY };
        let text = egui::RichText::new(format!("{}  {}", icon, label))
            .color(color)
            .size(16.0);

        if ui.add(egui::Button::new(text).selected(is_selected).fill(egui::Color32::TRANSPARENT)).clicked() {
            self.current_tab = tab;
        }
    }

    fn weapon_config(&mut self) -> &mut WeaponConfig {
        if self.aimbot_tab == AimbotTab::Weapon {
            self.config
                .aim
                .weapons
                .get_mut(&self.aimbot_weapon)
                .unwrap()
        } else {
            &mut self.config.aim.global
        }
    }

    pub fn render(&mut self) {
        let self_ptr = self as *mut Self;

        let gui = self.gui.as_mut().unwrap();

        if let Err(err) = gui.make_current() {
            log::error!("could not make gui window current: {err}");
            return;
        }
        gui.run(|ctx| (unsafe { &mut *self_ptr }).gui(ctx));
        gui.clear();
        gui.paint();

        if let Err(err) = gui.swap_buffers() {
            log::error!("could not swap gui window buffers: {err}");
            return;
        }

        let overlay = self.overlay.as_mut().unwrap();

        overlay.window().set_cursor_hittest(false).unwrap();
        if let Err(err) = overlay.make_current() {
            log::error!("could not make overlay window current: {err}");
            return;
        }

        overlay.run(move |egui_ctx| {
            (unsafe { &mut *self_ptr }).overlay(egui_ctx);
        });
        overlay.clear();
        overlay.paint();

        if let Err(err) = overlay.swap_buffers() {
            log::error!("could not swap overlay window buffers: {err}");
        }
    }
}
