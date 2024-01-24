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
    cos,
    sin,
    tan,
    pi,
)

init_printing()

a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z = symbols("a:z")
a0, a1, a2, a3, a4, a5, a6, a7, a8, a9 = symbols("a:10")

# θ, φ, μ, σ, υ, ν, ρ = symbols("θ φ μ σ υ ν ρ")

cylindrical = [[x, r * cos(θ)], [y, r * sin(θ)], [z, z]]
spherical = [[x, ρ * cos(θ) * sin(φ)], [y, ρ * sin(θ) * sin(φ)], [z, ρ * cos(φ)]]

vol_c = r
vol_s = ρ**2 * sin(φ)


def evaluate(f, s):
    return [f.subs(x) for x in s]


def grad(f, v):
    return Matrix([f.diff(x) for x in v])


def divergence(vec_F, *vars):
    return sum(Matrix([vec_F[i].diff(v) for i, v in enumerate(vars)]))


def curl_part(vec_F, u, v, *vars):
    return vec_F[v].diff(vars[u]) - vec_F[u].diff(vars[v])


def curl_op(vec_F, *vars):
    """Return the curl of a vector field."""
    return Matrix([curl_part(vec_F, u, v, *vars) for u, v in [(1, 2), (2, 0), (0, 1)]])


def lg_mult(f, g, v):
    l = symbols(f"λ:{len(g)}")

    t = [l[i] * grad(g[i], v) for i in range(len(g))]

    t_s = t[0]
    for i in range(len(g) - 1):
        t_s += t[i + 1]

    sols = solve([grad(f, v) + t_s] + g)

    return sols, evaluate(f, sols)
