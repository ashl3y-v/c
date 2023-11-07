from sympy import (
    init_printing,
    parse_expr,
    symbols,
    Symbol,
    pprint,
    solve,
    Matrix,
    Sum,
    sqrt,
)

init_printing()

θ = Symbol('θ')
φ = Symbol('φ')
u = Symbol('u')
v = Symbol('v')

def evaluate(f, s):
    return [f.subs(x) for x in s]


def grad(f, v):
    return Matrix([f.diff(x) for x in v])

def nabla(f, x, y, z):
    return Matrix([f.diff(x), f.diff(y), f.diff(z)])

def nabla_v(f, x, y, z,  v):
    return nabla(f, x, y, z).dot(v)

def gradient_op(f, *variables):
    """return the vector gradient of a scalar function."""
    return Matrix([f.diff(v) for v in variables ])

def divergence_op(vec_F, *vars):
    """Return the scalar divergence of a vector field."""
    return sum( Matrix( [ vec_F[i].diff(v) for i,v in enumerate(vars)]) )

def curl_part(vec_F, u, v, *vars):
    return vec_F[v].diff(vars[u]) - vec_F[u].diff(vars[v])
    
def curl_op(vec_F, *vars):
    """Return the curl of a vector field."""
    return Matrix([curl_part(vec_F, u, v, *vars) for u, v in [(1,2), (2,0), (0,1)]])

def lg_mult(f, g, v):
    l = symbols(f"λ:{len(g)}")

    t = [l[i] * grad(g[i], v) for i in range(len(g))]

    t_s = t[0]
    for i in range(len(g) - 1):
        t_s += t[i + 1]

    sols = solve([grad(f, v) + t_s] + g)

    return sols, evaluate(f, sols)
