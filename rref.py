import numpy as np


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
