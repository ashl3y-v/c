import numpy as np


def rref(a: np.ndarray) -> np.ndarray:
    rows = a.shape[-2]

    a = np.array(np.linalg.qr(a, mode="r"))

    rank = np.count_nonzero(~np.isclose(np.diag(a), 0))

    if rank == a.shape[-1]:
        return np.eye(rows, a.shape[-1])

    a = np.linalg.inv(a[:rank, :rank]) @ a[:rank, :]

    a = np.vstack([a, np.zeros([rows - a.shape[-2], a.shape[-1]])])

    return a
