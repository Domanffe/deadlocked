use egui::{Align2, Color32, Painter, Stroke, pos2};

use crate::{
    cs2::entity::weapon_class::WeaponClass, data::Data, math::world_to_screen, ui::app::App,
};

impl App {
    pub fn overlay_debug(&self, painter: &Painter, data: &Data) {
        if self.config.hud.debug {
            painter.line(
                vec![pos2(0.0, 0.0), pos2(data.window_size.x, data.window_size.y)],
                Stroke::new(self.config.hud.line_width, Color32::WHITE),
            );
            painter.line(
                vec![pos2(data.window_size.x, 0.0), pos2(0.0, data.window_size.y)],
                Stroke::new(self.config.hud.line_width, Color32::WHITE),
            );
        }
    }

    pub fn draw_bomb_timer(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.bomb_timer || !data.bomb.planted {
            return;
        }

        if let Some(pos) = world_to_screen(&data.bomb.position, data) {
            self.text(
                painter,
                format!("{:.3}", data.bomb.timer),
                pos,
                Align2::CENTER_CENTER,
                None,
            );
            if data.bomb.being_defused {
                self.text(
                    painter,
                    format!("defusing {:.3}", data.bomb.defuse_remain_time),
                    pos2(pos.x, pos.y + self.config.hud.font_size),
                    Align2::CENTER_CENTER,
                    None,
                );
            }
        }

        let fraction = (data.bomb.timer / 40.0).clamp(0.0, 1.0);
        let color = self.health_color((fraction * 100.0) as i32, 255);
        painter.line(
            vec![
                pos2(0.0, data.window_size.y),
                pos2(data.window_size.x * fraction, data.window_size.y),
            ],
            Stroke::new(self.config.hud.line_width * 3.0, color),
        );
    }

    pub fn draw_bomb_damage(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.bomb_damage || !data.bomb.planted || !data.in_game {
            return;
        }

        let bomb_pos = data.bomb.position;
        let player_pos = data.local_player.position;
        let distance = bomb_pos.distance(player_pos);
        let max_damage = 500.0;
        let radius = 1750.0;

        if distance > radius {
            return;
        }

        let mut damage = max_damage * (1.0 - (distance / radius));

        if data.local_player.armor > 0 {
            damage *= 0.5;
        }

        let damage = damage.round() as i32;
        if damage <= 0 {
            return;
        }

        let is_lethal = damage >= data.local_player.health;
        let text = if is_lethal {
            format!("LETHAL DAMAGE: -{}", damage)
        } else {
            format!("BOMB DAMAGE: -{}", damage)
        };

        let color = if is_lethal {
            crate::ui::color::Colors::RED
        } else {
            crate::ui::color::Colors::YELLOW
        };

        let center_x = data.window_size.x / 2.0;
        let pos = pos2(center_x, data.window_size.y * 0.75);

        self.text(painter, text, pos, Align2::CENTER_CENTER, Some(color));
    }

    pub fn draw_spectator_list(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.spectator_list || data.spectators.is_empty() {
            return;
        }

        let mut text = String::from("Spectators:\n");
        for name in &data.spectators {
            text.push_str(&format!("  {}\n", name));
        }

        let padding = 10.0;
        let pos = pos2(data.window_size.x - padding, padding);

        self.text(
            painter,
            text,
            pos,
            Align2::RIGHT_TOP,
            Some(crate::ui::color::Colors::ACCENT),
        );
    }

    pub fn draw_keybind_list(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.keybind_list {
            return;
        }

        let mut active_binds = Vec::new();

        if data.esp_active {
            active_binds.push(("ESP", "Enabled"));
        }

        if data.aimbot_active {
            active_binds.push(("Aimbot", "Active"));
        }

        if data.triggerbot_active {
            active_binds.push(("Triggerbot", "Active"));
        }

        if active_binds.is_empty() {
            return;
        }

        let padding = 12.0;
        let mut y_offset = data.window_size.y * 0.4;

        for (name, status) in active_binds {
            let text = format!("{}: {}", name, status);
            let pos = pos2(data.window_size.x - padding, y_offset);

            self.text(
                painter,
                text,
                pos,
                Align2::RIGHT_TOP,
                Some(crate::ui::color::Colors::TEXT),
            );
            y_offset += self.config.hud.font_size + 4.0;
        }
    }

