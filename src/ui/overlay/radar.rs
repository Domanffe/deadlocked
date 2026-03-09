use egui::{Color32, Painter, Pos2, Rect, Stroke, StrokeKind, vec2};

use crate::{data::Data, ui::app::App};

impl App {
    pub fn draw_radar(&self, painter: &Painter, data: &Data) {
        if !self.config.radar.enabled {
            return;
        }

        let scale = self.config.radar.scale;
        let size = self.config.radar.size;
        let radar_pos = Pos2::new(self.config.radar.position[0], self.config.radar.position[1]);
        let center = radar_pos + vec2(size / 2.0, size / 2.0);
        let radar_rect = Rect::from_min_size(radar_pos, vec2(size, size));

        // Draw radar background
        painter.rect(
            radar_rect,
            egui::CornerRadius::same(0),
            Color32::from_black_alpha(150),
            Stroke::new(1.0, self.config.accent_color),
            StrokeKind::Outside,
        );

        // Draw cross lines
        painter.line_segment(
            [
                Pos2::new(center.x, radar_pos.y),
                Pos2::new(center.x, radar_pos.y + size),
            ],
            Stroke::new(1.0, Color32::from_white_alpha(50)),
        );
        painter.line_segment(
            [
                Pos2::new(radar_pos.x, center.y),
                Pos2::new(radar_pos.x + size, center.y),
            ],
            Stroke::new(1.0, Color32::from_white_alpha(50)),
        );

        let local_pos = data.local_player.position;
        let local_yaw = data.local_player.rotation;

        let draw_points = |entities: &Vec<crate::data::PlayerData>, color: Color32| {
            for player in entities {
                if player.health <= 0 {
                    continue;
                }

                let delta = player.position - local_pos;

                // Rotate point based on local player yaw
                let (sin_y, cos_y) = local_yaw.to_radians().sin_cos();
                let rot_x = delta.x * sin_y - delta.y * cos_y;
                let rot_y = -(delta.x * cos_y + delta.y * sin_y);

                // Apply scale
                let mut rot_x = rot_x * (scale / 10.0);
                let mut rot_y = rot_y * (scale / 10.0);

                // Clamp to radar bounds
                let half_size = size / 2.0 - 4.0; // Margin
                if rot_x.abs() > half_size || rot_y.abs() > half_size {
                    let max_abs = rot_x.abs().max(rot_y.abs());
                    rot_x = (rot_x / max_abs) * half_size;
                    rot_y = (rot_y / max_abs) * half_size;
                }

                let point = center + vec2(rot_x, rot_y);

                painter.circle_filled(point, 3.0, color);
                painter.circle_stroke(point, 3.0, Stroke::new(1.0, Color32::BLACK));
            }
        };

        // Draw enemies
        draw_points(&data.players, Color32::RED);

        // Draw friendlies if enabled
        if self.config.player.show_friendlies {
            draw_points(&data.friendlies, Color32::GREEN);
        }

        // Draw local player
        painter.circle_filled(center, 3.5, self.config.accent_color);
        painter.circle_stroke(center, 3.5, Stroke::new(1.0, Color32::BLACK));

        // Draw local player view angle indicator
        let fov_rad = std::f32::consts::PI / 4.0; // Rough FOV indication
        let length = 15.0;
        let p1 = center + vec2(0.0, -length);
        let p2 = center + vec2((fov_rad).sin() * length, -((fov_rad).cos() * length));
        let p3 = center + vec2((-fov_rad).sin() * length, -((-fov_rad).cos() * length));

        painter.line_segment([center, p1], Stroke::new(1.0, Color32::WHITE));
        painter.line_segment(
            [center, p2],
            Stroke::new(1.0, Color32::from_white_alpha(100)),
        );
        painter.line_segment(
            [center, p3],
            Stroke::new(1.0, Color32::from_white_alpha(100)),
        );
    }
}
