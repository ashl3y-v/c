import numpy as np
from numba import njit


@njit
def find_repeatand_len(n: int, base: int):
    remainder_set = set()
    remainder = 1 % n
    repeatand_length = 0

    while remainder != 0 and remainder not in remainder_set:
        remainder_set.add(remainder)
        remainder = (remainder * base) % n
        repeatand_length += 1

    return repeatand_length


find_repeatand_len_vec = np.vectorize(find_repeatand_len, otypes=[int])


def ts_first_n(ns: np.ndarray, bs: np.ndarray):
    return np.array([find_repeatand_len_vec(np.array(ns), b) for b in bs])
