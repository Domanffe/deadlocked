use glam::vec2;
use crate::utils::log;

use crate::{
    config::{Config, KeyMode},
    cs2::{CS2, entity::player::Player},
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

        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        if config.flash_check && local_player.is_flashed(self) {
            return;
        }

        if let Some(target_player) = self.target.player.as_ref() {
            if !grenade && config.visibility_check && !target_player.visible(self, &local_player) {
                return;
            }
            if !grenade && !target_player.is_valid(self) {
                return;
            }
        } else if !grenade {
            return;
        }

        let target_angle = {
            let mut smallest_fov = 360.0;
            let mut smallest_angle = glam::Vec2::ZERO;
            if grenade {
                if let Some(target_grenade) = self.target_grenade.as_ref() {
                    let angle = target_grenade.view_angles;
                    let fov = angles_to_fov(&local_player.view_angles(self), &angle);
                    if fov < smallest_fov {
                        smallest_angle = angle;
                    }
                }
            } else if let Some(target_player) = self.target.player.as_ref() {
                let velocity = if config.prediction {
                    target_player.velocity(self)
                } else {
                    glam::Vec3::ZERO
                };

                for bone in &config.bones {
                    let mut bone_pos = target_player.bone_position(self, bone.u64());
                    
                    if config.visibility_check {
                        let eye_pos = local_player.eye_position(self);
                        
                        // Map geometry check
                        if let Some(bvh) = &self.bvh {
                            if !bvh.has_line_of_sight(eye_pos, bone_pos) {
                                continue;
                            }
                        }

                        // Volumetric smoke check
                        if self.is_line_blocked_by_smoke(eye_pos, bone_pos) {
                            continue;
                        }

                        // Game spotted mask (as extra fallback for other dynamic blockers)
                        let spotted_mask = target_player.spotted_mask(self);
                        if (spotted_mask & (1 << self.target.local_pawn_index)) == 0 {
                            continue;
                        }
                    }

                    if config.prediction {
                        bone_pos += velocity * config.prediction_factor * 0.01;
                    }

                    let angle = self.angle_to_target(
                        &local_player,
                        &bone_pos,
                        &self.target.previous_aim_punch,
                    );
                    let fov = angles_to_fov(&local_player.view_angles(self), &angle);
                    if fov < smallest_fov {
                        smallest_fov = fov;
                        smallest_angle = angle;
                    }
                }
            }
            smallest_angle
        };

        if target_angle == glam::Vec2::ZERO {
            return;
        }

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

        if local_player.shots_fired(self) < config.start_bullet {
            return;
        }

        let mut aim_angles = view_angles - target_angle;
        if aim_angles.y < -180.0 {
            aim_angles.y += 360.0
        }
        vec2_clamp(&mut aim_angles);

        let sensitivity = self.get_sensitivity() * local_player.fov_multiplier(self);

        let smooth = if grenade { 1.0 } else { config.smooth + 1.0 }.clamp(1.0, 20.0);
        let mouse_angles = vec2(
            aim_angles.y / sensitivity * 50.0,
            -aim_angles.x / sensitivity * 50.0,
        ) / smooth;

        log::debug!(
            "aimbot mouse movement: {:.2}/{:.2}",
            mouse_angles.x,
            mouse_angles.y
        );
        mouse.move_rel_humanized(&mouse_angles, smooth);
    }
}
