use std::f64::consts::PI;

use super::polynomial::Polynomial;

pub fn generate_chebyshev_polynomials(n: usize) -> Vec<Polynomial> {
    let mut chebyshev_polynomials = vec![Polynomial::new(vec![1.0]), Polynomial::new(vec![0.0, 1.0])];
    for i in 2..n+1 {
        chebyshev_polynomials.push(chebyshev_polynomials[i-1].mul(2.0).increase_all_powers().add(&chebyshev_polynomials[i-2].mul(-1.0)));
    }
    chebyshev_polynomials
}

fn coefficient(chebyshev_polynomials: &Vec<Polynomial>, f: &impl Fn(f64) -> f64, n: usize, j: usize) -> f64 {
    let mut idx = 0.0;
    for k in 0..n {
        let x_k = f64::cos(PI * (k as f64 + 0.5) / (n as f64 + 1.0));
        idx += f(x_k) * chebyshev_polynomials[j].evaluate(x_k);
    }
    return 2.0 * idx / (n as f64 + 1.0)
}

/// f is the function to be approximated
/// ts is the vector of Chebychev polynomials
/// n is the order of the resulting polynomial
pub fn generate_chebyshev_approximation(chebyshev_polynomials: &Vec<Polynomial>, f: impl Fn(f64) -> f64, n: usize) -> Polynomial {
    let mut polynomial = Polynomial::new(vec![-0.5*coefficient(&chebyshev_polynomials, &f, n, 0)]);
    for j in 0..n {
        println!("{}", coefficient(chebyshev_polynomials, &f, n, j));
        polynomial = polynomial.add(&chebyshev_polynomials[j].mul(coefficient(&chebyshev_polynomials, &f, n, j)));
    }
    polynomial
}