from math import sin, cos, atan, pi, sqrt, tan

h = 0.0001
max_delta = 0.0001
max_iterations = 10
delta = 9999999999
t = 50;
i = 0
learning_rate = 0.1
expected_t = 993060

def mean_anomaly(period, time_of_periapsis, t):
    time_since_periapsis = t - time_of_periapsis
    return (2.0 * pi * time_since_periapsis) / period;

def solve_kepler_equation(t, period, time_of_periapsis, e):
    Ma = mean_anomaly(period, time_of_periapsis, t)
    max_delta = 0.0001
    delta = 999999
    eccentric_anomaly = Ma + (0.999999 * 4.0 * e * Ma * (pi - Ma)) / (8.0 * e * Ma + 4.0 * e * (e - pi) + pi**2)
    while delta > max_delta:
        delta = -(eccentric_anomaly - e * sin(eccentric_anomaly) - Ma) / (1.0 - e * cos(eccentric_anomaly))
        eccentric_anomaly += delta
    return eccentric_anomaly

def position(t, period, time_of_periapsis, a, e, w):
    eccentric_anomaly = solve_kepler_equation(t, period, time_of_periapsis, e)
    true_anomaly = 2.0 * atan(sqrt((1.0 + e) / (1.0 - e)) * tan(eccentric_anomaly / 2.0));
    theta = true_anomaly + w
    print(theta)
    r = (a * (1 - e**2)) / (1 + e*cos(theta - w))
    return (r*cos(theta), r*sin(theta))

def distance_squared(t):
    position_a = position(t, 1196638, 0, 2.4361e8, 0.96716, 1.578) # spacecraft
    position_b = position(t, 2413346, -1206673, 3.88866e8, 0.04278, -1.14159) # moon
    # print("spacecraft", position_a)
    # print("moon", position_b)
    return (position_a[0] - position_b[0])**2 + (position_a[1] - position_b[1])**2

def f(t):
    soi = 6.695e7
    return distance_squared(t) - soi**2

while abs(delta) > max_delta and i < max_iterations:
    f_t_minux_h = f(t - h)
    f_t = f(t)
    f_t_plus_h = f(t + h)
    f_prime_t = (f_t_plus_h - f_t_minux_h) / (2*h)

    delta = abs(learning_rate * -f_t / f_prime_t)

    t += delta
    i += 1

    print(i, t, f(t))
    print("")

print("Expected", expected_t)
print("Found", t)