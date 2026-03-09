use glam::Vec2;
use utils::log;

use crate::{
    config::Config,
    cs2::{
        CS2,
        entity::{player::Player, weapon_class::WeaponClass},
    },
    os::mouse::Mouse,
};

#[derive(Debug, Default)]
pub struct Recoil {
    previous: Vec2,
    unaccounted: Vec2,
}

impl CS2 {
    pub fn rcs(&mut self, config: &Config, mouse: &mut Mouse) {
        let config = self.rcs_config(config);

        if !config.enabled {
            return;
        }

        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        let weapon_class = local_player.weapon_class(self);
        let disallowed_weapons = [
            WeaponClass::Unknown,
            WeaponClass::Knife,
            WeaponClass::Grenade,
            WeaponClass::Pistol,
            WeaponClass::Shotgun,
        ];
        if disallowed_weapons.contains(&weapon_class) {
            return;
        }

        let shots_fired = local_player.shots_fired(self);
        let aim_punch = match (weapon_class, local_player.aim_punch(self)) {
            (WeaponClass::Sniper, _) => Vec2::ZERO,
            (_, punch) if punch.length() == 0.0 && shots_fired > 1 => self.recoil.previous,
            (_, punch) => punch,
        };

        if shots_fired < 1 {
            self.recoil.previous = aim_punch;
            self.recoil.unaccounted = Vec2::ZERO;
            return;
        }
        let sensitivity = self.get_sensitivity() * local_player.fov_multiplier(self);

        let mouse_angle = Vec2::new(
            (aim_punch.y - self.recoil.previous.y) / sensitivity * 100.0,
            -(aim_punch.x - self.recoil.previous.x) / sensitivity * 100.0,
        ) + self.recoil.unaccounted;
        let mouse_angle = mouse_angle / (config.smooth + 1.0).clamp(1.0, 2.0);

        self.recoil.unaccounted = Vec2::ZERO;

        // only if the aimbot is not active
        self.recoil.previous = aim_punch;
        if (0.0..1.0).contains(&mouse_angle.x) {
            self.recoil.unaccounted.x = mouse_angle.x;
        }
        if (0.0..1.0).contains(&mouse_angle.y) {
            self.recoil.unaccounted.y = mouse_angle.y;
        }

        log::debug!(
            "rcs mouse movement: {:.2}/{:.2}",
            mouse_angle.x,
            mouse_angle.y
        );
        mouse.move_rel(&mouse_angle)
    }
}
