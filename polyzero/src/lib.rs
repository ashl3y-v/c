use num_complex::Complex;
use num_traits::{Float, Num};
use polynomial::Polynomial;
use rayon::prelude::*;
use std::f64::consts::TAU;
use std::ops::{Div, Mul, Neg};

// idea: write halley fractal visualization

/// symbolic derivative of the polynomial
pub fn diff<T>(p: &Polynomial<Complex<T>>) -> Polynomial<Complex<T>>
where
    T: Clone + Num + From<f64>,
    Complex<T>: Mul<Complex<T>, Output = Complex<T>>,
{
    Polynomial::new(
        p.data()
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| Complex::new(T::from(i as f64), 0_f64.into()) * c.to_owned())
            .collect::<Vec<_>>(),
    )
}

/// run halley's method on polynomial `p` with initial guesses `z` for <= `m` iterations until the change in `z` is less than `eps`
pub fn halley<T>(
    p: &Polynomial<Complex<T>>,
    z: Vec<Complex<T>>,
    max_iter: usize,
    eps: T,
) -> Vec<Complex<T>>
where
    T: Clone + Send + Sync + Num + Float + From<f64>,
    Vec<Complex<T>>: Sized + rayon::iter::IntoParallelIterator<Item = Complex<T>>,
{
    // the derivatives of the function
    let dp = diff(p);
    let ddp = diff(&dp);

    // run it in parallel over z
    z.into_par_iter()
        .map(|mut z| {
            let mut i = 0;

            loop {
                // the function and its derivative at the point
                let p_z = p.eval(z);

                if p_z.norm() < eps {
                    break Some(z);
                }

                let dp_z = dp.eval(z);
                let ddp_z = ddp.eval(z);

                // update z
                // x_n+1 = x_n - 2 f(x_n) f'(x_n) / (2 [f'(x_n)]^2 - f(x_n) f''(x_n))
                z = z
                    - ((Complex::<T>::new(2_f64.into(), 0_f64.into()) * p_z * dp_z)
                        / (Complex::<T>::new(2_f64.into(), 0_f64.into()) * dp_z.powu(2)
                            - p_z * ddp_z));

                // check if we have gone on for >= m iterations and then break None, which will be filtered out
                if i >= max_iter {
                    break None;
                }

                i += 1;
            }
        })
        .filter_map(|z| z)
        .collect()
}

/// convert a vec of points into samples of `n` points around each in a circle with magnitude `eps`
/// real case is case of n = 2 always
pub fn samp<T>(z: &[Complex<T>], n: usize, eps: &T) -> Vec<Complex<T>>
where
    T: Clone + Num + From<f64>,
{
    // e^i(n/n τ) = e^i(0/n τ) so k goes from 0 to n - 1
    z.iter()
        .flat_map(|z| {
            (0..n).map(|i| {
                let angle = TAU * i as f64 / n as f64;
                z.to_owned()
                    + (Complex::from(eps.to_owned())
                        * Complex::new(angle.cos().into(), angle.sin().into()))
            })
        })
        .collect()
}

// idea: use certain sampling method for the convex hull of the derivative and the interior
// attempt to find all the complex roots of the polynomial with radii `n` * its degree, maximum iterations of newton's method `m`, coefficient of rounding for eliminating identical solutions `r_coef`, and `eps`, passed to sampling and newton's method, and used for rounding
// a degree k polynomial will have k roots (although they may be the same) and is not trivially solved
// differentiating this a times will yield a polynomial with k - a roots
// if k - a = 1 or 2, this can be trivially solved for its zeros
// the zeros of the derivative of a polynomial lie inside the convex hull of its zeros by the Gauss-Lucas theorem
// therefore, we begin newton's method at the points of the derivative's zeros radiating out
// to attempt to land initial points in the basins of convergence of every root
pub fn roots<T>(
    p: &Polynomial<Complex<T>>,
    radii_coef: usize,
    max_iter: usize,
    round_coef: T,
    eps: T,
) -> Vec<Complex<T>>
where
    T: Copy + Num + Float + Send + Sync + From<f64>,
    Complex<T>: Neg<Output = Complex<T>>
        + Mul<Complex<T>, Output = Complex<T>>
        + Div<Complex<T>, Output = Complex<T>>,
{
    match p.data() {
        [] | [_] => vec![],
        &[b, a] => vec![-b / a],
        &[c, b, a] => {
            let m = -b / (Complex::new(2_f64.into(), 0_f64.into()) * a);
            let d = (m.powu(2) - c / a).sqrt();

            vec![m - d, m + d]
        }
        _ => halley(
            p,
            samp(
                &roots(&diff(p), radii_coef, max_iter, round_coef, eps),
                radii_coef * (p.data().len() - 1),
                &eps,
            ),
            max_iter,
            eps,
        ),
    }
    .into_iter()
    .fold(Vec::new(), |mut acc, z| {
        if !acc
            .iter()
            .any(|&acc_z| (acc_z - z).norm() < round_coef * eps)
        {
            acc.push(z);
        }

        acc
    })
}
