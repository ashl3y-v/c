use nalgebra::{Complex, ComplexField, DMatrix, Dyn, RealField};

/// find the complex zeros with real coefficients
pub fn eigenzeros<T>(p: Vec<T>) -> Option<Vec<T>>
where
    T: Copy + ComplexField,
{
    let n = p.len() - 1;

    let leading_coef = *p.last().expect("Couldn't get leading coefficient");

    let p = p
        .into_iter()
        .take(n)
        .map(|c| c / leading_coef)
        .collect::<Vec<_>>();

    let c = DMatrix::from_fn_generic(Dyn(p.len()), Dyn(p.len()), |i, j| {
        // hi
        if j == p.len() - 1 {
            -p[i]
        } else if i < 1 {
            T::zero()
        } else if i - 1 == j {
            T::one()
        } else {
            T::zero()
        }
    });

    c.eigenvalues().map(|v| v.as_slice().to_vec())
}
