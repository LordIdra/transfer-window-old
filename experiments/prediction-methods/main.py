from math import sqrt
from chebyshev import *

a_1 = 9.5;
b_1 = 9.2;
phi_1 = 3.7;

a_2 = 3.7;
b_2 = 1.5;
phi_2 = 0.9;

soi = 1.0;

def c(a: float, b: float) -> float:
    return sqrt(a**2 - b**2)

def eccentricity(a: float, b: float) -> float:
    return sqrt(1.0 - b**2 / a**2)

def ellipse_1(theta: float) -> (float, float):
    e_1 = eccentricity(a_1, b_1);
    radius = (a_1 * (1.0 - e_1**2)) / (1.0 + e_1 * cos(theta - phi_1))
    return (radius * cos(theta), radius * sin(theta))

# def inner_circle(theta: float) -> (float, float):
#     c0 = c(a_1, b_1)
#     r = b_1 - soi
#     return (r*cos(theta) - c0*cos(phi_1), r*sin(theta) - c0*sin(phi_1))

# def outer_circle(theta: float) -> (float, float):
#     c1 = c(a_1, b_1)
#     r = a_1 + soi
#     return (r*cos(theta) - c1*cos(phi_1), r*sin(theta) - c1*sin(phi_1))

def ellipse_2(theta: float) -> (float, float):
    e_2 = eccentricity(a_2, b_2);
    radius = (a_2 * (1.0 - e_2**2)) / (1.0 + e_2 * cos(theta - phi_2));
    return (radius * cos(theta), radius * sin(theta))
    # return (a_2*cos(theta)*cos(phi) - b_2*sin(theta)*sin(phi) + c_x,
    #         a_2*cos(theta)*sin(phi) + b_2*sin(theta)*cos(phi) + c_y)

def distance_squared(theta: float) -> float:
    theta *= pi
    point_1 = ellipse_2(theta)
    point_2 = ellipse_1(theta)
    distance_vector = (point_1[0] - point_2[0], point_1[1] - point_2[1])
    distance = (distance_vector[0]**2 + distance_vector[1]**2)
    return distance

def f(x: float) -> float:
    return sin(x)

x_axis = []
actual = []
expected = []
for i in range(-100, 100):
    theta_1 = (i / 100)
    theta_2 = theta_1*pi
    x_axis.append(theta_2)
    actual.append(approximate(distance_squared, 10, theta_1))
    expected.append(distance_squared(theta_1))

plt.plot(x_axis, expected)
plt.plot(x_axis, actual)
plt.show()