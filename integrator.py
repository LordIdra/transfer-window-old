from tkinter import *
from math import sqrt, cos, sin

GRAVITATIONAL_CONSTANT = 6.67408e-11;
SCALE = 5.0e-7;
PI = 3.14159

def magnitude(vector: [float, float]):
    return sqrt(vector[0]**2 + vector[1]**2)

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
        canvas.create_oval(400+scaled_position[0]-5, 400+scaled_position[1]-5, 400+scaled_position[0]+5, 400+scaled_position[1]+5, fill=self.color, outline=self.color)


earth = Object(None, 5.9722e24, [0, 0], [0, 0], "blue")
moon = Object(earth, 0.07346e24, [0.4055e9 * cos(PI/6), 0.4055e9 * sin(PI/6)], [0.970e3 * cos(PI/6 + PI/2), 0.970e3 * sin(PI/6 + PI/2)], "gray")

window = Tk()
canvas = Canvas(window, width=800, height=800, background="black")
canvas.pack()

previous_previous_distance = 0
previous_distance = 0

for i in range(0, 5000):
    moon.step(500.0)
    moon.render(canvas)
    window.update()

    if i > 3:
        if (previous_distance < previous_previous_distance) and (moon.distance() > previous_distance): # periapsis
            periapsis_vector = moon.position
        elif (previous_distance > previous_previous_distance) and (moon.distance() < previous_distance): # apoapsis
            apoapsis_vector = moon.position

    previous_previous_distance = previous_distance
    previous_distance = moon.distance()

apoapsis = magnitude(apoapsis_vector)
periapsis = magnitude(periapsis_vector)
semi_major_axis = (apoapsis + periapsis) / 2

print(semi_major_axis)