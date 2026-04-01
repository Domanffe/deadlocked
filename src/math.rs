use glam::{Vec2, Vec3};

pub fn angles_from_vector(forward: &Vec3) -> Vec2 {
    let mut yaw;
    let mut pitch;

    // forward vector points up or down
    if forward.x == 0.0 && forward.y == 0.0 {
        yaw = 0.0;
        pitch = if forward.z > 0.0 { 270.0 } else { 90.0 };
    } else {
        yaw = forward.y.atan2(forward.x).to_degrees();
        if yaw < 0.0 {
            yaw += 360.0;
        }

        pitch = (-forward.z)
            .atan2(Vec2::new(forward.x, forward.y).length())
            .to_degrees();
        if pitch < 0.0 {
            pitch += 360.0;
        }
    }

    Vec2::new(pitch, yaw)
}

pub fn angles_to_fov(view_angles: &Vec2, aim_angles: &Vec2) -> f32 {
    let mut delta = view_angles - aim_angles;

    if delta.x > 180.0 {
        delta.x = 360.0 - delta.x;
    }
    delta.x = delta.x.abs();

    // clamp?
    delta.y = ((delta.y + 180.0) % 360.0 - 180.0).abs();

    delta.length()
}

pub fn vec2_clamp(vec: &mut Vec2) {
    if vec.x > 89.0 && vec.x <= 180.0 {
        vec.x = 89.0;
    }
    if vec.x > 180.0 {
        vec.x -= 360.0;
    }
    if vec.x < -89.0 {
        vec.x = -89.0;
    }
    vec.y = (vec.y + 180.0) % 360.0 - 180.0;
}

pub fn dist_to_line(point: Vec3, line_start: Vec3, line_end: Vec3) -> f32 {
    let line = line_end - line_start;
    let point_relative = point - line_start;
    let length_sq = line.length_squared();
    if length_sq < 0.0001 {
        return point.distance(line_start);
    }
    let t = (point_relative.dot(line) / length_sq).clamp(0.0, 1.0);
    let projection = line_start + line * t;
    point.distance(projection)
}

pub fn rotate_point(point: Vec2, center: Vec2, angle_rad: f32) -> Vec2 {
    let sin = angle_rad.sin();
    let cos = angle_rad.cos();

    let p = point - center;
    let x = p.x * cos - p.y * sin;
    let y = p.x * sin + p.y * cos;

    Vec2::new(x, y) + center
}

pub fn world_to_screen(position: &Vec3, data: &crate::data::Data) -> Option<egui::Pos2> {
    let vm = &data.view_matrix;
    let mut screen_position = Vec2::new(
        vm.x_axis.x * position.x
            + vm.x_axis.y * position.y
            + vm.x_axis.z * position.z
            + vm.x_axis.w,
        vm.y_axis.x * position.x
            + vm.y_axis.y * position.y
            + vm.y_axis.z * position.z
            + vm.y_axis.w,
    );

    let w = vm.w_axis.x * position.x
        + vm.w_axis.y * position.y
        + vm.w_axis.z * position.z
        + vm.w_axis.w;

    if w < 0.0001 {
        return None;
    }

    screen_position /= w;

    let half_size = Vec2::new(data.window_size.x * 0.5, data.window_size.y * 0.5);

    screen_position.x = half_size.x + 0.5 * screen_position.x * data.window_size.x + 0.5;
    screen_position.y = half_size.y - 0.5 * screen_position.y * data.window_size.y + 0.5;

    if screen_position.x < 0.0
        || screen_position.x > data.window_size.x
        || screen_position.y < 0.0
        || screen_position.y > data.window_size.y
    {
        return None;
    }

    Some(egui::pos2(screen_position.x, screen_position.y))
}

#[cfg(test)]
mod tests {
    use glam::{Vec2, vec3};

    use super::{angles_to_fov, dist_to_line, vec2_clamp};

    #[test]
    fn angles_to_fov_handles_yaw_wrap() {
        let view = Vec2::new(0.0, 359.0);
        let aim = Vec2::new(0.0, 1.0);
        let fov = angles_to_fov(&view, &aim);
        assert!((fov - 2.0).abs() < 1e-3);
    }

    #[test]
    fn vec2_clamp_limits_pitch_and_wraps_yaw() {
        let mut angles = Vec2::new(120.0, 370.0);
        vec2_clamp(&mut angles);
        assert!((angles.x - 89.0).abs() < 1e-6);
        assert!((angles.y - 10.0).abs() < 1e-6);
    }

    #[test]
    fn dist_to_line_clamps_to_segment_endpoints() {
        let d = dist_to_line(
            vec3(5.0, 5.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            vec3(10.0, 0.0, 0.0),
        );
        assert!((d - 5.0).abs() < 1e-6);
    }
}
