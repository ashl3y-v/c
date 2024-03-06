import numpy as np
from math import floor, ceil, log, exp, copysign


def float_to_hex(value: float):
    result = ""
    l = list()
    # converting to positive number and storing the - sign to the list
    if value < 0:
        value = -value
        l.append("-")

    ivalue = int(value)  # represent the integer part
    fvalue = value - ivalue  # represent the floating part

    l.append(f"{ivalue:x}")  # storing the hexadecimal representation of integer part
    l.append(".")

    # float is 8 bytes and so has at most 16 hexadecimal values
    for i in range(16):
        fvalue = fvalue * 16
        digit = int(fvalue)
        l.append(f"{digit:x}")
        fvalue -= digit  # removing the integer part
        if fvalue == 0:
            break
    # converting the result to a string
    for v in l:
        result += str(v)

    return result


def my_format(x: float, b: int):
    e = floor(log(abs(x), b))
    c = float_to_hex(x / float(b)**e)

    return (c, e, b)

@np.vectorize
def cost(x: float, b:int):
    c, *_ = my_format(x, b)

    c = c.replace(".","")

    return sum([int(s, 16) for s in c])

if __name__ == "__main__":
    n = 2**(64*(np.random.random([64]) - 0.5))
    for i in [2, 16, 64, 256]:
        print(f"{i}: {cost(n, i).sum()}")
