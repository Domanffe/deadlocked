use egui::Ui;

use crate::ui::{
    app::App,
    gui::helpers::{checkbox, color_picker, scroll, section},
};

impl App {
    pub fn hud_settings(&mut self, ui: &mut Ui) {
        let mut grenade_trails = self.config.hud.grenade_trails;
        scroll(ui, "hud_settings", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| self.hud_left(ui));
                cols[1].vertical(|ui| self.hud_right(ui));
            });

            section(ui, "Grenade Trails", Some(&mut grenade_trails), |ui| {
                ui.columns(2, |cols| {
                    let left = &mut cols[0];
                    if color_picker(left, "Smoke", &mut self.config.hud.smoke_trail_color) { self.send_config(); }
                    if color_picker(left, "Flash", &mut self.config.hud.flash_trail_color) { self.send_config(); }
                    if color_picker(left, "Decoy", &mut self.config.hud.decoy_trail_color) { self.send_config(); }

                    let right = &mut cols[1];
                    if color_picker(right, "Molotov", &mut self.config.hud.molotov_trail_color) { self.send_config(); }
                    if color_picker(right, "Incendiary", &mut self.config.hud.incendiary_trail_color) { self.send_config(); }
                    if color_picker(right, "HE Grenade", &mut self.config.hud.he_trail_color) { self.send_config(); }
                });
            });

            if self.config.hud.grenade_trails != grenade_trails {
                self.config.hud.grenade_trails = grenade_trails;
                self.send_config();
            }

            section(ui, "Global HUD Colors", None, |ui| {
                ui.columns(2, |cols| {
                    if color_picker(&mut cols[0], "Text Color", &mut self.config.hud.text_color) { self.send_config(); }
                    if color_picker(&mut cols[1], "Crosshair", &mut self.config.hud.crosshair_color) { self.send_config(); }
                });
            });
        });
    }

    fn hud_left(&mut self, ui: &mut Ui) {
        section(ui, "General", None, |ui| {
            if checkbox(ui, "Bomb Timer", &mut self.config.hud.bomb_timer) { self.send_config(); }
            if checkbox(ui, "FOV Circle", &mut self.config.hud.fov_circle) { self.send_config(); }
            if checkbox(ui, "Sniper Crosshair", &mut self.config.hud.sniper_crosshair) { self.send_config(); }
        });
    }

    fn hud_right(&mut self, ui: &mut Ui) {
        section(ui, "World ESP", None, |ui| {
            if checkbox(ui, "Dropped Weapons", &mut self.config.hud.dropped_weapons) { self.send_config(); }
        });
    }
}
