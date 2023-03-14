#![feature(test)]

#[path = "../guest/src/bin/bounded.rs"]
mod bounded;

#[cfg(test)]
mod tests {
    use crate::bounded::is_bounded;

    #[test]
    fn empty_list_points_is_bounded() {
        assert!(is_bounded(&[]));
    }

    #[test]
    fn point_in_bounds_is_bounded() {
        let point: (f32, f32) = (0.0, 0.0);
        assert!(is_bounded(&[point]));
    }

    #[test]
    fn point_outside_radius_isnt_bounded() {
        let point: (f32, f32) = (2.0, 0.0);
        assert!(!is_bounded(&[point]));
    }

    #[test]
    fn point_outside_circle_curve_isnt_bounded() {
        let point: (f32, f32) = (0.9, 0.9);
        assert!(!is_bounded(&[point]));
    }

    #[test]
    fn point_on_circle_edge_isnt_bounded() {
        let point: (f32, f32) = (1.0, 0.0);
        assert!(!is_bounded(&[point]));
    }
}
