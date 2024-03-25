import numpy as np
import scipy as sp
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


if __name__ == "__main__":
    bases = np.array([10])
    ns = np.expand_dims(np.array([15]), axis=-1)
    # ns = np.expand_dims(np.array(range(2, 2**13 + 1)), axis=-1)
    t = repeatand_digits(ns, bases).T
    print(t)

    # for i in range(t.shape[0]):
    #     print(2 + i)
    #     print(sp.stats.linregress(np.log(ns.squeeze()), np.log(t[i])))

    # for n in 2 ** np.arange(2, 6):
    #     ns = np.expand_dims(np.array(range(2, n + 1)), axis=-1)
    #     t = repeatand_digits(ns, bases).T
    #     dig = t.mean(axis=1)
    #     siz = bases**dig
    #     print(f"n: {n}")
    #     print("dig")
    #     print(dig[dig.argsort()] / dig.min())
    #     print("siz")
    #     print(siz[siz.argsort()] / siz.min())
    #     print("b by dig")
    #     print(bases[dig.argsort()])
    #     print("b by siz")
    #     print(bases[siz.argsort()])
    #     print("")
