import numpy as np
import timeit

np.set_printoptions(suppress=True, edgeitems=8, linewidth=256)


def rref(a: np.ndarray) -> np.ndarray:
    m, n = a.shape

    a = np.array(np.linalg.qr(a, mode="r"))

    r = np.count_nonzero(~np.isclose(np.diag(a), 0))

    if r == n:
        return np.eye(m, n)

    a[:r, r:] = np.linalg.inv(a[:r, :r]) @ a[:r, r:]
    a[:r, :r] = np.eye(r)

    if r < m:
        a = np.vstack([a, np.zeros([m - a.shape[-2], n])])

    return a


# if __name__ == "__main__":
#     for i in range(2, 10):
#         for j in range(2, 10):
#             for n in range(50):
#                 a = np.random.random([5, 5])
#                 if n % 2 == 0:
#                     a[-1] = a[0]
#                 if n % 3 == 0:
#                     a[:, -1] = a[:, 0]
#                 if n % 4 == 0:
#                     u, s, vh = np.linalg.svd(a)
#                     s[1:] = 0
#                     a = u * s @ vh

#                 assert np.allclose(rref(a), rref_ref(a))

#     a = np.random.random([2**11, 2**11])
#     a[-3:] = a[0]

#     d0 = timeit.timeit(lambda: rref(a), number=8)
#     d1 = timeit.timeit(lambda: rref_ref(a), number=8)
