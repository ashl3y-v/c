import numpy as np


# broken
def rref(a: np.ndarray) -> np.ndarray:
    m, n = a.shape

    a = np.array(np.linalg.qr(a, mode="r"))

    r = np.count_nonzero(~np.isclose(np.diag(a), 0))

    if r == n:
        return np.eye(m, n)

    a[:r, r:] = np.linalg.inv(a[:r, :r]) @ a[:r, r:]
    a[:r, :r] = np.eye(r)

    scale = np.diag(a[:, (a != 0).argmax(axis=-1)]).copy()
    scale[scale == 0] = 1

    a /= np.expand_dims(scale, axis=-1)

    if r < m:
        a = np.vstack([a, np.zeros([m - a.shape[-2], n])])

    return a
