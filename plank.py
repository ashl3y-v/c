import quantities as pq

l_P = pq.constants.l_P
m_P = pq.constants.m_P
t_P = pq.constants.t_P
T_P = pq.constants.T_P
q_P = pq.UnitConstant(
    "Planck_charge", (pq.epsilon_0 * pq.constants.hbar * pq.c) ** (1 / 2), "q_P"
)
I_P = pq.UnitConstant("Planck_current", q_P / t_P, "I_P")
e_P = l_P**2 * m_P / t_P**2

pq.set_default_units(length=l_P, mass=m_P, time=t_P, temperature=T_P, current=I_P)
