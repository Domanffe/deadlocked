use egui::Ui;
use utils::log;
use uuid::Uuid;

use crate::{
    message::{Message, RadarStatus, Target},
    ui::{
        app::App,
        color::Colors,
        gui::helpers::{scroll, section},
    },
};

impl App {
    fn radar_link(&self, uuid: &Uuid) -> String {
        format!("http://{}/?uuid={}", self.config.radar.url, uuid)
    }

    pub fn radar_settings(&mut self, ui: &mut Ui) {
        let mut enabled = self.config.radar.enabled;
        let radar_text = self.t("radar");
        let status_label = self.t("status");
        let server_url_label = self.t("server_url");
        let open_browser_text = self.t("open_browser");
        let copy_link_text = self.t("copy_link");
        let radar_desc_text = self.t("radar_desc");

        scroll(ui, "radar_settings", |ui| {
            section(ui, radar_text, Some(&mut enabled), |ui| {
                let (status_text, status_color) = match self.radar_status {
                    RadarStatus::Connected(_) => (self.t("connected"), Colors::GREEN),
                    RadarStatus::Disconnected => (self.t("disconnected"), Colors::YELLOW),
                };

                ui.horizontal(|ui| {
                    ui.label(status_label);
                    ui.label(
                        egui::RichText::new(status_text)
                            .color(status_color)
                            .strong(),
                    );
                });

                ui.add_space(4.0);
                ui.label(server_url_label);
                if ui
                    .text_edit_singleline(&mut self.config.radar.url)
                    .changed()
                {
                    self.send_message(
                        Message::ChangeRadarUrl(self.config.radar.url.clone()),
                        Target::Radar,
                    );
                    self.save();
                }

                if let RadarStatus::Connected(uuid) = self.radar_status {
                    ui.add_space(10.0);
                    ui.columns(2, |cols| {
                        if cols[0]
                            .button(egui::RichText::new(open_browser_text).strong())
                            .clicked()
                        {
                            let link = self.radar_link(&uuid);
                            let _ = std::process::Command::new("xdg-open").arg(&link).status();
                            log::info!("opened link ({link})");
                        }

                        if cols[1]
                            .button(egui::RichText::new(copy_link_text).strong())
                            .clicked()
                        {
                            let link = self.radar_link(&uuid);
                            let _ = self.clipboard.set_text(link.clone());
                            log::info!("copied link ({link})");
                        }
                    });
                }
            });

            if self.config.radar.enabled != enabled {
                self.config.radar.enabled = enabled;
                self.send_message(
                    Message::RadarSetEnabled(self.config.radar.enabled),
                    Target::Radar,
                );
                self.save();
            }

            if self.config.radar.enabled {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(radar_desc_text)
                        .small()
                        .color(Colors::GRAY),
                );
            }
        });
    }
}
