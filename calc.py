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


def evaluate(f, s):
    return [f.subs(x) for x in s]


def grad(f, v):
    return Matrix([f.diff(x) for x in v])


def lg_mult(f, g, v):
    l = symbols(f"λ:{len(g)}")

    t = [l[i] * grad(g[i], v) for i in range(len(g))]

    t_s = t[0]
    for i in range(len(g) - 1):
        t_s += t[i + 1]

    sols = solve([grad(f, v) + t_s] + g)

    return sols, evaluate(f, sols)


f = parse_expr("x**2 + y")
g = [parse_expr("x**2 + y**2 - 1")]
v = symbols("x y")

sols, vals = lg_mult(f, g, v)

pprint(sols)
pprint(vals)
