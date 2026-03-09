use egui::{DragValue, Ui};

use crate::ui::{
    app::App,
    gui::helpers::{
        checkbox, checkbox_hover, color_picker, combo_box, drag, keybind, scroll, section,
    },
};

impl App {
    pub fn player_settings(&mut self, ui: &mut Ui) {
        scroll(ui, "player", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| {
                    self.player_left(ui);
                });
                cols[1].vertical(|ui| {
                    self.player_right(ui);
                });
            });

            section(ui, self.t("colors"), None, |ui| {
                ui.columns(2, |cols| {
                    let left = &mut cols[0];
                    let mut box_visible_color = self.config.player.box_visible_color;
                    if color_picker(left, self.t("box_visible"), &mut box_visible_color) {
                        self.config.player.box_visible_color = box_visible_color;
                        self.send_config();
                    }

                    let mut box_invisible_color = self.config.player.box_invisible_color;
                    if color_picker(left, self.t("box_invisible"), &mut box_invisible_color) {
                        self.config.player.box_invisible_color = box_invisible_color;
                        self.send_config();
                    }

                    let right = &mut cols[1];
                    let mut skeleton_visible_color = self.config.player.skeleton_visible_color;
                    if color_picker(
                        right,
                        self.t("skeleton_visible"),
                        &mut skeleton_visible_color,
                    ) {
                        self.config.player.skeleton_visible_color = skeleton_visible_color;
                        self.send_config();
                    }

                    let mut skeleton_invisible_color = self.config.player.skeleton_invisible_color;
                    if color_picker(
                        right,
                        self.t("skeleton_invisible"),
                        &mut skeleton_invisible_color,
                    ) {
                        self.config.player.skeleton_invisible_color = skeleton_invisible_color;
                        self.send_config();
                    }
                });
            });
        });
    }

    fn player_left(&mut self, ui: &mut Ui) {
        let mut enabled = self.config.player.enabled;
        section(ui, self.t("player_esp"), Some(&mut enabled), |ui| {
            if keybind(
                ui,
                "esp_hotkey",
                self.t("hot_toggle"),
                &mut self.config.player.esp_hotkey,
            ) {
                self.send_config();
            }

            if checkbox_hover(
                ui,
                self.t("show_team"),
                self.t("show_team_desc"),
                &mut self.config.player.show_friendlies,
            ) {
                self.send_config();
            }

            if combo_box(
                ui,
                "draw_box",
                self.t("box_style"),
                &mut self.config.player.draw_box,
            ) {
                self.send_config();
            }

            if combo_box(
                ui,
                "box_mode",
                self.t("box_fill"),
                &mut self.config.player.box_mode,
            ) {
                self.send_config();
            }

            if drag(
                ui,
                self.t("box_thickness"),
                egui::DragValue::new(&mut self.config.player.box_thickness)
                    .range(0.1..=5.0)
                    .speed(0.1),
            ) {
                self.send_config();
            }
        });

        if self.config.player.enabled != enabled {
            self.config.player.enabled = enabled;
            self.send_config();
        }

        section(ui, self.t("draw_skeleton"), None, |ui| {
            if combo_box(
                ui,
                "draw_skeleton",
                self.t("mode"),
                &mut self.config.player.draw_skeleton,
            ) {
                self.send_config();
            }

            if drag(
                ui,
                self.t("thickness"),
                egui::DragValue::new(&mut self.config.player.skeleton_thickness)
                    .range(0.1..=5.0)
                    .speed(0.1),
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                self.t("head_circle"),
                &mut self.config.player.head_circle,
            ) {
                self.send_config();
            }

            if checkbox_hover(
                ui,
                self.t("visible_only"),
                self.t("visible_only_desc"),
                &mut self.config.player.visible_only,
            ) {
                self.send_config();
            }
        });

        section(ui, self.t("snaplines"), None, |ui| {
            if checkbox(ui, self.t("enabled"), &mut self.config.player.snaplines) {
                self.send_config();
            }
            if self.config.player.snaplines {
                if combo_box(
                    ui,
                    "snapline_start",
                    self.t("snapline_start"),
                    &mut self.config.player.snapline_start,
                ) {
                    self.send_config();
                }
                if color_picker(
                    ui,
                    self.t("snapline_color"),
                    &mut self.config.player.snapline_color,
                ) {
                    self.send_config();
                }
            }
        });

        let mut sound_enabled = self.config.player.sound.enabled;
        section(ui, self.t("sound_esp"), Some(&mut sound_enabled), |ui| {
            if drag(
                ui,
                self.t("fadeout"),
                DragValue::new(&mut self.config.player.sound.fadeout_duration)
                    .range(0.0..=10.0)
                    .speed(0.01),
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                self.t("show_visible"),
                &mut self.config.player.sound.show_visible,
            ) {
                self.send_config();
            }

            ui.add_space(4.0);
            ui.label(self.t("detection_ranges"));
            ui.horizontal(|ui| {
                if ui
                    .add(
                        DragValue::new(&mut self.config.player.sound.footstep_diameter)
                            .speed(10.0)
                            .range(200.0..=6000.0),
                    )
                    .changed()
                {
                    self.send_config();
                }
                ui.label(self.t("footstep"));
            });
            ui.horizontal(|ui| {
                if ui
                    .add(
                        DragValue::new(&mut self.config.player.sound.gunshot_diameter)
                            .speed(10.0)
                            .range(200.0..=10000.0),
                    )
                    .changed()
                {
                    self.send_config();
                }
                ui.label(self.t("gunshot"));
            });
        });

        if self.config.player.sound.enabled != sound_enabled {
            self.config.player.sound.enabled = sound_enabled;
            self.send_config();
        }
    }

    fn player_right(&mut self, ui: &mut Ui) {
        section(ui, self.t("information"), None, |ui| {
            if checkbox(
                ui,
                self.t("player_name"),
                &mut self.config.player.player_name,
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                self.t("weapon_icon"),
                &mut self.config.player.weapon_icon,
            ) {
                self.send_config();
            }

            if checkbox(ui, self.t("tags"), &mut self.config.player.tags) {
                self.send_config();
            }
        });

        section(ui, self.t("vitals"), None, |ui| {
            if checkbox(ui, self.t("health_bar"), &mut self.config.player.health_bar) {
                self.send_config();
            }

            if checkbox(ui, self.t("armor_bar"), &mut self.config.player.armor_bar) {
                self.send_config();
            }
        });
    }
}
