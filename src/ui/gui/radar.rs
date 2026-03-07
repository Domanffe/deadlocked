use crate::utils::log;
use egui::Ui;
use uuid::Uuid;

use crate::{
    config::Language,
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
        scroll(ui, "radar_settings", |ui| {
            section(ui, self.t("radar"), Some(&mut enabled), |ui| {
                let (status_text, status_color) = match self.radar_status {
                    RadarStatus::Connected(_) => (
                        if self.config.language == Language::Russian {
                            "Подключено"
                        } else {
                            "Connected"
                        },
                        Colors::GREEN,
                    ),
                    RadarStatus::Disconnected => (
                        if self.config.language == Language::Russian {
                            "Отключено"
                        } else {
                            "Disconnected"
                        },
                        Colors::YELLOW,
                    ),
                };

                ui.horizontal(|ui| {
                    ui.label(if self.config.language == Language::Russian {
                        "Статус:"
                    } else {
                        "Status:"
                    });
                    ui.label(
                        egui::RichText::new(status_text)
                            .color(status_color)
                            .strong(),
                    );
                });

                ui.add_space(4.0);
                ui.label(if self.config.language == Language::Russian {
                    "URL сервера:"
                } else {
                    "Server URL:"
                });
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
                            .button(
                                egui::RichText::new(if self.config.language == Language::Russian {
                                    "Открыть в браузере"
                                } else {
                                    "Open in Browser"
                                })
                                .strong(),
                            )
                            .clicked()
                        {
                            let link = self.radar_link(&uuid);
                            let _ = std::process::Command::new("xdg-open").arg(&link).status();
                            log::info!("opened link ({link})");
                        }

                        if cols[1]
                            .button(
                                egui::RichText::new(if self.config.language == Language::Russian {
                                    "Копировать ссылку"
                                } else {
                                    "Copy Share Link"
                                })
                                .strong(),
                            )
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
                ui.label(egui::RichText::new(if self.config.language == Language::Russian { "Веб-радар позволяет видеть карту и игроков на любом устройстве через браузер." } else { "The web radar allows you to see the map and players on any device via a browser." }).small().color(Colors::GRAY));
            }
        });
    }
}
