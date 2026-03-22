use egui::Ui;

use crate::{
    config::Language,
    constants::cs2::GRENADES,
    ui::{
        app::App,
        color::Colors,
        grenades::{Grenade, write_grenades},
        gui::{
            helpers::{scroll, section},
            translations::Trans,
        },
    },
};

impl App {
    pub fn grenade_settings(&mut self, ui: &mut Ui) {
        let lang = self.config.language;
        scroll(ui, "grenade_settings", |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| {
                    if self.current_grenade.is_some() {
                        self.edit_grenade(ui, lang);
                    } else {
                        self.record_grenade(ui, lang);
                    }
                });

                cols[1].vertical(|ui| {
                    section(ui, Trans::get(lang, "lineups"), None, |ui| {
                        self.grenade_list(ui, lang);
                    });
                });
            });
        });
    }

    fn grenade_list(&mut self, ui: &mut Ui, lang: Language) {
        let mut should_write = false;

        let mut grenades = self.grenades.lock();
        if grenades.is_empty() {
            ui.label(
                egui::RichText::new(if lang == Language::Russian {
                    "Раскидок еще нет."
                } else {
                    "No lineups saved yet."
                })
                .small()
                .color(Colors::GRAY),
            );
            return;
        }

        for (map, map_grenades) in grenades.iter_mut() {
            let mut delete_grenade_index = None;

            ui.collapsing(egui::RichText::new(map).strong(), |ui| {
                for (index, grenade) in map_grenades.iter().enumerate() {
                    let active = match &self.current_grenade {
                        Some(g) => &g.0 == map && g.1 == index,
                        None => false,
                    };
                    ui.horizontal(|ui| {
                        if ui.selectable_label(active, &grenade.name).clicked() {
                            self.current_grenade = if active {
                                None
                            } else {
                                Some((map.to_owned(), index))
                            };
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button("🗑")
                                .on_hover_text(Trans::get(lang, "delete"))
                                .clicked()
                            {
                                delete_grenade_index = Some(index);
                            }
                        });
                    });
                }
                if let Some(index) = delete_grenade_index {
                    map_grenades.remove(index);
                    should_write = true;
                }
            });
        }

        if should_write {
            write_grenades(&grenades);
        }
    }

    fn record_grenade(&mut self, ui: &mut Ui, lang: Language) {
        section(ui, Trans::get(lang, "record_lineup"), None, |ui| {
            let data = self.data.read();

            if !data.in_game {
                ui.label(
                    egui::RichText::new(if lang == Language::Russian {
                        "Зайдите в игру для записи раскидок."
                    } else {
                        "Enter a game to record lineups."
                    })
                    .small()
                    .color(Colors::YELLOW),
                );
                return;
            }

            if !GRENADES.contains(&data.local_player.weapon) {
                ui.colored_label(
                    Colors::ORANGE,
                    if lang == Language::Russian {
                        "Возьмите гранату в руки."
                    } else {
                        "Hold a grenade to record."
                    },
                );
                return;
            }

            ui.label(format!("{}:", Trans::get(lang, "name")));
            ui.text_edit_singleline(&mut self.new_grenade.name);
            ui.add_space(4.0);
            ui.label(format!("{}:", Trans::get(lang, "description")));
            ui.text_edit_multiline(&mut self.new_grenade.description);

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.new_grenade.modifiers.jump,
                    Trans::get(lang, "jump"),
                );
                ui.checkbox(
                    &mut self.new_grenade.modifiers.duck,
                    Trans::get(lang, "duck"),
                );
            });

            ui.add_space(10.0);
            if ui
                .button(egui::RichText::new(Trans::get(lang, "save")).strong())
                .clicked()
            {
                let mut grenades = self.grenades.lock();
                let map_grenades = grenades.entry(data.map_name.clone()).or_default();

                let mut grenade = self.new_grenade.clone();
                grenade.position = data.local_player.position;
                grenade.view_angles = data.view_angles;
                grenade.weapon = data.local_player.weapon.clone();

                map_grenades.push(grenade);
                write_grenades(&grenades);
                self.new_grenade = Grenade::new();
            }
        });
    }

    fn edit_grenade(&mut self, ui: &mut Ui, lang: Language) {
        let (map, index) = self.current_grenade.as_ref().unwrap();
        let map = map.clone();
        let index = *index;

        section(ui, Trans::get(lang, "edit_lineup"), None, |ui| {
            let mut grenades = self.grenades.lock();
            let grenade = &mut grenades.get_mut(&map).unwrap()[index];

            ui.label(format!("{}:", Trans::get(lang, "name")));
            ui.text_edit_singleline(&mut grenade.name);
            ui.add_space(4.0);
            ui.label(format!("{}:", Trans::get(lang, "description")));
            ui.text_edit_multiline(&mut grenade.description);

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.checkbox(&mut grenade.modifiers.jump, Trans::get(lang, "jump"));
                ui.checkbox(&mut grenade.modifiers.duck, Trans::get(lang, "duck"));
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button(Trans::get(lang, "save")).clicked() {
                    write_grenades(&grenades);
                    self.current_grenade = None;
                }
                if ui.button(Trans::get(lang, "cancel")).clicked() {
                    self.current_grenade = None;
                }
            });
        });
    }
}
