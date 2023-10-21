from tkinter import *
from math import sqrt, cos, sin, atan2

GRAVITATIONAL_CONSTANT = 6.67408e-11;
SCALE = 5.0e-7;
PI = 3.14159

def magnitude(vector: [float, float]):
    return sqrt(vector[0]**2 + vector[1]**2)

def normalize(vector: [float, float]):
    return [vector[0] / magnitude(vector), vector[1] / magnitude(vector)]

class Object:
    def __init__(self, parent, mass: float, position: [float, float], velocity: [float, float], color: str):
        self.parent = parent
        self.mass = mass
        self.position = position
        self.velocity = velocity
        self.color = color

    def step(self, delta_time: float):
        self.position[0] += self.velocity[0] * delta_time
        self.position[1] += self.velocity[1] * delta_time
        displacement = [self.position[0] - self.parent.position[0], self.position[1] - self.parent.position[1]]
        distance = self.distance()
        acceleration = GRAVITATIONAL_CONSTANT * self.parent.mass / distance**2
        displacement_unit = [displacement[0] / distance, displacement[1] / distance]
        self.velocity[0] -= acceleration * displacement_unit[0] * delta_time
        self.velocity[1] -= acceleration * displacement_unit[1] * delta_time

    def distance(self):
        return sqrt(self.position[0]**2 + self.position[1]**2)

    def render(self, canvas: Canvas):
        scaled_position = (self.position[0] * SCALE, self.position[1] * SCALE)
        #canvas.create_oval(400+scaled_position[0]-5, 400-scaled_position[1]-5, 400+scaled_position[0]+5, 400-scaled_position[1]+5, fill=self.color, outline=self.color)


start_position = [-34042272.784328505,228391355.7630093]
start_velocity = [-914.5252638280764,-789.2460296350055]
earth = Object(None, 5.9722e24, [0, 0], [0, 0], "blue")
moon = Object(earth, 0.07346e24, [start_position[0], start_position[1]], [start_velocity[0], start_velocity[1]], "gray")

window = Tk()
canvas = Canvas(window, width=800, height=800, background="black")
canvas.pack()

previous_previous_distance = 0
previous_distance = 0

for i in range(0, 14000):
    moon.step(100.0)
    moon.render(canvas)
    window.update()

    if i > 3:
        if (previous_distance < previous_previous_distance) and (previous_distance < moon.distance()): # periapsis
            periapsis_vector = [moon.position[0], moon.position[1]]
        elif (previous_distance > previous_previous_distance) and (previous_distance > moon.distance()): # apoapsis
            apoapsis_vector = [moon.position[0], moon.position[1]]
    
    print(moon.position, moon.velocity)

    previous_previous_distance = previous_distance
    previous_distance = moon.distance()


apoapsis = magnitude(apoapsis_vector)
periapsis = magnitude(periapsis_vector)
semi_major_axis = (apoapsis + periapsis) / 2
specific_angular_momentum = magnitude(start_position) * magnitude(start_velocity)
argument_of_periapsis = atan2(periapsis_vector[1], periapsis_vector[0])
eccentricity = 1 - periapsis / semi_major_axis

print("periapsis vector", periapsis_vector)
print("periapsis argument", )
print("periapsis distance", periapsis)
print("apoapsis distance", apoapsis)
print("semimajor axis", semi_major_axis)
print("eccentricity", eccentricity)

true_anomaly = atan2(start_position[1], start_position[0])
mean_anomaly = true_anomaly - argument_of_periapsis
radial_speed = (earth.mass * GRAVITATIONAL_CONSTANT / specific_angular_momentum) * eccentricity * sin(mean_anomaly)
normal_speed = specific_angular_momentum / magnitude(start_position)
radial_direction = normalize(start_position)
normal_direction = [-radial_direction[1], radial_direction[0]]
print("velocity", [(radial_speed * radial_direction[0]) + (normal_speed * normal_direction[0]), (radial_speed * radial_direction[1]) + (normal_speed * normal_direction[1])])