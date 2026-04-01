use glam::Vec2;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use strum::IntoEnumIterator;

use crate::{
    config::{Config, TargetingMode},
    constants::cs2,
    cs2::{
        CS2,
        bones::Bones,
        entity::{player::Player, weapon_class::WeaponClass},
    },
    math::angles_to_fov,
};

#[derive(Debug, Default)]
pub struct Target {
    pub player: Option<Player>,
    pub angle: Vec2,
    pub distance: f32,
    pub bone_index: u64,
    pub local_pawn_index: u64,
    pub previous_aim_punch: Vec2,
    pub backtrack_history: RefCell<HashMap<u64, VecDeque<crate::data::BacktrackRecord>>>,
    pub sticky_pawn: u64,
    pub sticky_seen_at: Option<Instant>,
}

impl Target {
    pub fn reset(&mut self) {
        let history = std::mem::take(&mut *self.backtrack_history.borrow_mut());
        let sticky_pawn = self.sticky_pawn;
        let sticky_seen_at = self.sticky_seen_at;
        *self = Target::default();
        *self.backtrack_history.borrow_mut() = history;
        self.sticky_pawn = sticky_pawn;
        self.sticky_seen_at = sticky_seen_at;
    }
}

fn stickiness_adjusted_score(
    base_score: f32,
    sticky_enabled: bool,
    candidate_pawn: u64,
    sticky_pawn: u64,
    sticky_strength: f32,
) -> f32 {
    if sticky_enabled && candidate_pawn == sticky_pawn {
        return base_score * (1.0 - sticky_strength.clamp(0.0, 0.8));
    }
    base_score
}

fn sticky_within_grace(last_seen: Option<Instant>, now: Instant, grace_window: Duration) -> bool {
    last_seen.is_some_and(|seen_at| now.saturating_duration_since(seen_at) <= grace_window)
}

impl CS2 {
    pub fn find_target(&mut self, config: &Config) {
        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        let team = local_player.team(self);
        if team != cs2::TEAM_CT && team != cs2::TEAM_T {
            self.target.reset();
            return;
        }

        let weapon_class = local_player.weapon_class(self);

        let view_angles = local_player.view_angles(self);
        let ffa = self.is_ffa();
        let shots_fired = local_player.shots_fired(self);
        let aim_punch = match (weapon_class, local_player.aim_punch(self) * 2.0) {
            (WeaponClass::Sniper, _) => Vec2::ZERO,
            (_, punch) if punch.length() == 0.0 && shots_fired > 1 => {
                self.target.previous_aim_punch
            }
            (_, punch) => punch,
        };
        self.target.previous_aim_punch = aim_punch;

        let aimbot_config = self.aimbot_config(config);
        let targeting_mode = &aimbot_config.targeting_mode;
        let max_fov = aimbot_config.fov;
        let is_custom_mode = self.is_custom_game_mode();

        let mut best_score = f32::MAX;
        let eye_position = local_player.eye_position(self);
        let now = Instant::now();

        if let Some(player) = &self.target.player
            && !player.is_valid(self)
        {
            self.target.player = None;
        }

        let player_weapon = local_player.weapon(self);
        let player_position = local_player.position(self);

        self.target_grenade = None;

        'grenades: {
            let grenades = self.grenades.lock();
            let Some(grenades) = grenades.get(&self.current_map()) else {
                break 'grenades;
            };

            for grenade in grenades {
                if player_weapon != grenade.weapon {
                    continue;
                }
                let distance = (player_position - grenade.position).length();
                if distance > 24.0 {
                    continue;
                }

                let angle = grenade.view_angles;
                let fov = angles_to_fov(&view_angles, &angle);

                let fov_limit = max_fov;
                if fov > fov_limit {
                    continue;
                }

                self.target_grenade = Some(grenade.clone());
            }

            // prioritizes grenade over players (to trigger this, you must be very close to the grenade position and look near the angle)
            if self.target_grenade.is_some() {
                return;
            }
        }

        if self.players.is_empty() {
            self.target.reset();
            return;
        }

        let target_friendlies = aimbot_config.target_friendlies;
        let sticky_enabled = aimbot_config.target_stickiness;
        let sticky_strength = aimbot_config.stickiness_strength.clamp(0.0, 0.8);
        let grace_window = Duration::from_millis(aimbot_config.stickiness_grace_ms.clamp(30, 2000));

        let previous_target = self.target.player;

        for player in &self.players {
            if !(ffa || target_friendlies && is_custom_mode) && team == player.team(self) {
                continue;
            }

            let head_position = player.bone_position(self, Bones::Head.u64());
            let distance = eye_position.distance(head_position);
            let angle = self.angle_to_target(&local_player, &head_position, &aim_punch);
            let fov = angles_to_fov(&view_angles, &angle);

            let fov_limit = max_fov * self.distance_scale(distance);
            if fov > fov_limit {
                continue;
            }

            if aimbot_config.visibility_check && !player.visible(self, &local_player) {
                continue;
            }

            let base_score = match targeting_mode {
                TargetingMode::Fov => fov,
                TargetingMode::Distance => distance,
            };
            let score = stickiness_adjusted_score(
                base_score,
                sticky_enabled,
                player.pawn,
                self.target.sticky_pawn,
                sticky_strength,
            );

            let should_select = score < best_score;

            if should_select {
                best_score = score;

                self.target.player = Some(*player);
                self.target.angle = angle;
                self.target.distance = distance;
                self.target.bone_index = Bones::Head.u64();
            }
        }

        if self.target.player.is_none()
            && sticky_enabled
            && let Some(previous_target) = previous_target
            && previous_target.is_valid(self)
            && sticky_within_grace(self.target.sticky_seen_at, now, grace_window)
        {
            self.target.player = Some(previous_target);
        }

        let Some(target) = &self.target.player else {
            return;
        };

        self.target.sticky_pawn = target.pawn;
        self.target.sticky_seen_at = Some(now);

        // update target angle
        let mut smallest_fov = 360.0;
        for bone in Bones::iter() {
            let bone_position = target.bone_position(self, bone.u64());
            let distance = eye_position.distance(bone_position);
            let angle = self.angle_to_target(&local_player, &bone_position, &aim_punch);
            let fov = angles_to_fov(&view_angles, &angle);

            if fov < smallest_fov {
                smallest_fov = fov;

                self.target.angle = angle;
                self.target.distance = distance;
                self.target.bone_index = bone.u64();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{stickiness_adjusted_score, sticky_within_grace};
    use std::time::{Duration, Instant};

    #[test]
    fn stickiness_reduces_score_for_same_target() {
        let score = stickiness_adjusted_score(10.0, true, 123, 123, 0.4);
        assert!((score - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn stickiness_does_not_reduce_for_other_target() {
        let score = stickiness_adjusted_score(10.0, true, 100, 200, 0.4);
        assert!((score - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn sticky_grace_window_respected() {
        let now = Instant::now();
        let recent = now.checked_sub(Duration::from_millis(80));
        let stale = now.checked_sub(Duration::from_millis(500));

        assert!(sticky_within_grace(recent, now, Duration::from_millis(120)));
        assert!(!sticky_within_grace(stale, now, Duration::from_millis(120)));
    }
}
