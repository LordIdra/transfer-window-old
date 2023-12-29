from math import sin, cos, pi
from random import randint

def solve_kepler_equation(eccentricity: float, mean_anomaly: float, start_offset: float) -> float:
    max_delta_squared = (1.0e-5)**2;
    max_attempts = 500;
    # Choosing an initial seed: https://www.aanda.org/articles/aa/full_html/2022/02/aa41423-21/aa41423-21.html#S5
    # Yes, they're actually serious about that 0.999999 thing (lmao)
    eccentric_anomaly = (mean_anomaly + start_offset
        + (0.999999 * 4.0 * eccentricity * mean_anomaly * (pi - mean_anomaly))
        / (8.0 * eccentricity * mean_anomaly + 4.0 * eccentricity * (eccentricity - pi) + pi.powi(2)))
    attempts = 0;
    while True:
        delta = -(eccentric_anomaly - eccentricity * sin(eccentric_anomaly) - mean_anomaly) / (1.0 - eccentricity * cos(eccentric_anomaly))
        if delta.powi(2) < max_delta_squared:
            break;
        if attempts > max_attempts:
            # Try with different start value
            start_offset = (0.01 * randint(0, 100) - 0.5) * 5.0
            return solve_kepler_equation(eccentricity, mean_anomaly, start_offset)
        eccentric_anomaly += delta
        attempts += 1
    return eccentric_anomaly