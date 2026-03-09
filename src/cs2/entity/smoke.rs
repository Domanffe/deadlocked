use glam::{Vec2, Vec3};
use serde::Serialize;

use crate::{
    cs2::{CS2, entity::player::Player},
    data::GrenadeInfo,
};

#[derive(Debug, Clone, Copy)]
pub struct Smoke {
    pub controller: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SmokeInfo {
    pub entity: u64,
    pub position: Vec3,
}

impl Smoke {
    pub fn new(controller: u64) -> Self {
        Self { controller }
    }

    pub fn color(&self, cs2: &CS2, color: &egui::Color32) {
        let current_color: Vec3 = cs2
            .process
            .read(self.controller + cs2.offsets.smoke.smoke_color);
        let new_color = Vec3::new(
            color.r() as f32 / 255.0,
            color.g() as f32 / 255.0,
            color.b() as f32 / 255.0,
        );
        if current_color != new_color {
            cs2.process
                .write(self.controller + cs2.offsets.smoke.smoke_color, new_color);
        }
    }

    pub fn disable(&self, cs2: &CS2) {
        let current: Vec2 = cs2
            .process
            .read(self.controller + cs2.offsets.smoke.smoke_color);
        if current != Vec2::ZERO {
            cs2.process
                .write(self.controller + cs2.offsets.smoke.smoke_color, Vec2::ZERO);
        }
    }

    pub fn info(&self, cs2: &CS2) -> SmokeInfo {
        let position = Player::entity(self.controller).position(cs2);

        SmokeInfo {
            entity: self.controller,
            position,
        }
    }
}

impl SmokeInfo {
    pub fn grenade(&self) -> GrenadeInfo {
        GrenadeInfo {
            entity: self.entity,
            position: self.position,
            name: "Smoke",
        }
    }
}
