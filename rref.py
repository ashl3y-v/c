import numpy as np


def rref(a: np.ndarray) -> np.ndarray:
    m, n = a.shape

    # QR decomposition
    a = np.linalg.qr(a, mode="r")

    # calculate rank from number of zero diag elements
    r = np.count_nonzero(~np.isclose(np.diag(a), 0))

    # return in special cases
    if r == n:
        return np.eye(m, n)
    elif r == 0:
        return np.zeros_like(a)

    # normalize to get ones
    first_in_rows = (a!=0).argmax(axis=-1)

    scale = a[range(a.shape[-2]), first_in_rows]
    scale[r:] = 1
    a /= np.expand_dims(scale, axis=-1)

    # calculate inverse of linearly independent subset
    indep_subset = a[:, first_in_rows]
    print(np.round(a, 3))
    print(np.round(indep_subset, 3))

    # apply inverse to dependent part

    # replace independent part with identity
    
    # pad and return
    
    a[:r, r:] = np.linalg.inv(a[:r, :r]) @ a[:r, r:]
    a[:r, :r] = np.eye(r)

    scale = np.diag(a[:, (a != 0).argmax(axis=-1)]).copy()
    scale[scale == 0] = 1

    a /= np.expand_dims(scale, axis=-1)

    if r < m:
        a = np.vstack([a, np.zeros([m - a.shape[-2], n])])

    a = np.triu(a)

    return a

if __name__ == "__main__":
    a = np.random.random([5,5])
    a[1] = a[0]

    r = rref(a)
