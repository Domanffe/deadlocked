use egui::Ui;

use crate::{
    config::Language,
    ui::{
        app::App,
        gui::helpers::{checkbox, color_picker, scroll, section},
    },
};

impl App {
    pub fn hud_settings(&mut self, ui: &mut Ui) {
        let mut grenade_trails = self.config.hud.grenade_trails;
        scroll(ui, "hud_settings", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| self.hud_left(ui));
                cols[1].vertical(|ui| self.hud_right(ui));
            });

            section(
                ui,
                if self.config.language == Language::Russian {
                    "Траектории гранат"
                } else {
                    "Grenade Trails"
                },
                Some(&mut grenade_trails),
                |ui| {
                    ui.columns(2, |cols| {
                        let left = &mut cols[0];
                        if color_picker(
                            left,
                            if self.config.language == Language::Russian {
                                "Дым"
                            } else {
                                "Smoke"
                            },
                            &mut self.config.hud.smoke_trail_color,
                        ) {
                            self.send_config();
                        }
                        if color_picker(
                            left,
                            if self.config.language == Language::Russian {
                                "Флешка"
                            } else {
                                "Flash"
                            },
                            &mut self.config.hud.flash_trail_color,
                        ) {
                            self.send_config();
                        }
                        if color_picker(
                            left,
                            if self.config.language == Language::Russian {
                                "Ложная"
                            } else {
                                "Decoy"
                            },
                            &mut self.config.hud.decoy_trail_color,
                        ) {
                            self.send_config();
                        }

                        let right = &mut cols[1];
                        if color_picker(
                            right,
                            if self.config.language == Language::Russian {
                                "Молотов"
                            } else {
                                "Molotov"
                            },
                            &mut self.config.hud.molotov_trail_color,
                        ) {
                            self.send_config();
                        }
                        if color_picker(
                            right,
                            if self.config.language == Language::Russian {
                                "Зажигательная"
                            } else {
                                "Incendiary"
                            },
                            &mut self.config.hud.incendiary_trail_color,
                        ) {
                            self.send_config();
                        }
                        if color_picker(
                            right,
                            if self.config.language == Language::Russian {
                                "Осколочная"
                            } else {
                                "HE Grenade"
                            },
                            &mut self.config.hud.he_trail_color,
                        ) {
                            self.send_config();
                        }
                    });
                },
            );

            if self.config.hud.grenade_trails != grenade_trails {
                self.config.hud.grenade_trails = grenade_trails;
                self.send_config();
            }

            section(
                ui,
                if self.config.language == Language::Russian {
                    "Цвета интерфейса"
                } else {
                    "Global HUD Colors"
                },
                None,
                |ui| {
                    ui.columns(2, |cols| {
                        if color_picker(
                            &mut cols[0],
                            if self.config.language == Language::Russian {
                                "Цвет текста"
                            } else {
                                "Text Color"
                            },
                            &mut self.config.hud.text_color,
                        ) {
                            self.send_config();
                        }
                        if color_picker(
                            &mut cols[1],
                            if self.config.language == Language::Russian {
                                "Прицел"
                            } else {
                                "Crosshair"
                            },
                            &mut self.config.hud.crosshair_color,
                        ) {
                            self.send_config();
                        }
                    });
                },
            );
        });
    }

    fn hud_left(&mut self, ui: &mut Ui) {
        section(
            ui,
            if self.config.language == Language::Russian {
                "Общее"
            } else {
                "General"
            },
            None,
            |ui| {
                if checkbox(ui, self.t("bomb_timer"), &mut self.config.hud.bomb_timer) {
                    self.send_config();
                }
                if checkbox(ui, self.t("bomb_damage"), &mut self.config.hud.bomb_damage) {
                    self.send_config();
                }
                if checkbox(
                    ui,
                    self.t("spectator_list"),
                    &mut self.config.hud.spectator_list,
                ) {
                    self.send_config();
                }
                if checkbox(
                    ui,
                    self.t("keybind_list"),
                    &mut self.config.hud.keybind_list,
                ) {
                    self.send_config();
                }
                if checkbox(ui, self.t("fov_circle"), &mut self.config.hud.fov_circle) {
                    self.send_config();
                }
                if checkbox(
                    ui,
                    if self.config.language == Language::Russian {
                        "Прицел для снайперок"
                    } else {
                        "Sniper Crosshair"
                    },
                    &mut self.config.hud.sniper_crosshair,
                ) {
                    self.send_config();
                }
            },
        );

        section(
            ui,
            if self.config.language == Language::Russian {
                "Индикаторы"
            } else {
                "Indicators"
            },
            None,
            |ui| {
                if checkbox(ui, self.t("fov_arrows"), &mut self.config.hud.fov_arrows) {
                    self.send_config();
                }
                if self.config.hud.fov_arrows {
                    ui.add(
                        egui::Slider::new(&mut self.config.hud.arrow_size, 5.0..=30.0).text(
                            if self.config.language == Language::Russian {
                                "Размер"
                            } else {
                                "Size"
                            },
                        ),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.config.hud.arrow_radius, 50.0..=400.0).text(
                            if self.config.language == Language::Russian {
                                "Радиус"
                            } else {
                                "Radius"
                            },
                        ),
                    );
                    self.send_config();
                }

                if checkbox(ui, self.t("hitmarker"), &mut self.config.hud.hitmarker) {
                    self.send_config();
                }
                if self.config.hud.hitmarker
                    && color_picker(
                        ui,
                        self.t("hitmarker_color"),
                        &mut self.config.hud.hitmarker_color,
                    )
                {
                    self.send_config();
                }

                if checkbox(
                    ui,
                    self.t("bullet_tracers"),
                    &mut self.config.hud.bullet_tracers,
                ) {
                    self.send_config();
                }
                if self.config.hud.bullet_tracers
                    && color_picker(
                        ui,
                        self.t("tracer_color"),
                        &mut self.config.hud.tracer_color,
                    )
                {
                    self.send_config();
                }
            },
        );
    }

    fn hud_right(&mut self, ui: &mut Ui) {
        section(
            ui,
            if self.config.language == Language::Russian {
                "Мир"
            } else {
                "World ESP"
            },
            None,
            |ui| {
                if checkbox(
                    ui,
                    if self.config.language == Language::Russian {
                        "Выпавшее оружие"
                    } else {
                        "Dropped Weapons"
                    },
                    &mut self.config.hud.dropped_weapons,
                ) {
                    self.send_config();
                }
            },
        );
    }
}
