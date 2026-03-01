use egui::{DragValue, Ui};

use crate::ui::{
    app::App,
    gui::helpers::{checkbox, drag, scroll, section},
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
            section(ui, "Smokes", Some(&mut no_smoke), |ui| {
                if checkbox(ui, "Custom Smoke Color", &mut self.config.misc.change_smoke_color) {
                    self.send_config();
                }

                if crate::ui::gui::helpers::color_picker(ui, "Smoke Color", &mut self.config.misc.smoke_color) {
                    self.send_config();
                }
            });
            if self.config.misc.no_smoke != no_smoke {
                self.config.misc.no_smoke = no_smoke;
                self.send_config();
            }
        });
    }

    fn unsafe_left(&mut self, ui: &mut Ui) {
        let mut no_flash = self.config.misc.no_flash;
        section(ui, "No Flash", Some(&mut no_flash), |ui| {
            if drag(
                ui,
                "Flash Opacity",
                DragValue::new(&mut self.config.misc.max_flash_alpha)
                    .range(0.0..=255.0)
                    .speed(1.0)
                    .max_decimals(0),
            ) {
                self.send_config();
            }
        });
        if self.config.misc.no_flash != no_flash {
            self.config.misc.no_flash = no_flash;
            self.send_config();
        }
    }

    fn unsafe_right(&mut self, ui: &mut Ui) {
        let mut fov_changer = self.config.misc.fov_changer;
        section(ui, "FOV Changer", Some(&mut fov_changer), |ui| {
            if drag(
                ui,
                "Field of View",
                DragValue::new(&mut self.config.misc.desired_fov)
                    .speed(1.0)
                    .range(1..=179),
            ) {
                self.send_config();
            }

            if ui.button("Reset Default").clicked() {
                self.config.misc.desired_fov = crate::constants::cs2::DEFAULT_FOV;
                self.send_config();
            }
        });
        if self.config.misc.fov_changer != fov_changer {
            self.config.misc.fov_changer = fov_changer;
            self.send_config();
        }
    }
}
