import numpy as np
import scipy as sp

def rref(a: np.ndarray, eps: float = 1e-8) -> np.ndarray:
    m, n = a.shape

    # QR decomposition
    a = np.linalg.qr(a, mode="r")
    r = np.count_nonzero((np.abs(np.diag(a)) > eps))

    print(a)

    # calculate inverse of linearly independent subset
    indep_idx = (np.abs(a) > eps).argmax(axis=-1)[(np.abs(a) > eps).any(axis=-1)]
    print(indep_idx)
    indep_idx = indep_idx[~np.insert(np.diff(indep_idx) == 0, 0, False)]
    dep_idx = np.delete(np.arange(a.shape[-1]), indep_idx)
    print(indep_idx, dep_idx, r)

    a[:r, dep_idx] = np.linalg.inv(a[:r, indep_idx]) @ a[:r, dep_idx]
    a[:, indep_idx] = np.eye(a.shape[-2], r)

    # scale
    nonzero_diag = (np.abs(a) > eps).argmax(axis=-1)

    scale = a[range(a.shape[-2]), nonzero_diag].copy()
    scale[np.abs(scale) < eps] = 1
    a /= np.expand_dims(scale, axis=-1)

    # pad and return

    # scale = np.diag(a[:, (a != 0).argmax(axis=-1)]).copy()
    # scale[scale == 0] = 1

    # a /= np.expand_dims(scale, axis=-1)

    if r < m:
        a = np.vstack([a, np.zeros([m - a.shape[-2], n])])

    return a


def rref_ref(B, tol=1e-8, debug=False):
    a = B.copy()
    rows, cols = a.shape
    r = 0
    pivots_pos = []
    row_exchanges = np.arange(rows)
    for c in range(cols):
        ## Find the pivot row:
        pivot = np.argmax(np.abs(a[r:rows, c])) + r
        m = np.abs(a[pivot, c])
        if m <= tol:
            ## Skip column c, making sure the approximately zero terms are
            ## actually zero.
            a[r:rows, c] = np.zeros(rows - r)
        else:
            ## keep track of bound variables
            pivots_pos.append((r, c))

            if pivot != r:
                ## Swap current row and pivot row
                a[[pivot, r], c:cols] = a[[r, pivot], c:cols]
                row_exchanges[[pivot, r]] = row_exchanges[[r, pivot]]

            ## Normalize pivot row
            a[r, c:cols] = a[r, c:cols] / a[r, c]

            ## Eliminate the current column
            v = a[r, c:cols]
            ## Above (before row r):
            if r > 0:
                ridx_above = np.arange(r)
                a[ridx_above, c:cols] = (
                    a[ridx_above, c:cols] - np.outer(v, a[ridx_above, c]).T
                )
            ## Below (after row r):
            if r < rows - 1:
                ridx_below = np.arange(r + 1, rows)
                a[ridx_below, c:cols] = (
                    a[ridx_below, c:cols] - np.outer(v, a[ridx_below, c]).T
                )
            r += 1
        ## Check if done
        if r == rows:
            break
    return a


if __name__ == "__main__":
    np.set_printoptions(suppress=True, linewidth=256)

    a = np.random.random([5, 5])
    a[-1] = 0
    a[:, 2] = a[:, 0] + a[:, 1]
    a[:, -1] = a[:, -2]

    if not np.allclose(rref(a), rref_ref(a)):
        print(rref(a))
        print(rref_ref(a))
        print("bad")
