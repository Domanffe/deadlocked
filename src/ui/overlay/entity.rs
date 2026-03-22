use std::time::Instant;

use egui::{Align2, Color32, FontId, Painter, Stroke};

use crate::{
    cs2::entity::{inferno::InfernoInfo, molotov::MolotovInfo, smoke::SmokeInfo},
    data::{Data, EntityInfo, GrenadeInfo},
    math::world_to_screen,
    ui::{app::App, overlay::convex_hull, trail::Trail},
};

impl App {
    pub fn draw_entity(&self, painter: &Painter, entity: &EntityInfo, data: &Data) {
        match entity {
            EntityInfo::Weapon { weapon, position } => {
                if !self.config.hud.dropped_weapons {
                    return;
                }
                let Some(position) = world_to_screen(position, data) else {
                    return;
                };
                painter.text(
                    position,
                    Align2::CENTER_CENTER,
                    format!("{weapon}"),
                    FontId::proportional(self.config.hud.font_size),
                    self.config.hud.text_color,
                );
            }
            EntityInfo::Inferno(inferno) => self.inferno(painter, data, inferno),
            EntityInfo::Smoke(smoke) => self.smoke(painter, data, smoke),
            EntityInfo::Molotov(molotov) => self.molotov(painter, data, molotov),
            EntityInfo::Flashbang(info) => {
                self.draw_grenade(painter, data, info, self.config.hud.flash_trail_color)
            }
            EntityInfo::HeGrenade(info) => {
                self.draw_grenade(painter, data, info, self.config.hud.he_trail_color)
            }
            EntityInfo::Decoy(info) => {
                self.draw_grenade(painter, data, info, self.config.hud.decoy_trail_color)
            }
        };
    }

    fn draw_grenade(
        &self,
        painter: &Painter,
        data: &Data,
        info: &GrenadeInfo,
        trail_color: Color32,
    ) {
        if !self.config.hud.grenade_trails {
            return;
        }
        let Some(position) = world_to_screen(&info.position, data) else {
            return;
        };
        self.text(painter, info.name, position, Align2::CENTER_CENTER, None);

        let stroke = Stroke::new(self.config.hud.line_width, trail_color);
        let Some(trail) = self.trails.get(&info.entity) else {
            return;
        };
        for window in trail.trail.windows(2) {
            if let [v1, v2] = window {
                let Some(v1) = world_to_screen(v1, data) else {
                    continue;
                };
                let Some(v2) = world_to_screen(v2, data) else {
                    continue;
                };
                painter.line_segment([v1, v2], stroke);
            }
        }
    }

    fn inferno(&self, painter: &Painter, data: &Data, inferno: &InfernoInfo) {
        use egui::Shape;

        if !self.config.hud.grenade_trails {
            return;
        }
        let hull: Vec<egui::Pos2> = convex_hull(&inferno.hull)
            .iter()
            .filter_map(|p| {
                let p = p + (p - inferno.position).clamp_length(60.0, 60.0);
                world_to_screen(&p, data)
            })
            .collect();
        if hull.len() < 3 {
            return;
        }

        let shape = Shape::convex_polygon(
            hull,
            Color32::from_rgba_unmultiplied(255, 0, 0, 127),
            Stroke::NONE,
        );
        painter.add(shape);

        self.draw_grenade(painter, data, &inferno.grenade(), Color32::TRANSPARENT);
    }

    fn smoke(&self, painter: &Painter, data: &Data, smoke: &SmokeInfo) {
        if !self.config.hud.grenade_trails {
            return;
        }
        self.draw_grenade(
            painter,
            data,
            &smoke.grenade(),
            self.config.hud.smoke_trail_color,
        );
    }

    fn molotov(&self, painter: &Painter, data: &Data, molotov: &MolotovInfo) {
        if !self.config.hud.grenade_trails {
            return;
        }
        if molotov.is_incendiary {
            self.draw_grenade(
                painter,
                data,
                &molotov.grenade(),
                self.config.hud.incendiary_trail_color,
            );
        } else {
            self.draw_grenade(
                painter,
                data,
                &molotov.grenade(),
                self.config.hud.molotov_trail_color,
            );
        }
    }

    pub fn update_trails(&mut self) {
        let data = self.data.read();
        for entity in &data.entities {
            let (entity_id, position) = match entity {
                EntityInfo::Inferno(info) => (info.entity, info.position),
                EntityInfo::Smoke(info) => (info.entity, info.position),
                EntityInfo::Molotov(info) => (info.entity, info.position),
                EntityInfo::Flashbang(info)
                | EntityInfo::HeGrenade(info)
                | EntityInfo::Decoy(info) => (info.entity, info.position),
                _ => continue,
            };
            if let Some(trail) = self.trails.get_mut(&entity_id) {
                if (position - *trail.trail.last().unwrap()).length() < 1.0 {
                    continue;
                }
                trail.trail.push(position);
                trail.last_update = Instant::now();
            } else {
                self.trails.insert(
                    entity_id,
                    Trail {
                        trail: vec![position],
                        last_update: Instant::now(),
                    },
                );
            }
        }

        let now = Instant::now();
        self.trails
            .retain(|_k, trail| now.duration_since(trail.last_update) < Trail::MAX_AGE);
    }
}
