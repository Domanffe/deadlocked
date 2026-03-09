use egui::Ui;

use crate::ui::{
    app::App,
    gui::helpers::{checkbox, color_picker, scroll, section},
};

impl App {
    pub fn hud_settings(&mut self, ui: &mut Ui) {
        let mut grenade_trails = self.config.hud.grenade_trails;
        let grenade_trails_text = self.t("grenade_trails");
        let smoke_text = self.t("smoke");
        let flash_text = self.t("flash");
        let decoy_text = self.t("decoy");
        let molotov_text = self.t("molotov");
        let incendiary_text = self.t("incendiary");
        let he_grenade_text = self.t("he_grenade");
        let hud_colors_text = self.t("hud_colors");
        let text_color_text = self.t("text_color");
        let crosshair_text = self.t("crosshair");

        scroll(ui, "hud_settings", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| self.hud_left(ui));
                cols[1].vertical(|ui| self.hud_right(ui));
            });

            section(ui, grenade_trails_text, Some(&mut grenade_trails), |ui| {
                ui.columns(2, |cols| {
                    let left = &mut cols[0];
                    if color_picker(left, smoke_text, &mut self.config.hud.smoke_trail_color) {
                        self.send_config();
                    }
                    if color_picker(left, flash_text, &mut self.config.hud.flash_trail_color) {
                        self.send_config();
                    }
                    if color_picker(left, decoy_text, &mut self.config.hud.decoy_trail_color) {
                        self.send_config();
                    }

                    let right = &mut cols[1];
                    if color_picker(right, molotov_text, &mut self.config.hud.molotov_trail_color) {
                        self.send_config();
                    }
                    if color_picker(
                        right,
                        incendiary_text,
                        &mut self.config.hud.incendiary_trail_color,
                    ) {
                        self.send_config();
                    }
                    if color_picker(right, he_grenade_text, &mut self.config.hud.he_trail_color) {
                        self.send_config();
                    }
                });
            });

            if self.config.hud.grenade_trails != grenade_trails {
                self.config.hud.grenade_trails = grenade_trails;
                self.send_config();
            }

            section(ui, hud_colors_text, None, |ui| {
                ui.columns(2, |cols| {
                    if color_picker(&mut cols[0], text_color_text, &mut self.config.hud.text_color) {
                        self.send_config();
                    }
                    if color_picker(&mut cols[1], crosshair_text, &mut self.config.hud.crosshair_color)
                    {
                        self.send_config();
                    }
                });
            });
        });
    }

    fn hud_left(&mut self, ui: &mut Ui) {
        let general_text = self.t("general");
        let bomb_timer_text = self.t("bomb_timer");
        let bomb_damage_text = self.t("bomb_damage");
        let spectator_list_text = self.t("spectator_list");
        let keybind_list_text = self.t("keybind_list");
        let fov_circle_text = self.t("fov_circle");
        let sniper_crosshair_text = self.t("sniper_crosshair");

        section(ui, general_text, None, |ui| {
            if checkbox(ui, bomb_timer_text, &mut self.config.hud.bomb_timer) {
                self.send_config();
            }
            if checkbox(ui, bomb_damage_text, &mut self.config.hud.bomb_damage) {
                self.send_config();
            }
            if checkbox(ui, spectator_list_text, &mut self.config.hud.spectator_list) {
                self.send_config();
            }
            if checkbox(ui, keybind_list_text, &mut self.config.hud.keybind_list) {
                self.send_config();
            }
            if checkbox(ui, fov_circle_text, &mut self.config.hud.fov_circle) {
                self.send_config();
            }
            if checkbox(ui, sniper_crosshair_text, &mut self.config.hud.sniper_crosshair) {
                self.send_config();
            }
        });

        let indicators_text = self.t("indicators");
        let fov_arrows_text = self.t("fov_arrows");
        let size_text = self.t("size");
        let radius_text = self.t("radius");
        let hitmarker_text = self.t("hitmarker");
        let hitmarker_color_text = self.t("hitmarker_color");
        let bullet_tracers_text = self.t("bullet_tracers");
        let tracer_color_text = self.t("tracer_color");

        section(ui, indicators_text, None, |ui| {
            if checkbox(ui, fov_arrows_text, &mut self.config.hud.fov_arrows) {
                self.send_config();
            }
            if self.config.hud.fov_arrows {
                ui.add(
                    egui::Slider::new(&mut self.config.hud.arrow_size, 5.0..=30.0).text(size_text),
                );
                ui.add(
                    egui::Slider::new(&mut self.config.hud.arrow_radius, 50.0..=400.0)
                        .text(radius_text),
                );
                self.send_config();
            }

            if checkbox(ui, hitmarker_text, &mut self.config.hud.hitmarker) {
                self.send_config();
            }
            if self.config.hud.hitmarker
                && color_picker(ui, hitmarker_color_text, &mut self.config.hud.hitmarker_color)
            {
                self.send_config();
            }

            if checkbox(ui, bullet_tracers_text, &mut self.config.hud.bullet_tracers) {
                self.send_config();
            }
            if self.config.hud.bullet_tracers
                && color_picker(ui, tracer_color_text, &mut self.config.hud.tracer_color)
            {
                self.send_config();
            }
        });
    }

    fn hud_right(&mut self, ui: &mut Ui) {
        let world_esp_text = self.t("world_esp");
        let dropped_weapons_text = self.t("dropped_weapons");

        section(ui, world_esp_text, None, |ui| {
            if checkbox(ui, dropped_weapons_text, &mut self.config.hud.dropped_weapons) {
                self.send_config();
            }
        });
    }
}
