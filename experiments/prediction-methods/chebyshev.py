from math import sin, cos, pi
import matplotlib.pyplot as plt

def T(n: float, x: float) -> float:
    if n == 0:
        return 1
    elif n == 1:
        return x
    else:
        return 2*x*T(n-1, x) - T(n-2, x)

# N = number of coefficients
# j = nth coefficient
def coefficient(f, N: float, j: float) -> float:
    idx = 0
    for k in range(0, N):
        x_k = cos(pi * (k + 0.5) / (N + 1))
        idx += f(x_k) * T(j, x_k)
    return 2 * idx / (N + 1)

def approximate(f, N: float, x: float) -> float:
    idx = -0.5*coefficient(f, N, 0)
    for k in range(0, N):
        idx += coefficient(f, N, k) * T(k, x)
    return idx

def test():
    def f(x: float) -> float:
        return 6*sin(5*x) - 4*cos(2*x) + 10*x

    x_axis = []
    actual = []
    expected = []
    for i in range(-100, 100):
        x = i / 100
        x_axis.append(x)
        actual.append(approximate(f, 7, x))
        expected.append(f(x))

    plt.plot(x_axis, actual)
    plt.plot(x_axis, expected)
    plt.show()

#test()