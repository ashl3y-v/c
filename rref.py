import numpy as np


def rref(a: np.ndarray) -> np.ndarray:
    rows = a.shape[-2]

    a = np.array(np.linalg.qr(a, mode="r"))
    npad = np.array([(0, 0)] * a.ndim)
    npad[-2] = (0, rows - a.shape[-2])

    a = np.pad(a, pad_width=npad)

    diag = np.diagonal(a, axis1=-2, axis2=-1).copy()
    diag[np.isclose(diag, 0)] = 1

    a[..., : diag.shape[-1], :] /= diag[..., :, np.newaxis]

    for i in range(1, min(a.shape[-2:])):
        a[..., :i, i:] -= (
            a[..., i, i:, np.newaxis] * a[..., np.newaxis, :i, i]
        ).swapaxes(-1, -2)

    return a
