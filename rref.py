import numpy as np


def rref(a: np.ndarray) -> np.ndarray:
    m, n = a.shape

    r = np.linalg.matrix_rank(a)

    if r == m:
        return np.eye(m, n)

    a = np.linalg.qr(a, mode="r")

    b = np.zeros_like(a)
    b[0, 0] = 1

    for i in range(n):
        x = np.linalg.pinv(b[: i + 1, : i + 1]) @ b[: i + 1, i : i + 1]
        print(x.shape)
        x = np.pad(x, ((0, m - (i + 1)), (0, 0)))
        print(b[:, : i + 1].T.shape, x.shape, b[: i + 1, i : i + 1].shape)
        err = np.linalg.norm(b[:, : i + 1].T @ x - b[: i + 1, i : i + 1])
        if err != 0:
            x = np.zeros([m, 1])
            x[i + 1] = 1
            print(x)

        b[:, i : i + 1] = x

    # if r < m:
    #     a = np.vstack([a, np.zeros([m - a.shape[-2], n])])

    return b


def rref_broken(a: np.ndarray) -> np.ndarray:
    m, n = a.shape

    a = np.linalg.qr(a, mode="r")

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

if __name__ == "__main__":
    a = np.random.random([5,5])
    a[-1] = 0
    b = rref(a)
    print(b)
