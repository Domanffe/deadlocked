use egui::{DragValue, Ui};

use crate::ui::{
    app::App,
    gui::helpers::{drag, scroll, section},
};

impl App {
    pub fn radar_settings(&mut self, ui: &mut Ui) {
        let mut enabled = self.config.radar.enabled;
        let radar_text = self.t("radar");

        scroll(ui, "radar_settings", |ui| {
            section(ui, radar_text, Some(&mut enabled), |ui| {
                let scale_text = self.t("size"); // Or use a separate key if we want
                if drag(
                    ui,
                    scale_text,
                    DragValue::new(&mut self.config.radar.size)
                        .range(50.0..=500.0)
                        .speed(1.0),
                ) {
                    self.send_config();
                }

                let radius_text = self.t("distance");
                if drag(
                    ui,
                    radius_text,
                    DragValue::new(&mut self.config.radar.scale)
                        .range(0.1..=5.0)
                        .speed(0.1),
                ) {
                    self.send_config();
                }

                // Temporary simple position sliders until drag-and-drop is implemented
                let pos_x_text = self.t("position_x");
                if drag(
                    ui,
                    pos_x_text,
                    DragValue::new(&mut self.config.radar.position[0])
                        .range(0.0..=3840.0)
                        .speed(1.0),
                ) {
                    self.send_config();
                }

                let pos_y_text = self.t("position_y");
                if drag(
                    ui,
                    pos_y_text,
                    DragValue::new(&mut self.config.radar.position[1])
                        .range(0.0..=2160.0)
                        .speed(1.0),
                ) {
                    self.send_config();
                }
            });

            if self.config.radar.enabled != enabled {
                self.config.radar.enabled = enabled;
                self.send_config();
            }
        });
    }
}
