use egui::{DragValue, Ui};
use strum::IntoEnumIterator as _;

use crate::{
    config::Language,
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
            ui.selectable_value(
                &mut self.aimbot_tab,
                AimbotTab::Global,
                if self.config.language == Language::Russian {
                    "Глобальные"
                } else {
                    "Global Settings"
                },
            );
            ui.add_space(8.0);
            ui.selectable_value(
                &mut self.aimbot_tab,
                AimbotTab::Weapon,
                if self.config.language == Language::Russian {
                    "Для оружия"
                } else {
                    "Weapon Overrides"
                },
            );

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

        section(ui, self.t("aimbot"), Some(&mut enabled), |ui| {
            if is_override {
                if checkbox(
                    ui,
                    if self.config.language == Language::Russian {
                        "Перезаписать"
                    } else {
                        "Override Global"
                    },
                    &mut self.weapon_config().aimbot.enable_override,
                ) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            if keybind(
                ui,
                "aimbot_hotkey",
                self.t("hotkey"),
                &mut self.config.aim.aimbot_hotkey,
            ) {
                self.send_config();
            }

            if combo_box(
                ui,
                "aimbot_mode",
                self.t("mode"),
                &mut self.weapon_config().aimbot.mode,
            ) {
                self.send_config();
            }

            if drag(
                ui,
                self.t("fov"),
                DragValue::new(&mut self.weapon_config().aimbot.fov)
                    .range(0.1..=180.0)
                    .speed(0.1),
            ) {
                self.send_config();
            }

            if drag(
                ui,
                self.t("smooth"),
                DragValue::new(&mut self.weapon_config().aimbot.smooth)
                    .range(1.0..=20.0)
                    .speed(0.1),
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                self.t("backtrack"),
                &mut self.weapon_config().aimbot.backtrack,
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                self.t("humanizer"),
                &mut self.weapon_config().aimbot.advanced_humanizer,
            ) {
                self.send_config();
            }

            if drag(
                ui,
                self.t("hitchance"),
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

        section(ui, self.t("targeting"), None, |ui| {
            if checkbox_hover(
                ui,
                if self.config.language == Language::Russian {
                    "Авто-FOV по дистанции"
                } else {
                    "Distance-Adjusted FOV"
                },
                if self.config.language == Language::Russian {
                    "Динамически менять FOV в зависимости от расстояния"
                } else {
                    "Dynamically scale FOV by distance"
                },
                &mut self.weapon_config().aimbot.distance_adjusted_fov,
            ) {
                self.send_config();
            }

            if checkbox_hover(
                ui,
                if self.config.language == Language::Russian {
                    "Целиться в своих"
                } else {
                    "Target Team"
                },
                if self.config.language == Language::Russian {
                    "Включить для DM или кастомных режимов"
                } else {
                    "Enable for DM/Custom modes"
                },
                &mut self.weapon_config().aimbot.target_friendlies,
            ) {
                self.send_config();
            }

            if combo_box(
                ui,
                "aim_targeting_mode",
                self.t("targeting"),
                &mut self.weapon_config().aimbot.targeting_mode,
            ) {
                self.send_config();
            }

            if checkbox(
                ui,
                self.t("multipoint"),
                &mut self.weapon_config().aimbot.multipoint,
            ) {
                self.send_config();
            }

            if self.weapon_config().aimbot.multipoint
                && drag(
                    ui,
                    self.t("multipoint_scale"),
                    DragValue::new(&mut self.weapon_config().aimbot.multipoint_scale)
                        .range(0.1..=1.0)
                        .speed(0.05),
                )
            {
                self.send_config();
            }

            ui.add_space(4.0);
            ui.label(format!("{}:", self.t("bones")));
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

        section(ui, self.t("recoil_control"), Some(&mut rcs_enabled), |ui| {
            if is_override {
                if checkbox(
                    ui,
                    if self.config.language == Language::Russian {
                        "Перезаписать"
                    } else {
                        "Override Global"
                    },
                    &mut self.weapon_config().rcs.enable_override,
                ) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            if drag(
                ui,
                self.t("smooth"),
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
        section(ui, self.t("triggerbot"), Some(&mut trigger_enabled), |ui| {
            if is_override {
                if checkbox(
                    ui,
                    if self.config.language == Language::Russian {
                        "Перезаписать"
                    } else {
                        "Override Global"
                    },
                    &mut self.weapon_config().triggerbot.enable_override,
                ) {
                    self.send_config();
                }
                ui.add_space(4.0);
            }

            if keybind(
                ui,
                "trigger_hotkey",
                self.t("hotkey"),
                &mut self.config.aim.triggerbot_hotkey,
            ) {
                self.send_config();
            }

            ui.horizontal(|ui| {
                let mut start = *self.weapon_config().triggerbot.delay.start();
                let mut end = *self.weapon_config().triggerbot.delay.end();
                let res = ui.add(
                    DragValue::new(&mut start)
                        .prefix(if self.config.language == Language::Russian {
                            "Мин: "
                        } else {
                            "Min: "
                        })
                        .range(0..=1000),
                );
                ui.add(
                    DragValue::new(&mut end)
                        .prefix(if self.config.language == Language::Russian {
                            "Макс: "
                        } else {
                            "Max: "
                        })
                        .range(0..=1000),
                );
                if res.changed() || start > end {
                    self.weapon_config().triggerbot.delay = start..=end;
                    self.send_config();
                }
                ui.label(if self.config.language == Language::Russian {
                    "Задержка (мс)"
                } else {
                    "Delay (ms)"
                });
            });

            if checkbox(
                ui,
                if self.config.language == Language::Russian {
                    "Только голова"
                } else {
                    "Head Only"
                },
                &mut self.weapon_config().triggerbot.head_only,
            ) {
                self.send_config();
            }
        });

        if self.weapon_config().triggerbot.enabled != trigger_enabled {
            self.weapon_config().triggerbot.enabled = trigger_enabled;
            self.send_config();
        }

        section(
            ui,
            if self.config.language == Language::Russian {
                "Доп. Проверки"
            } else {
                "Extra Checks"
            },
            None,
            |ui| {
                if checkbox(
                    ui,
                    self.t("visibility_check"),
                    &mut self.weapon_config().aimbot.visibility_check,
                ) {
                    self.send_config();
                }
                if checkbox(
                    ui,
                    self.t("flash_check"),
                    &mut self.weapon_config().aimbot.flash_check,
                ) {
                    self.send_config();
                }

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                if checkbox_hover(
                    ui,
                    self.t("prediction"),
                    if self.config.language == Language::Russian {
                        "Учитывать скорость цели (полезно для бегущих)"
                    } else {
                        "Compensate for target velocity (useful for moving targets)"
                    },
                    &mut self.weapon_config().aimbot.prediction,
                ) {
                    self.send_config();
                }

                if self.weapon_config().aimbot.prediction
                    && drag(
                        ui,
                        if self.config.language == Language::Russian {
                            "Сила"
                        } else {
                            "Strength"
                        },
                        DragValue::new(&mut self.weapon_config().aimbot.prediction_factor)
                            .range(0.1..=5.0)
                            .speed(0.1),
                    )
                {
                    self.send_config();
                }
            },
        );
    }
}
