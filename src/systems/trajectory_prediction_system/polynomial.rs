use std::collections::VecDeque;

use roots::find_roots_eigen;

#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }

    pub fn add(&self, other: &Self) -> Polynomial {
        let n = usize::min(self.coefficients.len(), other.coefficients.len());
        let (mut primary_coefficients, secondary_coefficients) = if self.coefficients.len() > other.coefficients.len() {
            (self.coefficients.clone(), &other.coefficients)
        } else {
            (other.coefficients.clone(), &self.coefficients)
        };
        for i in 0..n {
            primary_coefficients[i] += secondary_coefficients[i]
        }
        Polynomial::new(primary_coefficients)
    }

    pub fn mul(&self, x: f64) -> Polynomial {
        let mut new_coefficients = self.coefficients.clone();
        for coefficient in &mut new_coefficients {
            *coefficient *= x;
        }
        Polynomial::new(new_coefficients)
    }

    pub fn increase_all_powers(&self) -> Polynomial {
        let mut new_coefficients = self.coefficients.clone();
        new_coefficients.insert(0, 0.0);
        Polynomial::new(new_coefficients)
    }

    pub fn differentiate(&self) -> Self {
        let mut new_coefficients = self.coefficients.clone();
        new_coefficients.remove(0);
        let mut i = 1.0;
        for x in &mut new_coefficients {
            *x *= i;
            i += 1.0;
        }
        Polynomial::new(new_coefficients)
    }

    fn normalize(&self) -> Self {
        let mut new_coefficients = self.coefficients.clone();
        let largest_degree_coefficient = new_coefficients.pop().expect("Polynomial vector cannot be empty");
        for x in &mut new_coefficients {
            *x /= largest_degree_coefficient;
        }
        Polynomial::new(new_coefficients)
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut idx = 0.0;
        let mut x_term = 1.0;
        for coefficient in &self.coefficients {
            idx += coefficient * x_term;
            x_term *= x;
        }
        idx
    }

    pub fn solve(&self) -> VecDeque<f64> {
        find_roots_eigen(self.normalize().coefficients)
    }
}