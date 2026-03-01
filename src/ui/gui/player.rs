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

            section(ui, "Colors", None, |ui| {
                ui.columns(2, |cols| {
                    let left = &mut cols[0];
                    let mut box_visible_color = self.config.player.box_visible_color;
                    if color_picker(left, "Box (visible)", &mut box_visible_color) {
                        self.config.player.box_visible_color = box_visible_color;
                        self.send_config();
                    }

                    let mut box_invisible_color = self.config.player.box_invisible_color;
                    if color_picker(left, "Box (invisible)", &mut box_invisible_color) {
                        self.config.player.box_invisible_color = box_invisible_color;
                        self.send_config();
                    }

                    let right = &mut cols[1];
                    let mut skeleton_color = self.config.player.skeleton_color;
                    if color_picker(right, "Skeleton", &mut skeleton_color) {
                        self.config.player.skeleton_color = skeleton_color;
                        self.send_config();
                    }
                });
            });
        });
    }

    fn player_left(&mut self, ui: &mut Ui) {
        let mut enabled = self.config.player.enabled;
        section(ui, "ESP", Some(&mut enabled), |ui| {
            if keybind(
                ui,
                "esp_hotkey",
                "Hot-Toggle",
                &mut self.config.player.esp_hotkey,
            ) {
                self.send_config();
            }

            if checkbox_hover(
                ui,
                "Show Team",
                "Show friendlies in custom modes",
                &mut self.config.player.show_friendlies,
            ) {
                self.send_config();
            }

            if combo_box(ui, "draw_box", "Box Style", &mut self.config.player.draw_box) {
                self.send_config();
            }

            if combo_box(ui, "box_mode", "Box Fill", &mut self.config.player.box_mode) {
                self.send_config();
            }
        });

        if self.config.player.enabled != enabled {
            self.config.player.enabled = enabled;
            self.send_config();
        }

        section(ui, "Skeleton", None, |ui| {
            if combo_box(
                ui,
                "draw_skeleton",
                "Draw Bones",
                &mut self.config.player.draw_skeleton,
            ) {
                self.send_config();
            }

            if checkbox(ui, "Head Circle", &mut self.config.player.head_circle) {
                self.send_config();
            }

            if checkbox_hover(
                ui,
                "Visible Only",
                "Hide players behind walls",
                &mut self.config.player.visible_only,
            ) {
                self.send_config();
            }
        });

        let mut sound_enabled = self.config.player.sound.enabled;
        section(ui, "Sound ESP", Some(&mut sound_enabled), |ui| {
            if drag(
                ui,
                "Fadeout (s)",
                DragValue::new(&mut self.config.player.sound.fadeout_duration)
                    .range(0.0..=10.0)
                    .speed(0.01),
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                "Show Visible",
                &mut self.config.player.sound.show_visible,
            ) {
                self.send_config();
            }

            ui.add_space(4.0);
            ui.label("Detection Ranges:");
            ui.horizontal(|ui| {
                if ui.add(DragValue::new(&mut self.config.player.sound.footstep_diameter).speed(10.0).range(200.0..=6000.0)).changed() { self.send_config(); }
                ui.label("Footstep");
            });
            ui.horizontal(|ui| {
                if ui.add(DragValue::new(&mut self.config.player.sound.gunshot_diameter).speed(10.0).range(200.0..=10000.0)).changed() { self.send_config(); }
                ui.label("Gunshot");
            });
        });

        if self.config.player.sound.enabled != sound_enabled {
            self.config.player.sound.enabled = sound_enabled;
            self.send_config();
        }
    }

    fn player_right(&mut self, ui: &mut Ui) {
        section(ui, "Information", None, |ui| {
            if checkbox(ui, "Name", &mut self.config.player.player_name) {
                self.send_config();
            }

            if checkbox(ui, "Weapon Icon", &mut self.config.player.weapon_icon) {
                self.send_config();
            }

            if checkbox(ui, "Show Tags", &mut self.config.player.tags) {
                self.send_config();
            }
        });

        section(ui, "Vitals", None, |ui| {
            if checkbox(ui, "Health Bar", &mut self.config.player.health_bar) {
                self.send_config();
            }

            if checkbox(ui, "Armor Bar", &mut self.config.player.armor_bar) {
                self.send_config();
            }
        });
    }
}