    pub fn draw_fov_circle(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.fov_circle || !data.in_game {
            return;
        }

        let weapon_config = self.aimbot_config(&data.weapon);

        let aim_fov = weapon_config.fov;

        if weapon_config.distance_adjusted_fov {
            self.draw_distance_scaled_fov_circle(painter, data, aim_fov, 125.0, Color32::GREEN);
            self.draw_distance_scaled_fov_circle(painter, data, aim_fov, 250.0, Color32::YELLOW);
            self.draw_distance_scaled_fov_circle(painter, data, aim_fov, 500.0, Color32::RED);
        } else {
            self.draw_simple_fov_circle(painter, data, aim_fov, Color32::WHITE);
        }
    }

    fn get_current_fov(&self) -> f32 {
        (if self.config.misc.fov_changer {
            self.config.misc.desired_fov
        } else {
            crate::constants::cs2::DEFAULT_FOV
        }) as f32
    }

    fn calculate_fov_radius(&self, data: &Data, target_fov: f32) -> f32 {
        let current_fov = self.get_current_fov();
        let screen_width = data.window_size.x;

        let current_fov_tan = (current_fov.to_radians() / 2.0).tan();
        if current_fov_tan == 0.0 {
            return 0.0;
        }

        let target_fov_tan = (target_fov.to_radians() / 2.0).tan();
        (target_fov_tan / current_fov_tan) * (screen_width / 2.0)
    }

    fn draw_fov_circle_impl(&self, painter: &Painter, data: &Data, radius: f32, color: Color32) {
        let center = pos2(data.window_size.x / 2.0, data.window_size.y / 2.0);
        let stroke = Stroke::new(self.config.hud.line_width, color);
        painter.circle_stroke(center, radius, stroke);
    }

    fn get_distance_fov_scale(&self, distance: f32) -> f32 {
        (5.0 - (distance / 125.0)).max(1.0)
    }

    fn draw_simple_fov_circle(
        &self,
        painter: &Painter,
        data: &Data,
        target_fov: f32,
        color: Color32,
    ) {
        let radius = self.calculate_fov_radius(data, target_fov);
        self.draw_fov_circle_impl(painter, data, radius, color);
    }

    fn draw_distance_scaled_fov_circle(
        &self,
        painter: &Painter,
        data: &Data,
        base_aim_fov: f32,
        distance: f32,
        color: Color32,
    ) {
        let scale = self.get_distance_fov_scale(distance);
        let target_fov = base_aim_fov * scale;

        let radius = self.calculate_fov_radius(data, target_fov);
        self.draw_fov_circle_impl(painter, data, radius, color);
    }

    pub fn draw_sniper_crosshair(&self, painter: &Painter, data: &Data) {
        if !self.config.hud.sniper_crosshair
            || WeaponClass::from_string(data.weapon.as_ref()) != WeaponClass::Sniper
        {
            return;
        }

        painter.line(
            vec![
                pos2(data.window_size.x / 2.0, data.window_size.y / 2.0 - 50.0),
                pos2(data.window_size.x / 2.0, data.window_size.y / 2.0 + 50.0),
            ],
            Stroke::new(self.config.hud.line_width, self.config.hud.crosshair_color),
        );
        painter.line(
            vec![
                pos2(data.window_size.x / 2.0 - 50.0, data.window_size.y / 2.0),
                pos2(data.window_size.x / 2.0 + 50.0, data.window_size.y / 2.0),
            ],
            Stroke::new(self.config.hud.line_width, self.config.hud.crosshair_color),
        );
    }
}
