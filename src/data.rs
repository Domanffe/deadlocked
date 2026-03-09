use std::collections::HashMap;

use glam::{Mat4, Vec2, Vec3};
use serde::Serialize;

use crate::cs2::bones::Bones;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SoundType {
    Footstep,
    Gunshot,
    Weapon,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BacktrackRecord {
    pub position: Vec3,
    pub head: Vec3,
    pub bones: HashMap<Bones, Vec3>,
    pub timestamp: f32,
}

#[derive(Debug, Default, Serialize)]
pub struct Data {
    pub in_game: bool,
    pub is_ffa: bool,
    pub is_custom_mode: bool,
    pub map_name: String,
    pub window_position: Vec2,
    pub window_size: Vec2,
    pub view_matrix: Mat4,
    pub view_angles: Vec2,
    pub local_player: PlayerData,
    pub players: Vec<PlayerData>,
    pub friendlies: Vec<PlayerData>,
    pub entities: Vec<EntityInfo>,
    pub spectators: Vec<String>,
    pub bomb: BombData,
    pub weapon: crate::cs2::entity::weapon::Weapon,
    pub aimbot_active: bool,
    pub triggerbot_active: bool,
    pub esp_active: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlayerData {
    pub steam_id: u64,
    pub health: i32,
    pub armor: i32,
    pub position: Vec3,
    pub head: Vec3,
    pub name: String,
    pub weapon: crate::cs2::entity::weapon::Weapon,
    pub bones: HashMap<Bones, Vec3>,
    pub has_defuser: bool,
    pub has_helmet: bool,
    pub has_bomb: bool,
    pub visible: bool,
    pub color: i32,
    pub rotation: f32,
    pub sound: Option<SoundType>,
}

#[derive(Debug, Default, Serialize)]
pub struct BombData {
    pub planted: bool,
    pub timer: f32,
    pub position: Vec3,
    pub being_defused: bool,
    pub defuse_remain_time: f32,
}

#[derive(Debug, Clone, Serialize)]
pub enum EntityInfo {
    Weapon {
        weapon: crate::cs2::entity::weapon::Weapon,
        position: Vec3,
    },
    Inferno(crate::cs2::entity::inferno::InfernoInfo),
    Smoke(crate::cs2::entity::smoke::SmokeInfo),
    Molotov(crate::cs2::entity::molotov::MolotovInfo),
    Flashbang(GrenadeInfo),
    HeGrenade(GrenadeInfo),
    Decoy(GrenadeInfo),
}

#[derive(Debug, Clone, Serialize)]
pub struct GrenadeInfo {
    pub entity: u64,
    pub name: &'static str,
    pub position: Vec3,
}

impl GrenadeInfo {
    pub fn new(entity: u64, name: &'static str, cs2: &crate::cs2::CS2) -> Self {
        use crate::cs2::entity::player::Player;
        Self {
            entity,
            name,
            position: Player::entity(entity).position(cs2),
        }
    }
}
