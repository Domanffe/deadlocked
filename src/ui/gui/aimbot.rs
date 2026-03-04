use egui::{DragValue, Ui};
use strum::IntoEnumIterator as _;

use crate::{
    cs2::bones::Bones,
    ui::{
        app::App,
        gui::helpers::{checkbox, checkbox_hover, combo_box, drag, keybind, scroll, section},
    },
};

#[derive(PartialEq)]
pub enum AimbotTab {
    Global,
    Weapon,
}

impl App {
    pub fn aimbot_settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.aimbot_tab, AimbotTab::Global, "Global Settings");
            ui.add_space(8.0);
            ui.selectable_value(&mut self.aimbot_tab, AimbotTab::Weapon, "Weapon Overrides");

            if self.aimbot_tab == AimbotTab::Weapon {
                ui.add_space(8.0);
                combo_box(ui, "aimbot_weapon", "", &mut self.aimbot_weapon);
            }
        });
        ui.add_space(10.0);

        scroll(ui, "aimbot_main", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| self.aimbot_left(ui));
                cols[1].vertical(|ui| self.aimbot_right(ui));
            });
        });
    }

    fn aimbot_left(&mut self, ui: &mut Ui) {
        let is_override = self.aimbot_tab == AimbotTab::Weapon;
        let mut enabled = self.weapon_config().aimbot.enabled;

        section(ui, "Aimbot", Some(&mut enabled), |ui| {
            if is_override {
                if checkbox(ui, "Override Global", &mut self.weapon_config().aimbot.enable_override) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            if keybind(ui, "aimbot_hotkey", "Hotkey", &mut self.config.aim.aimbot_hotkey) {
                self.send_config();
            }

            if combo_box(ui, "aimbot_mode", "Method", &mut self.weapon_config().aimbot.mode) {
                self.send_config();
            }

            if drag(ui, "FOV", DragValue::new(&mut self.weapon_config().aimbot.fov).range(0.1..=180.0).speed(0.1)) {
                self.send_config();
            }

            if drag(ui, "Smooth", DragValue::new(&mut self.weapon_config().aimbot.smooth).range(1.0..=20.0).speed(0.1)) {
                self.send_config();
            }

            if checkbox(ui, "Backtrack", &mut self.weapon_config().aimbot.backtrack) {
                self.send_config();
            }

            if checkbox(ui, "Advanced Humanizer", &mut self.weapon_config().aimbot.advanced_humanizer) {
                self.send_config();
            }
        });

        if self.weapon_config().aimbot.enabled != enabled {
            self.weapon_config().aimbot.enabled = enabled;
            self.send_config();
        }

        section(ui, "Targeting", None, |ui| {
            if checkbox_hover(ui, "Distance-Adjusted FOV", "Dynamically scale FOV by distance", &mut self.weapon_config().aimbot.distance_adjusted_fov) {
                self.send_config();
            }

            if checkbox_hover(ui, "Target Team", "Enable for DM/Custom modes", &mut self.weapon_config().aimbot.target_friendlies) {
                self.send_config();
            }

            if combo_box(ui, "aim_targeting_mode", "Preferred Mode", &mut self.weapon_config().aimbot.targeting_mode) {
                self.send_config();
            }

            ui.add_space(4.0);
            ui.label("Hitboxes:");
            ui.horizontal_wrapped(|ui| {
                for bone in Bones::iter() {
                    let text = format!("{:?}", bone);
                    let index = self.weapon_config().aimbot.bones.iter().position(|b| *b == bone);
                    if ui.selectable_label(index.is_some(), text).clicked() {
                        if let Some(index) = index {
                            self.weapon_config().aimbot.bones.remove(index);
                        } else {
                            self.weapon_config().aimbot.bones.push(bone);
                        }
                        self.send_config();
                    }
                }
            });
        });
    }

    fn aimbot_right(&mut self, ui: &mut Ui) {
        let is_override = self.aimbot_tab == AimbotTab::Weapon;
        let mut rcs_enabled = self.weapon_config().rcs.enabled;

        section(ui, "Recoil Control (RCS)", Some(&mut rcs_enabled), |ui| {
            if is_override {
                if checkbox(ui, "Override Global", &mut self.weapon_config().rcs.enable_override) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            if drag(ui, "Smoothness", DragValue::new(&mut self.weapon_config().rcs.smooth).range(0.0..=1.0).speed(0.01)) {
                self.send_config();
            }
        });

        if self.weapon_config().rcs.enabled != rcs_enabled {
            self.weapon_config().rcs.enabled = rcs_enabled;
            self.send_config();
        }

        let mut trigger_enabled = self.weapon_config().triggerbot.enabled;
        section(ui, "Triggerbot", Some(&mut trigger_enabled), |ui| {
            if is_override {
                if checkbox(ui, "Override Global", &mut self.weapon_config().triggerbot.enable_override) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            if keybind(ui, "trigger_hotkey", "Hotkey", &mut self.config.aim.triggerbot_hotkey) {
                self.send_config();
            }

            ui.horizontal(|ui| {
                let mut start = *self.weapon_config().triggerbot.delay.start();
                let mut end = *self.weapon_config().triggerbot.delay.end();
                let res = ui.add(DragValue::new(&mut start).prefix("Min: ").range(0..=1000));
                ui.add(DragValue::new(&mut end).prefix("Max: ").range(0..=1000));
                if res.changed() || start > end {
                     self.weapon_config().triggerbot.delay = start..=end;
                     self.send_config();
                }
                ui.label("Delay (ms)");
            });

            if checkbox(ui, "Head Only", &mut self.weapon_config().triggerbot.head_only) {
                self.send_config();
            }
        });

        if self.weapon_config().triggerbot.enabled != trigger_enabled {
            self.weapon_config().triggerbot.enabled = trigger_enabled;
            self.send_config();
        }

        section(ui, "Extra Checks", None, |ui| {
            if checkbox(ui, "Visibility Check", &mut self.weapon_config().aimbot.visibility_check) {
                self.send_config();
            }
            if checkbox(ui, "Flash Check", &mut self.weapon_config().aimbot.flash_check) {
                self.send_config();
            }
            
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);
            
            if checkbox_hover(ui, "Movement Prediction", "Compensate for target velocity (useful for moving targets)", &mut self.weapon_config().aimbot.prediction) {
                self.send_config();
            }
            
            if self.weapon_config().aimbot.prediction
                && drag(ui, "Strength", DragValue::new(&mut self.weapon_config().aimbot.prediction_factor).range(0.1..=5.0).speed(0.1))
            {
                self.send_config();
            }
        });
    }
}
