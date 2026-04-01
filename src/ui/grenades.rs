use std::{collections::HashMap, fs::read_to_string};

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use utils::log;
use uuid::Uuid;

use crate::{config::BASE_PATH, constants::GRENADE_FILE_NAME, cs2::entity::weapon::Weapon};

pub type GrenadeList = HashMap<String, Vec<Grenade>>;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Grenade {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub position: Vec3,
    pub view_angles: Vec2,
    pub weapon: Weapon,
    #[serde(default)]
    pub modifiers: GrenadeModifiers,
}

impl Grenade {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GrenadeModifiers {
    pub jump: bool,
    pub duck: bool,
    pub run: bool,
}

pub fn read_grenades() -> GrenadeList {
    let path = BASE_PATH.join(GRENADE_FILE_NAME);
    if !path.exists() {
        log::info!("no grenade list found");
        return GrenadeList::default();
    }

    let grenade_list_file = match read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            log::warn!("failed to read grenade list file {}: {err}", path.display());
            return GrenadeList::default();
        }
    };
    match serde_json::from_str(&grenade_list_file) {
        Ok(list) => list,
        Err(err) => {
            log::warn!("grenade list file invalid: {err}");
            GrenadeList::default()
        }
    }
}

pub fn write_grenades(grenades: &GrenadeList) {
    let out = match serde_json::to_string(grenades) {
        Ok(out) => out,
        Err(err) => {
            log::warn!("failed to serialize grenade list: {err}");
            return;
        }
    };
    let path = BASE_PATH.join(GRENADE_FILE_NAME);
    if let Err(err) = std::fs::write(&path, out) {
        log::warn!(
            "failed to write grenade list file {}: {err}",
            path.display()
        );
    }
}
