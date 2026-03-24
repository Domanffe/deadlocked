use egui::{DragValue, Ui};
use strum::IntoEnumIterator as _;

use crate::{
    cs2::bones::Bones,
    ui::{
        app::App,
        color::Colors,
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
        let global_text = self.t("global_settings");
        let weapon_text = self.t("weapon_overrides");
        let weapon_icon = self.t("weapon_icon");

        ui.horizontal(|ui| {
            let global_selected = self.aimbot_tab == AimbotTab::Global;
            if ui
                .add(
                    egui::Button::new(global_text)
                        .fill(if global_selected {
                            self.config.accent_color.linear_multiply(0.22)
                        } else {
                            Colors::HIGHLIGHT
                        })
                        .stroke(if global_selected {
                            egui::Stroke::new(1.0, self.config.accent_color)
                        } else {
                            egui::Stroke::new(1.0, Colors::GRAY)
                        }),
                )
                .clicked()
            {
                self.aimbot_tab = AimbotTab::Global;
            }
            let weapon_selected = self.aimbot_tab == AimbotTab::Weapon;
            if ui
                .add(
                    egui::Button::new(weapon_text)
                        .fill(if weapon_selected {
                            self.config.accent_color.linear_multiply(0.22)
                        } else {
                            Colors::HIGHLIGHT
                        })
                        .stroke(if weapon_selected {
                            egui::Stroke::new(1.0, self.config.accent_color)
                        } else {
                            egui::Stroke::new(1.0, Colors::GRAY)
                        }),
                )
                .clicked()
            {
                self.aimbot_tab = AimbotTab::Weapon;
            }

            if self.aimbot_tab == AimbotTab::Weapon {
                ui.add_space(8.0);
                combo_box(ui, "aimbot_weapon", weapon_icon, &mut self.aimbot_weapon);
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

        let aimbot_text = self.t("aimbot");
        section(ui, aimbot_text, Some(&mut enabled), |ui| {
            if is_override {
                let override_text = self.t("override_global");
                if checkbox(
                    ui,
                    override_text,
                    &mut self.weapon_config().aimbot.enable_override,
                ) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            let hotkey_text = self.t("hotkey");
            if keybind(
                ui,
                "aimbot_hotkey",
                hotkey_text,
                &mut self.config.aim.aimbot_hotkey,
            ) {
                self.send_config();
            }

            let mode_text = self.t("mode");
            if combo_box(
                ui,
                "aimbot_mode",
                mode_text,
                &mut self.weapon_config().aimbot.mode,
            ) {
                self.send_config();
            }

            let fov_text = self.t("fov");
            if drag(
                ui,
                fov_text,
                DragValue::new(&mut self.weapon_config().aimbot.fov)
                    .range(0.1..=180.0)
                    .speed(0.1),
            ) {
                self.send_config();
            }

            let smooth_text = self.t("smooth");
            if drag(
                ui,
                smooth_text,
                DragValue::new(&mut self.weapon_config().aimbot.smooth)
                    .range(1.0..=20.0)
                    .speed(0.1),
            ) {
                self.send_config();
            }

            let backtrack_text = self.t("backtrack");
            if checkbox(
                ui,
                backtrack_text,
                &mut self.weapon_config().aimbot.backtrack,
            ) {
                self.send_config();
            }

            let humanizer_text = self.t("humanizer");
            if checkbox(
                ui,
                humanizer_text,
                &mut self.weapon_config().aimbot.advanced_humanizer,
            ) {
                self.send_config();
            }

            let hitchance_text = self.t("hitchance");
            if drag(
                ui,
                hitchance_text,
                DragValue::new(&mut self.weapon_config().aimbot.hitchance)
                    .range(0.0..=100.0)
                    .speed(1.0),
            ) {
                self.send_config();
            }
        });

        if self.weapon_config().aimbot.enabled != enabled {
            self.weapon_config().aimbot.enabled = enabled;
            self.send_config();
        }

        let targeting_text = self.t("targeting");
        section(ui, targeting_text, None, |ui| {
            let distance_fov = self.t("distance_fov");
            let distance_fov_desc = self.t("distance_fov_desc");
            if checkbox_hover(
                ui,
                distance_fov,
                distance_fov_desc,
                &mut self.weapon_config().aimbot.distance_adjusted_fov,
            ) {
                self.send_config();
            }

            let target_team = self.t("target_team");
            let target_team_desc = self.t("target_team_desc");
            if checkbox_hover(
                ui,
                target_team,
                target_team_desc,
                &mut self.weapon_config().aimbot.target_friendlies,
            ) {
                self.send_config();
            }

            let targeting_mode_text = self.t("targeting");
            if combo_box(
                ui,
                "aim_targeting_mode",
                targeting_mode_text,
                &mut self.weapon_config().aimbot.targeting_mode,
            ) {
                self.send_config();
            }

            let multipoint_text = self.t("multipoint");
            if checkbox(
                ui,
                multipoint_text,
                &mut self.weapon_config().aimbot.multipoint,
            ) {
                self.send_config();
            }

            if self.weapon_config().aimbot.multipoint {
                let scale_text = self.t("multipoint_scale");
                if drag(
                    ui,
                    scale_text,
                    DragValue::new(&mut self.weapon_config().aimbot.multipoint_scale)
                        .range(0.1..=1.0)
                        .speed(0.05),
                ) {
                    self.send_config();
                }
            }

            ui.add_space(4.0);
            let bones_text = self.t("bones");
            ui.label(format!("{}:", bones_text));
            ui.horizontal_wrapped(|ui| {
                for bone in Bones::iter() {
                    let text = format!("{:?}", bone);
                    let index = self
                        .weapon_config()
                        .aimbot
                        .bones
                        .iter()
                        .position(|b| *b == bone);
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

        let recoil_text = self.t("recoil_control");
        section(ui, recoil_text, Some(&mut rcs_enabled), |ui| {
            if is_override {
                let override_text = self.t("override_global");
                if checkbox(
                    ui,
                    override_text,
                    &mut self.weapon_config().rcs.enable_override,
                ) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            let rcs_smooth_text = self.t("recoil_smooth");
            if drag(
                ui,
                rcs_smooth_text,
                DragValue::new(&mut self.weapon_config().rcs.smooth)
                    .range(0.0..=1.0)
                    .speed(0.01),
            ) {
                self.send_config();
            }
        });

        if self.weapon_config().rcs.enabled != rcs_enabled {
            self.weapon_config().rcs.enabled = rcs_enabled;
            self.send_config();
        }

        let mut trigger_enabled = self.weapon_config().triggerbot.enabled;
        let trigger_text = self.t("triggerbot");
        section(ui, trigger_text, Some(&mut trigger_enabled), |ui| {
            if is_override {
                let override_text = self.t("override_global");
                if checkbox(
                    ui,
                    override_text,
                    &mut self.weapon_config().triggerbot.enable_override,
                ) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            let hotkey_text = self.t("hotkey");
            if keybind(
                ui,
                "trigger_hotkey",
                hotkey_text,
                &mut self.config.aim.triggerbot_hotkey,
            ) {
                self.send_config();
            }

            ui.horizontal(|ui| {
                if ui
                    .add(crate::ui::drag_range::DragRange::new(
                        &mut self.weapon_config().triggerbot.delay,
                        0..=1000,
                    ))
                    .changed()
                {
                    self.send_config();
                }
                let delay_text = self.t("delay");
                ui.label(delay_text);
            });

            let head_only_text = self.t("head_only");
            if checkbox(
                ui,
                head_only_text,
                &mut self.weapon_config().triggerbot.head_only,
            ) {
                self.send_config();
            }
        });

        if self.weapon_config().triggerbot.enabled != trigger_enabled {
            self.weapon_config().triggerbot.enabled = trigger_enabled;
            self.send_config();
        }

        let extra_checks_text = self.t("extra_checks");
        section(ui, extra_checks_text, None, |ui| {
            let visibility_check_text = self.t("visibility_check");
            if checkbox(
                ui,
                visibility_check_text,
                &mut self.weapon_config().aimbot.visibility_check,
            ) {
                self.send_config();
            }
            let flash_check_text = self.t("flash_check");
            if checkbox(
                ui,
                flash_check_text,
                &mut self.weapon_config().aimbot.flash_check,
            ) {
                self.send_config();
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            let prediction_text = self.t("prediction");
            let prediction_desc = self.t("prediction_desc");
            if checkbox_hover(
                ui,
                prediction_text,
                prediction_desc,
                &mut self.weapon_config().aimbot.prediction,
            ) {
                self.send_config();
            }

            if self.weapon_config().aimbot.prediction {
                let strength_text = self.t("strength");
                if drag(
                    ui,
                    strength_text,
                    DragValue::new(&mut self.weapon_config().aimbot.prediction_factor)
                        .range(0.1..=5.0)
                        .speed(0.1),
                ) {
                    self.send_config();
                }
            }
        });
    }
}
