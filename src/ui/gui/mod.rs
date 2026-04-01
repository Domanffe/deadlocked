use egui::{Align, Ui};
use utils::log;

use crate::{
    config::{WeaponConfig, write_config},
    message::{Envelope, GameStatus, Message, Target},
    ui::{app::App, color::Colors, gui::aimbot::AimbotTab},
};

pub mod aimbot;
mod config;
mod grenade;
pub mod helpers;
mod hud;
mod player;
mod radar;
pub mod translations;
mod r#unsafe;

use crate::ui::gui::translations::Trans;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn tab_index(tab: Tab) -> f32 {
        match tab {
            Tab::Aimbot => 0.0,
            Tab::Player => 1.0,
            Tab::Hud => 2.0,
            Tab::Radar => 3.0,
            Tab::Grenades => 4.0,
            Tab::Unsafe => 5.0,
            Tab::Config => 6.0,
        }
    }

    pub fn t(&self, key: &str) -> &'static str {
        Trans::get(self.config.language, key)
    }

    pub fn send_config(&mut self) {
        self.active_preset = None;
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

    fn gui(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx().clone();
        ctx.set_pixels_per_point(self.display_scale.clamp(0.75, 2.5));

        let mut style = (*ctx.global_style()).clone();
        style.spacing.item_spacing = egui::vec2(12.0, 12.0);
        style.spacing.window_margin = egui::Margin::same(16);
        style.spacing.button_padding = egui::vec2(12.0, 10.0);
        style.spacing.indent = 16.0;
        style.spacing.interact_size = egui::vec2(120.0, 32.0);

        let widget_radius = egui::CornerRadius::same(8);
        style.visuals.widgets.noninteractive.corner_radius = widget_radius;
        style.visuals.widgets.inactive.corner_radius = widget_radius;
        style.visuals.widgets.hovered.corner_radius = widget_radius;
        style.visuals.widgets.active.corner_radius = widget_radius;
        style.visuals.widgets.open.corner_radius = widget_radius;

        style.visuals.window_corner_radius = egui::CornerRadius::same(12);
        style.visuals.menu_corner_radius = egui::CornerRadius::same(10);
        style.visuals.override_text_color = Some(Colors::TEXT);
        style.visuals.window_fill = Colors::BASE;
        style.visuals.panel_fill = Colors::BASE;
        style.visuals.extreme_bg_color = Colors::BACKDROP;
        style.visuals.faint_bg_color = Colors::HIGHLIGHT;

        style.visuals.widgets.noninteractive.bg_fill = Colors::BASE;
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, Colors::HIGHLIGHT);
        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, Colors::SUBTEXT);

        style.visuals.widgets.inactive.bg_fill = Colors::HIGHLIGHT;
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, Colors::GRAY);
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Colors::TEXT);

        style.visuals.widgets.hovered.bg_fill = Colors::HIGHLIGHT.gamma_multiply(1.12);
        style.visuals.widgets.hovered.bg_stroke =
            egui::Stroke::new(1.0, self.config.accent_color.linear_multiply(0.7));
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Colors::WHITE);
        style.visuals.widgets.hovered.expansion = 0.5;

        style.visuals.widgets.active.bg_fill = self.config.accent_color.linear_multiply(0.28);
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.2, self.config.accent_color);
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Colors::WHITE);
        style.visuals.widgets.active.expansion = 1.0;

        style.visuals.selection.bg_fill = self.config.accent_color.linear_multiply(0.36);
        style.visuals.selection.stroke = egui::Stroke::new(1.0, self.config.accent_color);
        style.visuals.window_shadow.color = egui::Color32::from_black_alpha(175);
        style.visuals.window_shadow.spread = 10;

        ctx.set_global_style(style);

        egui::Panel::left("sidebar")
            .resizable(false)
            .default_size(188.0)
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgba_premultiplied(10, 10, 10, 246))
                    .stroke(egui::Stroke::new(1.0, Colors::HIGHLIGHT))
                    .inner_margin(egui::Margin::same(16)),
            )
            .show_inside(ui, |ui| {
                let stripe_rect = egui::Rect::from_min_size(
                    ui.min_rect().left_top(),
                    egui::vec2(ui.available_width(), 2.0),
                );
                ui.painter()
                    .rect_filled(stripe_rect, 0.0, self.config.accent_color);

                ui.vertical_centered(|ui| {
                    ui.add_space(16.0);
                    ui.heading(
                        egui::RichText::new("DEADLOCKED // 4.7")
                            .strong()
                            .color(self.config.accent_color)
                            .size(20.0),
                    );
                    ui.add_space(16.0);
                });

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    self.sidebar_button(ui, Tab::Aimbot, "\u{f04fe}", self.t("aimbot"));
                    self.sidebar_button(ui, Tab::Player, "\u{f0013}", self.t("player_esp"));
                    self.sidebar_button(ui, Tab::Hud, "\u{f0379}", self.t("hud"));
                    self.sidebar_button(ui, Tab::Radar, "\u{f0437}", self.t("radar"));
                    self.sidebar_button(ui, Tab::Grenades, "\u{f0691}", self.t("grenades"));
                    self.sidebar_button(ui, Tab::Unsafe, "\u{f0ce6}", self.t("unsafe"));
                    self.sidebar_button(ui, Tab::Config, "\u{f168b}", self.t("config"));
                });

                ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                    ui.add_space(12.0);
                    if ui
                        .button(egui::RichText::new(self.t("report_issue")).small())
                        .clicked()
                    {
                        self.open_path("https://github.com/Domanffe/deadlocked/issues");
                    }

                    ui.add_space(12.0);
                    let (status_text, status_color) = match self.game_status {
                        GameStatus::Working => (self.t("system_active"), Colors::GREEN),
                        GameStatus::NotStarted => (self.t("waiting_game"), Colors::YELLOW),
                    };

                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("●").color(status_color),
                        ));
                        ui.label(
                            egui::RichText::new(status_text)
                                .small()
                                .color(Colors::SUBTEXT),
                        );
                    });
                    ui.separator();
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgba_premultiplied(17, 17, 17, 246))
                    .stroke(egui::Stroke::new(1.0, Colors::HIGHLIGHT))
                    .inner_margin(egui::Margin::same(16)),
            )
            .show_inside(ui, |ui| {
                let target_tab = Self::tab_index(self.current_tab);
                let animated_tab = ctx.animate_value_with_time(
                    egui::Id::new("gui_tab_transition"),
                    target_tab,
                    0.16,
                );
                let transition = (1.0 - (target_tab - animated_tab).abs()).clamp(0.0, 1.0);
                let slide_offset = (1.0 - transition) * 10.0;
                let text_alpha = (0.82 + 0.18 * transition).clamp(0.0, 1.0);

                ui.add_space(slide_offset);
                ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color =
                        Some(Colors::TEXT.linear_multiply(text_alpha));
                    ui.vertical(|ui| match self.current_tab {
                        Tab::Aimbot => self.aimbot_settings(ui),
                        Tab::Player => self.player_settings(ui),
                        Tab::Hud => self.hud_settings(ui),
                        Tab::Radar => self.radar_settings(ui),
                        Tab::Grenades => self.grenade_settings(ui),
                        Tab::Unsafe => self.unsafe_settings(ui),
                        Tab::Config => self.config_settings(ui, &ctx),
                    });
                });
            });
    }
    fn open_path(&self, path: &str) {
        let sudo_user = std::env::var("SUDO_USER").unwrap_or_default();
        let sudo_uid = std::env::var("SUDO_UID").unwrap_or_default();

        if !sudo_user.is_empty() {
            let mut cmd = std::process::Command::new("sudo");
            cmd.args(["-u", &sudo_user, "env"]);

            // Mandatory session & DE variables
            let vars = [
                "WAYLAND_DISPLAY",
                "DISPLAY",
                "XDG_CURRENT_DESKTOP",
                "XDG_SESSION_TYPE",
                "XDG_SESSION_DESKTOP",
                "XDG_RUNTIME_DIR",
                "DBUS_SESSION_BUS_ADDRESS",
            ];

            for var in vars {
                if let Ok(val) = std::env::var(var) {
                    cmd.arg(format!("{}={}", var, val));
                }
            }

            if !sudo_uid.is_empty() {
                let runtime = format!("/run/user/{}", sudo_uid);
                if std::path::Path::new(&runtime).exists() {
                    cmd.arg(format!("XDG_RUNTIME_DIR={}", runtime));
                    cmd.arg(format!(
                        "DBUS_SESSION_BUS_ADDRESS=unix:path={}/bus",
                        runtime
                    ));
                }
            }

            cmd.arg(format!("HOME=/home/{}", sudo_user));
            cmd.arg(format!("USER={}", sudo_user));

            let _ = cmd.arg("xdg-open").arg(path).status();
        } else {
            let _ = std::process::Command::new("xdg-open").arg(path).status();
        }
    }

    fn sidebar_button(&mut self, ui: &mut egui::Ui, tab: Tab, icon: &str, label: &str) {
        let is_selected = self.current_tab == tab;

        let color = if is_selected {
            Colors::WHITE
        } else {
            Colors::SUBTEXT
        };
        let text = egui::RichText::new(format!("{}  {}", icon, label))
            .color(color)
            .size(15.0)
            .strong(); // make sidebar text a bit bolder

        let fill = if is_selected {
            self.config.accent_color.linear_multiply(0.20)
        } else {
            egui::Color32::TRANSPARENT
        };

        let button = egui::Button::new(text)
            .fill(fill)
            .stroke(if is_selected {
                egui::Stroke::new(1.0, self.config.accent_color.linear_multiply(0.85))
            } else {
                egui::Stroke::new(1.0, Colors::HIGHLIGHT)
            })
            .min_size(egui::vec2(164.0, 36.0));

        let response = ui.add(button);

        let hover_t = ui.ctx().animate_bool_with_time(
            ui.id().with(("sidebar_hover", label)),
            response.hovered(),
            0.12,
        );
        if hover_t > 0.0 {
            ui.painter().rect_stroke(
                response.rect.expand(0.5),
                egui::CornerRadius::same(8),
                egui::Stroke::new(
                    1.0,
                    self.config.accent_color.linear_multiply(0.35 * hover_t),
                ),
                egui::StrokeKind::Outside,
            );
        }

        if response.clicked() {
            self.current_tab = tab;
        }
        ui.add_space(4.0); // Spacing between buttons
    }

    fn weapon_config(&mut self) -> &mut WeaponConfig {
        if self.aimbot_tab == AimbotTab::Weapon {
            if let Some(weapon) = self.config.aim.weapons.get_mut(&self.aimbot_weapon) {
                weapon
            } else {
                &mut self.config.aim.global
            }
        } else {
            &mut self.config.aim.global
        }
    }

    pub fn render_gui(&mut self) {
        let Some(mut gui) = self.gui.take() else {
            return;
        };

        if let Err(err) = gui.make_current() {
            log::error!("could not make gui window current: {err}");
            self.gui = Some(gui);
            return;
        }
        gui.run(|ui| self.gui(ui));
        gui.clear();
        gui.paint();

        if let Err(err) = gui.swap_buffers() {
            log::error!("could not swap gui window buffers: {err}");
        }

        self.gui = Some(gui);
    }

    pub fn render_overlay(&mut self) {
        let Some(mut overlay) = self.overlay.take() else {
            return;
        };

        if let Err(err) = overlay.window().set_cursor_hittest(false) {
            log::warn!("could not disable overlay cursor hit-test: {err}");
        }
        if let Err(err) = overlay.make_current() {
            log::error!("could not make overlay window current: {err}");
            self.overlay = Some(overlay);
            return;
        }

        {
            let data = self.data.read();
            self.update_window(&overlay, &data);
        }

        overlay.run(|ui| self.overlay(ui.ctx()));
        overlay.clear();
        overlay.paint();

        if let Err(err) = overlay.swap_buffers() {
            log::error!("could not swap overlay window buffers: {err}");
        }

        self.overlay = Some(overlay);
    }
}
