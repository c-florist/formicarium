pub fn target_distance_sq(pos_x: f32, pos_y: f32, target_x: f32, target_y: f32) -> f32 {
    (target_x - pos_x).powi(2) + (target_y - pos_y).powi(2)
}

pub fn calculate_attraction_strength(distance_sq: f32, pheromone_strength: f32) -> f32 {
    pheromone_strength / (distance_sq.sqrt() + 1.0)
}

pub fn normalize_vector(dx: f32, dy: f32) -> Option<(f32, f32)> {
    let magnitude = (dx * dx + dy * dy).sqrt();
    if magnitude > 1e-6 {
        Some((dx / magnitude, dy / magnitude))
    } else {
        None
    }
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

    #[test]
    fn test_calculate_attraction_strength_with_distance() {
        let result = calculate_attraction_strength(16.0, 50.0);
        assert_eq!(result, 50.0 / (4.0 + 1.0));
    }

    #[test]
    fn test_calculate_attraction_strength_zero_distance() {
        let result = calculate_attraction_strength(0.0, 100.0);
        assert_eq!(result, 100.0 / 1.0);
    }

    #[test]
    fn test_normalize_vector_valid() {
        let result = normalize_vector(3.0, 4.0);
        assert!(result.is_some());
        let (dx, dy) = result.unwrap();
        assert!((dx - 0.6).abs() < 1e-6);
        assert!((dy - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_vector_zero() {
        let result = normalize_vector(0.0, 0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_normalize_vector_very_small() {
        let result = normalize_vector(1e-7, 1e-7);
        assert!(result.is_none());
    }
}
