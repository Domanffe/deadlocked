use glam::vec2;
use utils::log;

use crate::{
    config::{Config, KeyMode},
    cs2::{
        CS2,
        entity::{player::Player, weapon_class::WeaponClass},
    },
    math::{angles_to_fov, vec2_clamp},
    os::mouse::Mouse,
};

#[derive(Debug, Default)]
pub struct Aimbot {
    pub active: bool,
}

impl CS2 {
    pub fn aimbot(&mut self, config: &Config, mouse: &mut Mouse) {
        let hotkey = config.aim.aimbot_hotkey;
        let config = self.aimbot_config(config);

        if !config.enabled {
            return;
        }

        let grenade = self.target_grenade.is_some();

        match config.mode {
            KeyMode::Hold => {
                if !self.input.is_key_pressed(hotkey) {
                    return;
                }
            }
            KeyMode::Toggle => {
                if self.input.key_just_pressed(hotkey) {
                    self.aim.active = !self.aim.active;
                }
                if !self.aim.active {
                    return;
                }
            }
        }

        if self.target.player.is_none() && self.target_grenade.is_none() {
            return;
        }

        let target = self.target.player.as_ref();

        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        let weapon_class = local_player.weapon_class(self);
        let disallowed_weapons = [
            WeaponClass::Unknown,
            WeaponClass::Knife,
            WeaponClass::Grenade,
        ];
        if disallowed_weapons.contains(&weapon_class) {
            return;
        }

        if config.flash_check && local_player.is_flashed(self) {
            return;
        }

        if !grenade && config.visibility_check {
            let Some(target) = target else {
                return;
            };
            if !target.visible(self, &local_player) {
                return;
            }
        }

        let target_angle = if grenade {
            let Some(grenade_target) = self.target_grenade.as_ref() else {
                return;
            };
            grenade_target.view_angles
        } else {
            let Some(target) = target else {
                return;
            };
            let mut smallest_fov = 360.0;
            let mut smallest_angle = glam::Vec2::ZERO;
            for bone in &config.bones {
                let bone_pos = target.bone_position(self, bone.u64());
                let angle =
                    self.angle_to_target(&local_player, &bone_pos, &self.target.previous_aim_punch);
                let fov = angles_to_fov(&local_player.view_angles(self), &angle);
                if fov < smallest_fov {
                    smallest_fov = fov;
                    smallest_angle = angle;
                }
            }
            smallest_angle
        };

        let view_angles = local_player.view_angles(self);
        if angles_to_fov(&view_angles, &target_angle)
            > (config.fov
                * if config.distance_adjusted_fov {
                    self.distance_scale(self.target.distance)
                } else {
                    1.0
                })
        {
            return;
        }

        if !grenade {
            let Some(target) = target else {
                return;
            };
            if !target.is_valid(self) {
                return;
            }
        }

        if local_player.shots_fired(self) < config.start_bullet {
            return;
        }

        let mut aim_angles = view_angles - target_angle;
        if aim_angles.y < -180.0 {
            aim_angles.y += 360.0
        }
        vec2_clamp(&mut aim_angles);

        let sensitivity = self.get_sensitivity() * local_player.fov_multiplier(self);

        let mouse_angles = vec2(
            aim_angles.y / sensitivity * 50.0,
            -aim_angles.x / sensitivity * 50.0,
        ) / (if grenade { 1.0 } else { config.smooth + 1.0 }).clamp(1.0, 20.0);

        log::debug!(
            "aimbot mouse movement: {:.2}/{:.2}",
            mouse_angles.x,
            mouse_angles.y
        );

        if config.advanced_humanizer {
            mouse.move_rel_humanized(&mouse_angles, config.smooth, true);
        } else {
            mouse.move_rel(&mouse_angles);
        }
    }
}
