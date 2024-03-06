import numpy as np
import quantities as pq
import scipy as sp

e = pq.UnitConstant(
    "elementary_charge", sp.constants.e * pq.constants.e.simplified.units, "e"
)
g = pq.UnitConstant(
    "Newtonian_constant_of_gravitation",
    sp.constants.G * pq.constants.G.simplified.units,
    "G",
)
h = pq.UnitConstant(
    "Planck_constant", sp.constants.h * pq.constants.h.simplified.units, "h"
)
k = pq.UnitConstant(
    "Boltzmann_constant", sp.constants.k * pq.constants.k.simplified.units, "k"
)
m_e = pq.UnitMass(
    "electron_mass",
    sp.constants.electron_mass * pq.constants.m_e.simplified.units,
    "m_e",
)
m_n = pq.UnitMass(
    "neutron_mass", sp.constants.neutron_mass * pq.constants.m_n.simplified.units, "m_n"
)
m_p = pq.UnitMass(
    "proton_mass", sp.constants.proton_mass * pq.constants.m_p.simplified.units, "m_p"
)
mu_0 = pq.UnitConstant(
    "magnetic_constant", sp.constants.mu_0 * pq.constants.mu_0.simplified.units, "mu_0"
)
n_a = pq.UnitConstant(
    "Avogadro_constant", sp.constants.N_A * pq.constants.N_A.simplified.units, "N_A"
)
u = pq.UnitMass(
    "atomic_mass_unit", sp.constants.u * pq.constants.u.simplified.units, "u"
)
ħ = pq.UnitConstant(
    "Planck_constant_over_2_pi",
    sp.constants.hbar * pq.constants.hbar.simplified.units,
    "ħ",
)
α = pq.UnitConstant(
    "fine_structure_constant",
    sp.constants.alpha * pq.constants.alpha.simplified.units,
    "α",
)
ε_0 = pq.UnitConstant(
    "electric_constant",
    sp.constants.epsilon_0 * pq.constants.epsilon_0.simplified.units,
    "ε_0",
)

l_P = pq.UnitLength("Planck_length", (ħ**0.5 * g**0.5 / pq.c**1.5).simplified, "l_P")
m_P = pq.UnitMass("Planck_mass", (ħ**0.5 * pq.c**0.5 / g**0.5).simplified, "m_P")
t_P = pq.UnitTime("Planck_time", (ħ**0.5 * g**0.5 / pq.c**2.5).simplified, "t_P")
T_P = pq.UnitTemperature(
    "Planck_temperature", (ħ**0.5 * pq.c**2.5 / g**0.5 / k).simplified, "T_P"
)
q_P = pq.UnitConstant(
    "Planck_charge", (ε_0**0.5 * ħ**0.5 * pq.c**0.5).simplified, "q_P"
)
E_P = pq.UnitConstant("Planck_energy", (ħ**0.5 * pq.c**2.5 / g**0.5).simplified, "E_P")
F_P = pq.UnitConstant("Planck_force", (pq.c**4 / g).simplified, "F_P")
I_P = pq.UnitCurrent("Planck_current", (ε_0**0.5 * pq.c**3 / g**0.5).simplified, "I_P")
