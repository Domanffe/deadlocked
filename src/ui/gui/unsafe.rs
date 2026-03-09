use egui::{DragValue, Ui};

use crate::ui::{
    app::App,
    gui::helpers::{checkbox, drag, scroll, section},
};

impl App {
    pub fn unsafe_settings(&mut self, ui: &mut Ui) {
        let smokes_text = self.t("smokes");
        let custom_smoke_color_text = self.t("custom_smoke_color");
        let smoke_color_text = self.t("smoke_color");

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
            section(ui, smokes_text, Some(&mut no_smoke), |ui| {
                if checkbox(
                    ui,
                    custom_smoke_color_text,
                    &mut self.config.misc.change_smoke_color,
                ) {
                    self.send_config();
                }

                if crate::ui::gui::helpers::color_picker(
                    ui,
                    smoke_color_text,
                    &mut self.config.misc.smoke_color,
                ) {
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
        let no_flash_text = self.t("no_flash");
        let flash_opacity_text = self.t("flash_opacity");

        let mut no_flash = self.config.misc.no_flash;
        section(ui, no_flash_text, Some(&mut no_flash), |ui| {
            if drag(
                ui,
                flash_opacity_text,
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
        let automation_text = self.t("automation");
        let auto_accept_text = self.t("auto_accept");
        let bunnyhop_text = self.t("bunnyhop");

        section(ui, automation_text, None, |ui| {
            if checkbox(ui, auto_accept_text, &mut self.config.misc.auto_accept) {
                self.send_config();
            }
            if checkbox(ui, bunnyhop_text, &mut self.config.misc.bunnyhop) {
                self.send_config();
            }
        });

        let fov_changer_text = self.t("fov_changer");
        let field_of_view_text = self.t("field_of_view");
        let reset_default_text = self.t("reset_default");

        let mut fov_changer = self.config.misc.fov_changer;
        section(ui, fov_changer_text, Some(&mut fov_changer), |ui| {
            if drag(
                ui,
                field_of_view_text,
                DragValue::new(&mut self.config.misc.desired_fov)
                    .speed(1.0)
                    .range(1..=179),
            ) {
                self.send_config();
            }

            if ui.button(reset_default_text).clicked() {
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
