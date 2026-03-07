use crate::utils::log;
use egui::{Align, Button, Context, Ui};

use crate::{
    config::{
        BASE_PATH, CONFIG_PATH, Config, Language, available_configs, delete_config, parse_config,
        write_config,
    },
    ui::{
        app::App,
        color::Colors,
        grenades::read_grenades,
        gui::helpers::{scroll, section},
    },
};

impl App {
    pub fn config_settings(&mut self, ui: &mut Ui, ctx: &Context) {
        scroll(ui, "config_scroll", |ui| {
            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    self.config_left(ui, ctx);
                });

                columns[1].vertical(|ui| {
                    self.config_presets(ui);

                    section(ui, self.t("saved_profiles"), None, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .button(self.t("reload"))
                                .on_hover_text("Reload all configs and grenades")
                                .clicked()
                            {
                                self.available_configs = available_configs();
                                *self.grenades.lock() = read_grenades();
                            }

                            ui.add_space(4.0);

                            ui.add(
                                egui::TextEdit::singleline(&mut self.new_config_name)
                                    .desired_width(120.0),
                            );
                            if ui.button(self.t("create")).clicked()
                                && !self.new_config_name.is_empty()
                            {
                                if !self.new_config_name.ends_with(".toml") {
                                    self.new_config_name.push_str(".toml");
                                }
                                let path = CONFIG_PATH.join(&self.new_config_name);
                                write_config(&self.config, &path);
                                self.new_config_name.clear();
                                self.current_config = path;
                                self.available_configs = available_configs();
                            }
                        });

                        ui.add_space(10.0);
                        self.config_right(ui);
                    });
                });
            });
        });
    }

    fn config_left(&mut self, ui: &mut Ui, ctx: &Context) {
        section(ui, self.t("active_profile"), None, |ui| {
            let current_name = self
                .current_config
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            ui.label(format!(
                "{}: {}",
                if self.config.language == Language::Russian {
                    "Текущий"
                } else {
                    "Current"
                },
                current_name
            ));
            ui.add_space(4.0);

            ui.columns(2, |cols| {
                if cols[0].button(self.t("reset")).clicked() {
                    self.config = Config::default();
                    self.send_config();
                    log::info!("loaded default config");
                }

                if cols[1].button(self.t("open_folder")).clicked() {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(BASE_PATH.as_os_str())
                        .status();
                }
            });
        });

        section(ui, "DEADLOCKED", None, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", self.t("language")));
                let mut lang = self.config.language;
                if ui
                    .selectable_value(&mut lang, Language::English, "English")
                    .changed()
                    || ui
                        .selectable_value(&mut lang, Language::Russian, "Русский")
                        .changed()
                {
                    self.config.language = lang;
                    self.send_config();
                }
            });

            ui.add_space(10.0);
            ui.label(format!("{}:", self.t("accent_color")));
            egui::ComboBox::from_id_salt("accent_picker")
                .selected_text(
                    Colors::ACCENT_COLORS
                        .iter()
                        .find(|c| c.1 == self.config.accent_color)
                        .unwrap_or(&Colors::ACCENT_COLORS[5])
                        .0,
                )
                .show_ui(ui, |ui| {
                    for (name, color) in Colors::ACCENT_COLORS {
                        if ui
                            .add(
                                Button::new(name)
                                    .selected(color == self.config.accent_color)
                                    .fill(color),
                            )
                            .clicked()
                        {
                            self.config.accent_color = color;
                            ctx.style_mut(|style| style.visuals.selection.bg_fill = color);
                            self.send_config();
                        }
                    }
                });
        });
    }

    fn config_presets(&mut self, ui: &mut Ui) {
        section(ui, self.t("presets"), None, |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui
                    .button(
                        egui::RichText::new(self.t("legit"))
                            .strong()
                            .color(Colors::GREEN),
                    )
                    .clicked()
                {
                    self.config.load_preset(0);
                    self.send_config();
                }
                if ui
                    .button(
                        egui::RichText::new(self.t("semi_legit"))
                            .strong()
                            .color(Colors::TEAL),
                    )
                    .clicked()
                {
                    self.config.load_preset(1);
                    self.send_config();
                }
                if ui
                    .button(
                        egui::RichText::new(self.t("recommended"))
                            .strong()
                            .color(self.config.accent_color),
                    )
                    .clicked()
                {
                    self.config.load_preset(2);
                    self.send_config();
                }
                if ui
                    .button(
                        egui::RichText::new(self.t("blatant"))
                            .strong()
                            .color(Colors::ORANGE),
                    )
                    .clicked()
                {
                    self.config.load_preset(3);
                    self.send_config();
                }
                if ui
                    .button(
                        egui::RichText::new(self.t("rage"))
                            .strong()
                            .color(Colors::RED),
                    )
                    .clicked()
                {
                    self.config.load_preset(4);
                    self.send_config();
                }
            });
        });
    }

    fn config_right(&mut self, ui: &mut Ui) {
        let mut clicked_config = None;
        let mut delete = None;

        for config in &self.available_configs {
            let name = config.file_name().unwrap().to_str().unwrap();
            let is_active = *config == self.current_config;

            ui.horizontal(|ui| {
                let name_width = ui.available_width() - 32.0;

                let text = if is_active {
                    egui::RichText::new(format!("● {}", name))
                        .color(self.config.accent_color)
                        .strong()
                } else {
                    egui::RichText::new(format!("  {}", name)).color(Colors::TEXT)
                };

                if ui
                    .add_sized(
                        [name_width, 24.0],
                        egui::Button::new(text)
                            .frame(is_active)
                            .fill(Colors::HIGHLIGHT.gamma_multiply(0.3)),
                    )
                    .clicked()
                {
                    clicked_config = Some(config.clone());
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("🗑").on_hover_text(self.t("delete")).clicked() {
                        delete = Some(config.clone());
                    }
                });
            });
            ui.add_space(2.0);
        }

        if let Some(config_path) = clicked_config {
            self.config = parse_config(&config_path);
            self.current_config = config_path;
            self.send_config();
            ui.ctx()
                .style_mut(|style| style.visuals.selection.bg_fill = self.config.accent_color);
        }

        if let Some(config) = delete {
            delete_config(&config);
            self.available_configs = available_configs();
            self.current_config = self.available_configs[0].clone();
            self.config = parse_config(&self.current_config);
        }
    }
}
