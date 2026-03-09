use glam::Vec3;
use serde::Serialize;

use crate::{cs2::CS2, data::GrenadeInfo};

#[derive(Debug, Clone, Copy)]
pub struct Inferno {
    pub controller: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct InfernoInfo {
    pub entity: u64,
    pub position: Vec3,
    pub hull: Vec<Vec3>,
}

impl Inferno {
    pub fn new(controller: u64) -> Self {
        Self { controller }
    }

    pub fn info(&self, cs2: &CS2) -> InfernoInfo {
        let is_burning: [u8; 64] = cs2
            .process
            .read_or_zeroed(self.controller + cs2.offsets.inferno.is_burning);
        let fire_count: i32 = cs2
            .process
            .read(self.controller + cs2.offsets.inferno.fire_count);
        let fire_positions: [Vec3; 64] = cs2
            .process
            .read_or_zeroed(self.controller + cs2.offsets.inferno.fire_positions);

        let mut hull = vec![];
        let mut position = Vec3::ZERO;
        let mut count = 0;

        for i in 0..(fire_count as usize).min(64) {
            if is_burning[i] == 0 {
                continue;
            }

            hull.push(fire_positions[i]);
            position += fire_positions[i];
            count += 1;
        }

        if count > 0 {
            position /= count as f32;
        }

        InfernoInfo {
            entity: self.controller,
            position,
            hull,
        }
    }
}

impl InfernoInfo {
    pub fn grenade(&self) -> GrenadeInfo {
        GrenadeInfo {
            entity: self.entity,
            position: self.position,
            name: "Inferno",
        }
    }
}
