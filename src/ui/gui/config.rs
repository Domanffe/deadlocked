use egui::{Button, Context, Ui};
use utils::log;

use crate::{
    config::{
        BASE_PATH, CONFIG_PATH, Config, Language, available_configs, delete_config, parse_config,
        write_config,
    },
    message::{Message, Target},
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

                    let saved_profiles_text = self.t("saved_profiles");
                    section(ui, saved_profiles_text, None, |ui| {
                        let reload_text = self.t("reload");
                        let create_text = self.t("create");

                        let mut create_requested = false;
                        let compact_layout = ui.available_width() < 360.0;

                        if compact_layout {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(
                                        Button::new(egui::RichText::new(reload_text).strong())
                                            .fill(Colors::HIGHLIGHT)
                                            .stroke(egui::Stroke::new(1.0, Colors::GRAY)),
                                    )
                                    .on_hover_text("Reload all configs and grenades")
                                    .clicked()
                                {
                                    self.available_configs = available_configs();
                                    *self.grenades.lock() = read_grenades();
                                }
                            });
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                let button_width = 92.0;
                                let input_width = (ui.available_width() - button_width - 8.0).max(120.0);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.new_config_name)
                                        .hint_text("profile.toml")
                                        .desired_width(input_width),
                                );
                                if ui
                                    .add(
                                        Button::new(egui::RichText::new(create_text).strong())
                                            .fill(self.config.accent_color.linear_multiply(0.24))
                                            .stroke(egui::Stroke::new(1.0, self.config.accent_color))
                                            .min_size(egui::vec2(button_width, 32.0)),
                                    )
                                    .clicked()
                                {
                                    create_requested = true;
                                }
                            });
                        } else {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(
                                        Button::new(egui::RichText::new(reload_text).strong())
                                            .fill(Colors::HIGHLIGHT)
                                            .stroke(egui::Stroke::new(1.0, Colors::GRAY)),
                                    )
                                    .on_hover_text("Reload all configs and grenades")
                                    .clicked()
                                {
                                    self.available_configs = available_configs();
                                    *self.grenades.lock() = read_grenades();
                                }

                                ui.add_space(4.0);
                                let button_width = 92.0;
                                let input_width = (ui.available_width() - button_width - 8.0).max(120.0);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.new_config_name)
                                        .hint_text("profile.toml")
                                        .desired_width(input_width),
                                );
                                if ui
                                    .add(
                                        Button::new(egui::RichText::new(create_text).strong())
                                            .fill(self.config.accent_color.linear_multiply(0.24))
                                            .stroke(egui::Stroke::new(1.0, self.config.accent_color))
                                            .min_size(egui::vec2(button_width, 32.0)),
                                    )
                                    .clicked()
                                {
                                    create_requested = true;
                                }
                            });
                        }

                        if create_requested && !self.new_config_name.is_empty() {
                            if !self.new_config_name.ends_with(".toml") {
                                self.new_config_name.push_str(".toml");
                            }
                            let path = CONFIG_PATH.join(&self.new_config_name);
                            write_config(&self.config, &path);
                            self.new_config_name.clear();
                            self.current_config = path;
                            self.available_configs = available_configs();
                        }

                        ui.add_space(10.0);
                        self.config_right(ui);
                    });
                });
            });
        });
    }

    fn config_left(&mut self, ui: &mut Ui, ctx: &Context) {
        let active_profile_text = self.t("active_profile");
        section(ui, active_profile_text, None, |ui| {
            let current_name = self
                .current_config
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Temporary")
                .to_string(); // Clone to avoid borrow issues

            let file_label = self.t("file");
            ui.horizontal(|ui| {
                ui.label(format!("{}:", file_label));
                ui.label(
                    egui::RichText::new(&current_name)
                        .color(self.config.accent_color)
                        .strong(),
                );
            });
            ui.add_space(4.0);

            ui.vertical_centered_justified(|ui| {
                let save_text = self.t("save_profile");
                if ui
                    .add(
                        Button::new(egui::RichText::new(save_text).strong())
                            .fill(self.config.accent_color.linear_multiply(0.22))
                            .stroke(egui::Stroke::new(1.0, self.config.accent_color)),
                    )
                    .on_hover_text("Save current settings to this file")
                    .clicked()
                {
                    self.save();
                    log::info!("saved config to {:?}", current_name);
                }

                ui.columns(2, |cols| {
                    let reset_text = self.t("reset");
                    if cols[0]
                        .add(
                            Button::new(reset_text)
                                .fill(Colors::ORANGE.linear_multiply(0.2))
                                .stroke(egui::Stroke::new(1.0, Colors::ORANGE)),
                        )
                        .clicked()
                    {
                        self.config = Config::default();
                        self.active_preset = None;
                        self.send_message(
                            Message::Config(Box::new(self.config.clone())),
                            Target::Game,
                        );
                        log::info!("reset settings to default");
                    }

                    let open_folder_text = self.t("open_folder");
                    if cols[1]
                        .add(
                            Button::new(open_folder_text)
                                .fill(Colors::HIGHLIGHT)
                                .stroke(egui::Stroke::new(1.0, Colors::GRAY)),
                        )
                        .clicked()
                    {
                        self.open_path(&BASE_PATH.to_string_lossy());
                    }
                });
            });
        });

        section(ui, "DEADLOCKED", None, |ui| {
            let language_text = self.t("language");
            ui.horizontal(|ui| {
                ui.label(format!("{}:", language_text));
                let mut lang = self.config.language;
                let english_selected = lang == Language::English;
                let russian_selected = lang == Language::Russian;
                let english_clicked = ui
                    .add(
                        Button::new("English")
                            .fill(if english_selected {
                                self.config.accent_color.linear_multiply(0.22)
                            } else {
                                Colors::HIGHLIGHT
                            })
                            .stroke(if english_selected {
                                egui::Stroke::new(1.0, self.config.accent_color)
                            } else {
                                egui::Stroke::new(1.0, Colors::GRAY)
                            }),
                    )
                    .clicked();
                let russian_clicked = ui
                    .add(
                        Button::new("Русский")
                            .fill(if russian_selected {
                                self.config.accent_color.linear_multiply(0.22)
                            } else {
                                Colors::HIGHLIGHT
                            })
                            .stroke(if russian_selected {
                                egui::Stroke::new(1.0, self.config.accent_color)
                            } else {
                                egui::Stroke::new(1.0, Colors::GRAY)
                            }),
                    )
                    .clicked();
                if english_clicked || russian_clicked {
                    lang = if english_clicked {
                        Language::English
                    } else {
                        Language::Russian
                    };
                    self.config.language = lang;
                    self.active_preset = None;
                    self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
                    self.save();
                }
            });

            ui.add_space(10.0);
            let accent_color_text = self.t("accent_color");
            ui.label(format!("{}:", accent_color_text));
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
                                    .fill(color)
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        if color == self.config.accent_color {
                                            Colors::WHITE
                                        } else {
                                            Colors::GRAY
                                        },
                                    )),
                            )
                            .clicked()
                        {
                            self.config.accent_color = color;
                            ctx.global_style_mut(|style| style.visuals.selection.bg_fill = color);
                            self.active_preset = None;
                            self.send_message(
                                Message::Config(Box::new(self.config.clone())),
                                Target::Game,
                            );
                            self.save();
                        }
                    }
                });

            ui.add_space(10.0);
        });
    }

    fn config_presets(&mut self, ui: &mut Ui) {
        let presets_text = self.t("presets");
        section(ui, presets_text, None, |ui| {
            ui.vertical_centered_justified(|ui| {
                let legit_text = self.t("legit");
                if ui
                    .add(
                        Button::new(
                            egui::RichText::new(legit_text)
                                .strong()
                                .color(Colors::GREEN),
                        )
                        .selected(self.active_preset == Some(0)),
                    )
                    .clicked()
                {
                    self.config.load_preset(0);
                    self.active_preset = Some(0);
                    self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
                }
                let semi_legit_text = self.t("semi_legit");
                if ui
                    .add(
                        Button::new(
                            egui::RichText::new(semi_legit_text)
                                .strong()
                                .color(Colors::TEAL),
                        )
                        .selected(self.active_preset == Some(1)),
                    )
                    .clicked()
                {
                    self.config.load_preset(1);
                    self.active_preset = Some(1);
                    self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
                }
                let recommended_text = self.t("recommended");
                if ui
                    .add(
                        Button::new(
                            egui::RichText::new(recommended_text)
                                .strong()
                                .color(self.config.accent_color),
                        )
                        .selected(self.active_preset == Some(2)),
                    )
                    .clicked()
                {
                    self.config.load_preset(2);
                    self.active_preset = Some(2);
                    self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
                }
                let blatant_text = self.t("blatant");
                if ui
                    .add(
                        Button::new(
                            egui::RichText::new(blatant_text)
                                .strong()
                                .color(Colors::ORANGE),
                        )
                        .selected(self.active_preset == Some(3)),
                    )
                    .clicked()
                {
                    self.config.load_preset(3);
                    self.active_preset = Some(3);
                    self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
                }
                let rage_text = self.t("rage");
                if ui
                    .add(
                        Button::new(egui::RichText::new(rage_text).strong().color(Colors::RED))
                            .selected(self.active_preset == Some(4)),
                    )
                    .clicked()
                {
                    self.config.load_preset(4);
                    self.active_preset = Some(4);
                    self.send_message(Message::Config(Box::new(self.config.clone())), Target::Game);
                }
            });
        });
    }

    fn config_right(&mut self, ui: &mut Ui) {
        let mut clicked_config = None;
        let mut delete = None;

        for config in &self.available_configs {
            let Some(name) = config.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let is_active = *config == self.current_config;

            ui.horizontal(|ui| {
                let delete_button_width = 30.0;
                let row_gap = 8.0;
                let name_width = (ui.available_width() - delete_button_width - row_gap).max(120.0);

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

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let delete_text = self.t("delete");
                    if ui
                        .add(
                            Button::new("x")
                                .fill(Colors::RED.linear_multiply(0.16))
                                .stroke(egui::Stroke::new(1.0, Colors::RED))
                                .min_size(egui::vec2(delete_button_width, 24.0)),
                        )
                        .on_hover_text(delete_text)
                        .clicked()
                    {
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
                .global_style_mut(|style| style.visuals.selection.bg_fill = self.config.accent_color);
        }

        if let Some(config) = delete {
            delete_config(&config);
            self.available_configs = available_configs();
            self.current_config = self.available_configs[0].clone();
            self.config = parse_config(&self.current_config);
        }
    }
}
