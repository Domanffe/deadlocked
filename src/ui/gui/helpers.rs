use std::hash::Hash;

use egui::{Color32, DragValue, Event, RichText, Sense, Ui, Widget};

use crate::cs2::key_codes::KeyCode;

fn compact_toggle(ui: &mut Ui, value: &mut bool) -> bool {
    let text = if *value { "●" } else { "" };
    let button = egui::Button::new(RichText::new(text).color(Colors::WHITE).size(14.0))
        .fill(if *value {
            ui.visuals().selection.bg_fill
        } else {
            Colors::HIGHLIGHT
        })
        .stroke(if *value {
            egui::Stroke::new(1.0, ui.visuals().selection.stroke.color)
        } else {
            egui::Stroke::new(1.0, Colors::GRAY)
        })
        .corner_radius(8.0)
        .min_size(egui::vec2(24.0, 32.0));

    if ui.add(button).clicked() {
        *value = !*value;
        return true;
    }
    false
}

pub fn section(
    ui: &mut Ui,
    title: &str,
    enabled: Option<&mut bool>,
    add_body: impl FnOnce(&mut Ui),
) {
    let accent_color = ui.visuals().selection.bg_fill;
    egui::Frame::NONE
        .fill(Colors::HIGHLIGHT.linear_multiply(0.65))
        .corner_radius(10.0)
        .stroke(egui::Stroke::new(1.0, Colors::GRAY.linear_multiply(0.7)))
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let (rect, _response) =
                        ui.allocate_exact_size(egui::vec2(4.0, 18.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 3.0, accent_color);

                    ui.label(
                        egui::RichText::new(title)
                            .strong()
                            .size(17.0)
                            .color(Colors::TEXT),
                    );

                    if let Some(val) = enabled {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let _ = compact_toggle(ui, val);
                        });
                    }
                });
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(10.0);
                add_body(ui);
            });
        });
    ui.add_space(12.0);
}

use crate::ui::color::Colors;

pub fn scroll(ui: &mut Ui, id: &str, add_content: impl FnOnce(&mut Ui)) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, true])
        .id_salt(id)
        .show(ui, add_content);
}

pub fn checkbox(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let label_response = ui.add(
            egui::Label::new(RichText::new(label).color(Colors::TEXT))
                .sense(egui::Sense::click()),
        );
        if label_response.clicked() {
            *value = !*value;
            changed = true;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if compact_toggle(ui, value) {
                changed = true;
            }
        });
    });
    changed
}

pub fn checkbox_hover(ui: &mut Ui, label: &str, hover_text: &str, value: &mut bool) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let label_response = ui
            .add(
                egui::Label::new(RichText::new(label).color(Colors::TEXT))
                    .sense(egui::Sense::click()),
            )
            .on_hover_text(hover_text);
        if label_response.clicked() {
            *value = !*value;
            changed = true;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if compact_toggle(ui, value) {
                changed = true;
            }
        });
    });
    changed
}

pub fn drag(ui: &mut Ui, label: &str, drag: DragValue) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(Colors::TEXT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(drag).changed() {
                changed = true;
            }
        });
    });
    changed
}

pub fn slider(ui: &mut Ui, label: &str, slider: egui::Slider<'_>) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(Colors::TEXT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(slider).changed() {
                changed = true;
            }
        });
    });
    changed
}

pub fn combo_box<T: std::fmt::Debug + strum::IntoEnumIterator + PartialEq>(
    ui: &mut Ui,
    id: &str,
    label: &str,
    value: &mut T,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(Colors::TEXT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::ComboBox::from_id_salt(id)
                .width(130.0)
                .selected_text(RichText::new(format!("{:?}", *value)).color(Colors::TEXT))
                .show_ui(ui, |ui| {
                    for mode in T::iter() {
                        let text = format!("{:?}", &mode);
                        if ui.selectable_value(value, mode, text).clicked() {
                            changed = true;
                        }
                    }
                });
        });
    });
    changed
}

pub fn color_picker(ui: &mut Ui, label: &str, color: &mut Color32) -> bool {
    let [mut r, mut g, mut b, mut a] = color.to_srgba_unmultiplied();
    let res = ui
        .horizontal(|ui| {
            let (response, painter) =
                ui.allocate_painter(ui.spacing().interact_size, Sense::hover());
            painter.rect_filled(
                response.rect,
                ui.style().visuals.widgets.inactive.corner_radius,
                *color,
            );
            let mut res = ui.add(DragValue::new(&mut r).prefix("r: "));
            res = res.union(ui.add(DragValue::new(&mut g).prefix("g: ")));
            res = res.union(ui.add(DragValue::new(&mut b).prefix("b: ")));
            res = res.union(ui.add(DragValue::new(&mut a).prefix("a: ")));
            ui.label(label);
            res
        })
        .inner;

    let changed = res.changed();
    if changed {
        *color = Color32::from_rgba_premultiplied(r, g, b, a);
    }

    changed
}

pub fn keybind(ui: &mut Ui, id: &str, label: &str, keycode: &mut KeyCode) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(Colors::TEXT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(Keybind::new(keycode, id)).changed() {
                changed = true;
            }
        });
    });
    changed
}

pub struct Keybind<'gui> {
    keycode: &'gui mut KeyCode,
    id: egui::Id,
}

impl<'gui> Keybind<'gui> {
    pub fn new(keycode: &'gui mut KeyCode, id: impl Hash) -> Self {
        Self {
            keycode,
            id: egui::Id::new(id),
        }
    }
}

impl<'gui> Widget for Keybind<'gui> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let listening_id = ui.make_persistent_id(self.id);

        let mut listening = {
            let ctx = ui.ctx();
            ctx.memory(|mem| mem.data.get_temp::<bool>(listening_id).unwrap_or(false))
        };

        let text = if listening {
            "...".to_string()
        } else {
            format!("{:?}", self.keycode)
        };

        let mut response = ui.button(
            RichText::new(text)
                .color(if listening { Colors::WHITE } else { Colors::TEXT })
                .monospace(),
        );

        if response.clicked() {
            listening = !listening;
        }

        if response.secondary_clicked() {
            listening = false;
        }

        if listening {
            let input = ui.input(|i| {
                for event in &i.events {
                    if let Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } = event
                    {
                        if *key == egui::Key::F35 {
                            return KeyCode::from_egui_modifiers(*modifiers);
                        } else {
                            return KeyCode::from_egui(*key);
                        }
                    }

                    if let Event::PointerButton {
                        button,
                        pressed: true,
                        ..
                    } = event
                    {
                        return Some(KeyCode::from_egui_mouse(*button));
                    }
                }
                None
            });

            if let Some(input) = input {
                if input != KeyCode::Escape {
                    *self.keycode = input;
                    response.mark_changed();
                }
                listening = false;
            }
        }

        let ctx = ui.ctx();
        ctx.memory_mut(|mem| mem.data.insert_temp(listening_id, listening));

        response
    }
}
