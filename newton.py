import time
import math
import cmath


# derivative of function
# (f(x+a) - f(x)) / a
# o is the order of the derivative
def d(f, x, o=1, a=1e-10):
    if o == 1:
        # first derivative is simple
        return (f(x + a) - f(x)) / a
    else:
        # higher order need recursive (derivative of derivative etc.)
        # basically the same thing
        return (d(f, x + a, o=o - 1, a=a) - d(f, x, o=o - 1, a=a)) / a


# newton's method, finds the zeros
def newton(f, z, a=1e-10):
    # the values when it thinks it's a zero, should be about 0
    y = []
    for i in range(len(z)):
        v = f(z[i])
        # while the value is greater than a, lower a means more accuracy
        while abs(v) > a:
            v = f(z[i])
            # changing guess
            z[i] = z[i] - v / d(f, z[i])
        y.append(v)

    return z, y


# this is just for making imaginary numbers not ugly
def format_imag(x):
    return f"{round(x.real, 4)} + {round(x.imag, 4)}i"


# function to find zeros of
f = lambda x: x**2 - 1

# initial points
z = [1 + 1j, -1 - 1j]

# time it
start_time = time.perf_counter()

# do it
z, y = newton(f, z)

end_time = time.perf_counter()

print("solution: ", list(map(format_imag, z)), list(map(format_imag, y)))

print("execution time: ", end_time - start_time)
