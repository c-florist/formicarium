pub fn target_distance_sq(pos_x: f32, pos_y: f32, target_x: f32, target_y: f32) -> f32 {
    (target_x - pos_x).powi(2) + (target_y - pos_y).powi(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_distance_sq_positive() {
        assert_eq!(target_distance_sq(0.0, 0.0, 3.0, 4.0), 25.0);
    }

    #[test]
    fn test_target_distance_sq_zero() {
        assert_eq!(target_distance_sq(0.0, 0.0, 0.0, 0.0), 0.0);
    }
}
