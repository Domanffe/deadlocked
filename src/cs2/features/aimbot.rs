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

fn adaptive_smooth_value(
    base_smooth: f32,
    enabled: bool,
    strength: f32,
    distance: f32,
    fov_error: f32,
    max_fov: f32,
) -> f32 {
    if !enabled {
        return base_smooth;
    }

    let distance_scale = (distance / 1000.0).clamp(0.4, 1.6);
    let error_scale = (fov_error / max_fov.max(0.1)).clamp(0.35, 1.9);
    let adaptive_target = base_smooth * distance_scale * error_scale;
    let strength = strength.clamp(0.0, 1.0);
    base_smooth + (adaptive_target - base_smooth) * strength
}

fn micro_correction_damp(
    enabled: bool,
    fov_error: f32,
    threshold: f32,
    strength: f32,
) -> Option<f32> {
    if !enabled {
        return None;
    }

    let threshold = threshold.clamp(0.1, 5.0);
    if fov_error > threshold {
        return None;
    }

    let phase = (fov_error / threshold).clamp(0.0, 1.0);
    let min_strength = strength.clamp(0.05, 1.0);
    Some(min_strength + (1.0 - min_strength) * phase)
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
        let fov_error = angles_to_fov(&view_angles, &target_angle);

        let smooth = adaptive_smooth_value(
            config.smooth,
            config.adaptive_smoothing && !grenade,
            config.adaptive_smoothing_strength,
            self.target.distance,
            fov_error,
            config.fov,
        );

        let mut mouse_angles = vec2(
            aim_angles.y / sensitivity * 50.0,
            -aim_angles.x / sensitivity * 50.0,
        ) / (if grenade { 1.0 } else { smooth + 1.0 }).clamp(1.0, 20.0);

        if let Some(damp) = micro_correction_damp(
            config.micro_correction && !grenade,
            fov_error,
            config.micro_correction_fov,
            config.micro_correction_strength,
        ) {
            mouse_angles *= damp;

            // Prevent tiny oscillations when almost perfectly centered.
            if mouse_angles.length() < 0.05 {
                return;
            }
        }

        log::debug!(
            "aimbot mouse movement: {:.2}/{:.2}, smooth={:.2}, fov_error={:.2}",
            mouse_angles.x,
            mouse_angles.y,
            smooth,
            fov_error
        );

        if config.advanced_humanizer {
            mouse.move_rel_humanized(&mouse_angles, smooth, true);
        } else {
            mouse.move_rel(&mouse_angles);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{adaptive_smooth_value, micro_correction_damp};

    #[test]
    fn adaptive_smoothing_disabled_keeps_base_value() {
        let smooth = adaptive_smooth_value(4.0, false, 1.0, 800.0, 5.0, 10.0);
        assert!((smooth - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn adaptive_smoothing_strength_zero_keeps_base_value() {
        let smooth = adaptive_smooth_value(5.0, true, 0.0, 1200.0, 9.0, 12.0);
        assert!((smooth - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn adaptive_smoothing_scales_with_error_and_distance() {
        let smooth = adaptive_smooth_value(4.0, true, 1.0, 1600.0, 8.0, 10.0);
        assert!(smooth > 4.0);
    }

    #[test]
    fn micro_correction_returns_none_outside_threshold() {
        let damp = micro_correction_damp(true, 3.0, 1.0, 0.3);
        assert!(damp.is_none());
    }

    #[test]
    fn micro_correction_uses_min_strength_at_zero_error() {
        let damp = micro_correction_damp(true, 0.0, 1.2, 0.35).expect("expected damp");
        assert!((damp - 0.35).abs() < f32::EPSILON);
    }

    #[test]
    fn micro_correction_reaches_one_at_threshold() {
        let damp = micro_correction_damp(true, 1.0, 1.0, 0.2).expect("expected damp");
        assert!((damp - 1.0).abs() < 0.0001);
    }
}
