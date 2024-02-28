from multiprocessing import Pool
import numpy as np
import scipy as sp
import timeit

np.set_printoptions(linewidth=2**8, suppress=True)


def rref(a: np.ndarray):
    assert a.ndim > 1

    if a.ndim == 2:
        return _rref_mat(a)
    else:
        orig_dim = a.shape
        a = a.reshape([-1, a.shape[-2], a.shape[-1]])
        return np.array([[_rref_mat(x) for x in a]]).reshape(orig_dim)


def rref_par(a: np.ndarray):
    assert a.ndim > 1

    if a.ndim == 2:
        return _rref_mat(a)
    else:
        orig_dim = a.shape
        a = a.reshape([-1, a.shape[-2], a.shape[-1]])
        with Pool() as p:
            return np.array(p.map(_rref_mat, a)).reshape(orig_dim)


def _rref_mat(a: np.ndarray):
    assert a.ndim > 1

    rank = np.linalg.matrix_rank(a)
    if rank == a.shape[-1]:
        a = a.copy()
        a[...] = 0
        a[np.diag_indices(rank)] = 1
        return a

    u = sp.linalg.lu(a)[-1]
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


def rankify_svdvals(a: np.ndarray):
    u, s, vh = np.linalg.svd(a)
    s[..., np.linalg.matrix_rank(a) :] = 0
    return u * s @ vh


def is_subspace(a: np.ndarray, b: np.ndarray, axis=None):
    x = np.linalg.pinv(b) @ a
    return (
        np.allclose(b @ x, a)
        if axis is None
        else np.isclose(np.linalg.norm(b @ x - a, axis=axis), 0)
    )


a = np.random.random([10, 10, 300, 300])

d0 = timeit.timeit(lambda: rref(a), number=1)
d1 = timeit.timeit(lambda: rref_ref0(a), number=1)
