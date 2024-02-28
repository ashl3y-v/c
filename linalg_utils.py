import numpy as np
import scipy as sp
from numpy.linalg import svd, pinv, matrix_rank, norm
from scipy.linalg import lu, orth, null_space


def rref(a: np.ndarray):
    u = lu(a)[-1]
    rank = matrix_rank(u)
    scale = np.diag(u).copy()
    scale[rank:] = 1
    u /= np.expand_dims(scale, axis=-1)

    for i in range(1, u.shape[-2]):
        ridx_above = np.arange(i)
        u[ridx_above, i:] = u[ridx_above, i:] - np.outer(u[i, i:], u[ridx_above, i]).T

    u[:rank, :rank] = np.eye(rank)
    u[np.tril_indices_from(u, k=-1)] = 0
    u = np.pad(u, ((0, a.shape[-2] - u.shape[-2]), (0, 0)))
    return u


def matrix_support(x: np.ndarray):
    return svd(x).Vh.T[..., : matrix_rank(x)]


def matrix_cokernel(x: np.ndarray):
    return svd(x).U[..., matrix_rank(x) :]


def is_subspace(a: np.ndarray, b: np.ndarray, axis=None):
    x = pinv(b) @ a
    return (
        np.allclose(b @ x, a)
        if axis is None
        else np.isclose(norm(b @ x - a, axis=axis), 0)
    )
