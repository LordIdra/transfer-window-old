const DERIVATIVE_DELTA: f64 = 1.0e-2;
const MAX_DELTA: f64 = 1.0e-8;
const MAX_ITERATIONS: usize = 50;

/// Returns (first derivative, second derivative)
pub fn differentiate(f: &impl Fn(f64) -> f64, x: f64) -> (f64, f64) {
    let f_1 = f(x - DERIVATIVE_DELTA);
    let f_2 = f(x);
    let f_3 = f(x + DERIVATIVE_DELTA);

    let f_prime_1 = (f_2 - f_1) / DERIVATIVE_DELTA;
    let f_prime_2 = (f_3 - f_2) / DERIVATIVE_DELTA;
    let f_prime = (f_prime_1 + f_prime_2) / 2.0;
    let f_prime_prime = (f_prime_2 - f_prime_1) / DERIVATIVE_DELTA;
    (f_prime, f_prime_prime)
}

pub fn newton_raphson_to_find_minimum(function: &impl Fn(f64) -> f64, starting_x: f64) -> Option<f64> {
    let mut x = starting_x;
    let mut i = 0;
    while i < MAX_ITERATIONS {
        let (first, second) = differentiate(function, x);
        let delta = -first/second;
        if delta.abs() < MAX_DELTA {
            return Some(x);
        }
        x += delta;
        i += 1;
    }
    return None;
}

pub fn newton_raphson(function: &impl Fn(f64) -> f64, starting_x: f64) -> Option<f64> {
    let mut x = starting_x;
    let mut i = 0;
    while i < MAX_ITERATIONS {
        let f_1 = function(x);
        let f_2 = function(x + DERIVATIVE_DELTA);
        let derivative = (f_2 - f_1) / DERIVATIVE_DELTA;
        let delta = -f_1 / derivative;
        if delta.abs() < MAX_DELTA {
            return Some(x);
        }
        x += delta;
        i += 1;
    }
    return None;
}

#[cfg(test)]
mod test {
    use crate::systems::trajectory_prediction_system::newton_raphson::newton_raphson;

    #[test]
    fn test_newton_raphson() {
        let function = |x: f64| x.powi(2) - 4.0;
        let starting_x = 4.0;
        assert!(newton_raphson(&function, starting_x).unwrap().abs() - 2.0 < 1.0e-3);
    }
}