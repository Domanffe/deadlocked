use egui::{DragValue, Ui};

use crate::{
    config::Language,
    ui::{
        app::App,
        gui::helpers::{checkbox, drag, scroll, section},
    },
};

impl App {
    pub fn unsafe_settings(&mut self, ui: &mut Ui) {
        scroll(ui, "unsafe_scroll", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| {
                    self.unsafe_left(ui);
                });

                cols[1].vertical(|ui| {
                    self.unsafe_right(ui);
                });
            });

            let mut no_smoke = self.config.misc.no_smoke;
            section(
                ui,
                if self.config.language == Language::Russian {
                    "Дымовые гранаты"
                } else {
                    "Smokes"
                },
                Some(&mut no_smoke),
                |ui| {
                    if checkbox(
                        ui,
                        if self.config.language == Language::Russian {
                            "Свой цвет дыма"
                        } else {
                            "Custom Smoke Color"
                        },
                        &mut self.config.misc.change_smoke_color,
                    ) {
                        self.send_config();
                    }

                    if crate::ui::gui::helpers::color_picker(
                        ui,
                        if self.config.language == Language::Russian {
                            "Цвет дыма"
                        } else {
                            "Smoke Color"
                        },
                        &mut self.config.misc.smoke_color,
                    ) {
                        self.send_config();
                    }
                },
            );
            if self.config.misc.no_smoke != no_smoke {
                self.config.misc.no_smoke = no_smoke;
                self.send_config();
            }
        });
    }

    fn unsafe_left(&mut self, ui: &mut Ui) {
        let mut no_flash = self.config.misc.no_flash;
        section(
            ui,
            if self.config.language == Language::Russian {
                "Анти-флеш"
            } else {
                "No Flash"
            },
            Some(&mut no_flash),
            |ui| {
                if drag(
                    ui,
                    if self.config.language == Language::Russian {
                        "Прозрачность"
                    } else {
                        "Flash Opacity"
                    },
                    DragValue::new(&mut self.config.misc.max_flash_alpha)
                        .range(0.0..=255.0)
                        .speed(1.0)
                        .max_decimals(0),
                ) {
                    self.send_config();
                }
            },
        );
        if self.config.misc.no_flash != no_flash {
            self.config.misc.no_flash = no_flash;
            self.send_config();
        }
    }

    fn unsafe_right(&mut self, ui: &mut Ui) {
        section(
            ui,
            if self.config.language == Language::Russian {
                "Автоматизация"
            } else {
                "Automation"
            },
            None,
            |ui| {
                if checkbox(ui, self.t("auto_accept"), &mut self.config.misc.auto_accept) {
                    self.send_config();
                }
                if checkbox(ui, self.t("bunnyhop"), &mut self.config.misc.bunnyhop) {
                    self.send_config();
                }
            },
        );

        let mut fov_changer = self.config.misc.fov_changer;
        section(
            ui,
            if self.config.language == Language::Russian {
                "Изменение FOV"
            } else {
                "FOV Changer"
            },
            Some(&mut fov_changer),
            |ui| {
                if drag(
                    ui,
                    if self.config.language == Language::Russian {
                        "Угол обзора"
                    } else {
                        "Field of View"
                    },
                    DragValue::new(&mut self.config.misc.desired_fov)
                        .speed(1.0)
                        .range(1..=179),
                ) {
                    self.send_config();
                }

                if ui
                    .button(if self.config.language == Language::Russian {
                        "Сбросить по умолчанию"
                    } else {
                        "Reset Default"
                    })
                    .clicked()
                {
                    self.config.misc.desired_fov = crate::constants::cs2::DEFAULT_FOV;
                    self.send_config();
                }
            },
        );
        if self.config.misc.fov_changer != fov_changer {
            self.config.misc.fov_changer = fov_changer;
            self.send_config();
        }
    }
}
