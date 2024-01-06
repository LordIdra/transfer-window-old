const ITERATIONS: usize = 20;

pub fn bisection_to_find_minimum(function: &impl Fn(f64) -> f64, min: f64, max: f64) -> f64 {
    let mut low = min;
    let mut high = max;
    let mut mid = min + max / 2.0;
    for _ in 0..ITERATIONS {
        if function(mid).is_sign_positive() && function(low).is_sign_positive() || function(mid).is_sign_negative() && function(low).is_sign_negative() {
            low = mid;
        } else {
            high = mid;
        }
        mid = (low + high) / 2.0;
    }
    mid
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::systems::trajectory_prediction_system::bisection::bisection_to_find_minimum;

    #[test]
    fn test_bisection_to_find_minimum_1() {
        let function = |x: f64| f64::sin(x);
        assert!(bisection_to_find_minimum(&function, -PI/2.0, PI/2.0).abs() < 1.0e-2);
    }

    #[test]
    fn test_bisection_to_find_minimum_2() {
        let function = |x: f64| x.powi(2) - 4.0;
        assert!(bisection_to_find_minimum(&function, 0.0,  10.0).abs() - 2.0 < 1.0e-2);
        assert!(bisection_to_find_minimum(&function, -10.0, 0.0).abs() - 2.0 < 1.0e-2);
    }
}