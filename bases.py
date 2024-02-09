import numpy as np
from numba import njit


@np.vectorize
@njit
def repeatand_digits(n: int, base: int):
    remainder_set = set()
    remainder = 1 % n
    repeatand_length = 0

    while remainder != 0 and remainder not in remainder_set:
        remainder_set.add(remainder)
        remainder = (remainder * base) % n
        repeatand_length += 1

    return repeatand_length


bases = np.array(
    [
        2,
        3,
        4,
        6,
        8,
        9,
        10,
        12,
        16,
        21,
        25,
        27,
        29,
        30,
        32,
        34,
        36,
        45,
        46,
        49,
        55,
        56,
        57,
        60,
        64,
    ]
)

# n = np.expand_dims(np.array(range(2, 2**n + 1)), axis=-1)
# t = repeatand_digits(n, bases).T
# t.mean(axis=1)
# bases * t.mean(axis=1)
# bases ** t.mean(axis=1)
# bases[t.mean(axis=1).argsort()]
# np.sort(t.mean(axis=1)) / t.mean(axis=1).min()
