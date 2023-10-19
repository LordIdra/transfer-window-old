from tkinter import *
from math import sqrt

GRAVITATIONAL_CONSTANT = 6.67408e-11;
SCALE = 1.0e-8;

class Object:
    def __init__(self, parent, mass: float, position: (float, float), velocity: (float, float)):
        self.parent = parent
        self.mass = mass
        self.position = position
        self.velocity = velocity

    def step(self, delta_time: float):
        self.position += self.velocity * delta_time
        displacement = (self.position - self.parent.position)[0]
        distance = sqrt(displacement[0]**2 + displacement[1]**2)
        self.velocity += GRAVITATIONAL_CONSTANT * self.parent.mass / distance**2

    def render(self, canvas: Canvas):
        scaled_position = [position[0] * SCALE, position[1] * SCALE]
        canvas.create_oval()


earth = Object(None, 5.9722e24 [0, 0], [0, 0])
moon = Object(earth, 0.07346e24, [0.4055e9, 0], [0, 0.970e3])

