import math

a = -3
b = -2

px = 8
py = 0

def f(x):
    return math.sqrt(x**2 * b**2 / a**2 - b**2)

def ds(x, y):
    return (x - px)**2 + (y - py)**2

def ds_prime(x, y):
    return (y - py) * (2.0 * b**2 * x) / (a**2 * y) + 2.0 * (x - px)

def ds_prime_prime(x, y):
    return (2 * b**2 / a**2) - (2 * b**2 * py / a**2) / ds(x, y) + (2 * b**2 * py / a**2) * ds_prime(x, y) / ds(x, y)**2 + 2

print(ds_prime_prime(5.538, f(5.538)))

x = 100000000
for i in range(0, 3):
    print(x)
    y = f(x)
    #print(ds_prime(x, y))
    delta = -ds_prime(x, y) / ds_prime_prime(x, y)
    x += delta