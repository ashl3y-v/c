from sympy import (
    Integer,
    Matrix,
    Rational,
    Sum,
    Symbol,
    cos,
    erf,
    exp,
    pi,
    pprint,
    sin,
    solve,
    sqrt,
    symbols,
    tan,
)
from sympy.stats import (
    Die,
    E,
    Normal,
    P,
    variance,
)

a = Symbol("a")
b = Symbol("b")
c = Symbol("c")
r = Symbol("r")
t = Symbol("t")
u = Symbol("u")
v = Symbol("v")
x = Symbol("x")
y = Symbol("y")
z = Symbol("z")
θ = Symbol("θ")
μ = Symbol("μ")
ρ = Symbol("ρ")
σ = Symbol("σ")
φ = Symbol("φ")

normpdf = (
    lambda x, μ, σ: 1 / (σ * sqrt(2 * pi)) * exp(-Rational(1, 2) * ((x - μ) / σ) ** 2)
)
normcdf = lambda x, μ, σ: Rational(1, 2) * (Integer(1) + erf((x - μ) / (σ * sqrt(2))))
invnormcdf = lambda x, μ, σ: solve(normcdf(a, μ, σ) - x, a)
