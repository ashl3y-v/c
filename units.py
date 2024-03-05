import numpy as np
import quantities as pq
import scipy as sp

G = pq.UnitConstant(
    "Newtonian_constant_of_gravitation",
    sp.constants.G * pq.constants.G.simplified.units,
    "G",
)
N_A = pq.UnitConstant(
    "Avogadro_constant", sp.constants.N_A * pq.constants.N_A.simplified.units, "N_A"
)
alpha = pq.UnitConstant(
    "fine_structure_constant",
    sp.constants.alpha * pq.constants.alpha.simplified.units,
    "α",
)
e = pq.UnitConstant(
    "elementary_charge", sp.constants.e * pq.constants.e.simplified.units, "e"
)
epsilon_0 = pq.UnitConstant(
    "electric_constant",
    sp.constants.epsilon_0 * pq.constants.epsilon_0.simplified.units,
    "ε_0",
)
h = pq.UnitConstant(
    "Planck_constant", sp.constants.h * pq.constants.h.simplified.units, "h"
)
hbar = pq.UnitConstant(
    "Planck_constant_over_2_pi",
    sp.constants.hbar * pq.constants.hbar.simplified.units,
    "ħ",
)
k = pq.UnitConstant(
    "Boltzmann_constant", sp.constants.k * pq.constants.k.simplified.units, "k"
)
m_e = pq.UnitConstant(
    "electron_mass",
    sp.constants.electron_mass * pq.constants.m_e.simplified.units,
    "m_e",
)
m_n = pq.UnitConstant(
    "neutron_mass", sp.constants.neutron_mass * pq.constants.m_n.simplified.units, "m_n"
)
m_p = pq.UnitConstant(
    "proton_mass", sp.constants.proton_mass * pq.constants.m_p.simplified.units, "m_p"
)
mu_0 = pq.UnitConstant(
    "magnetic_constant", sp.constants.mu_0 * pq.constants.mu_0.simplified.units, "mu_0"
)
u = pq.UnitConstant(
    "unified_atomic_mass_unit", sp.constants.u * pq.constants.u.simplified.units, "u"
)
R = pq.UnitConstant("molar_gas_constant", (k * N_A).simplified, "R")
