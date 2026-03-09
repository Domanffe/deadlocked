use glam::Vec3;
use serde::Serialize;

use crate::{
    cs2::{CS2, entity::player::Player},
    data::GrenadeInfo,
};

#[derive(Debug, Clone, Copy)]
pub struct Molotov {
    pub controller: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MolotovInfo {
    pub entity: u64,
    pub position: Vec3,
    pub is_incendiary: bool,
}

impl Molotov {
    pub fn new(controller: u64) -> Self {
        Self { controller }
    }

    pub fn info(&self, cs2: &CS2) -> MolotovInfo {
        let position = Player::entity(self.controller).position(cs2);
        let is_incendiary: u8 = cs2
            .process
            .read(self.controller + cs2.offsets.molotov.is_incendiary);

        MolotovInfo {
            entity: self.controller,
            position,
            is_incendiary: is_incendiary != 0,
        }
    }
}

impl MolotovInfo {
    pub fn grenade(&self) -> GrenadeInfo {
        GrenadeInfo {
            entity: self.entity,
            position: self.position,
            name: if self.is_incendiary {
                "Incendiary"
            } else {
                "Molotov"
            },
        }
    }
}
