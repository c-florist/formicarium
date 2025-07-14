pub fn target_distance_sq(pos_x: f32, pos_y: f32, target_x: f32, target_y: f32) -> f32 {
    (target_x - pos_x).powi(2) + (target_y - pos_y).powi(2)
}
