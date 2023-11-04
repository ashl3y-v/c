use itertools::Itertools;
use num_traits::{Float, Num};
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Neg};

pub use num_complex::Complex;

/// run newton's method on polynomial `p` with initial guesses `z` for <= `m` iterations until the change in `z` is less than `eps`
pub fn newt<T>(p: &Pol<T>, z: Vec<Complex<T>>, m: usize, eps: T) -> Vec<Complex<T>>
where
    T: Num + Float + Clone + Send + Sync + From<u32>,
    Complex<T>: Mul<T, Output = Complex<T>>,
    Vec<Complex<T>>: Sized + rayon::iter::IntoParallelIterator<Item = Complex<T>>,
{
    // the derivative of the function
    let dp = p.diff();

    // run it in parallel over z
    z.into_par_iter()
        .map(|mut z| {
            let mut i = 0;

            loop {
                // the function and its derivative at the point
                let p_z = p.eval(z);
                let dp_z = dp.eval(z);

                // how much z should change
                let dz = p_z / dp_z;

                if dz.norm() < eps {
                    break Some(z);
                }

                // update z
                z = z - dz;

                // check if we have gone on for >= m iterations and then break None, which will be filtered out
                if i >= m {
                    break None;
                }

                i += 1;
            }
        })
        .filter_map(|z| z)
        .collect()
}

/// convert a vec of points into samples of `n` points around each in a circle with magnitude `eps`
pub fn samp<T>(z: &Vec<Complex<T>>, n: usize, eps: T) -> Vec<Complex<T>>
where
    T: Clone + Num + Float + From<f64>,
    Complex<T>:
        Add<Complex<T>, Output = Complex<T>> + Mul<Complex<T>, Output = Complex<T>> + From<T>,
{
    // 1/n worth of rotation, e^i(τ/n), e^i(τ/n)^k = e^i(k/n τ)
    let r: Complex<T> = Complex::cis((std::f64::consts::TAU / n as f64).into());

    // e^i(n/n τ) = e^i(0/n τ) so k goes from 0 to n - 1
    z.iter()
        .flat_map(|z| (0..n).map(move |i| *z + Complex::from(eps) * r.powu(i as u32)))
        .collect()
}

/// polynomial, coefficients are listed from lowest to highest power
#[derive(Debug)]
pub struct Pol<T> {
    coef: Vec<Complex<T>>,
}

impl<T> Pol<T> {
    /// construct polynomial from vector of coefficients
    pub fn new(coef: Vec<Complex<T>>) -> Self {
        Pol { coef }
    }

    /// evaluate the polynomial at point `z`
    pub fn eval(&self, z: Complex<T>) -> Complex<T>
    where
        T: Clone + Num,
    {
        self.coef
            .iter()
            .enumerate()
            .map(|(i, c)| c * z.powu(i as u32))
            .sum()
    }

    /// the general derivative of the polynomial, symbolic
    pub fn diff(&self) -> Pol<T>
    where
        T: From<u32>,
        Complex<T>: Clone + Mul<T, Output = Complex<T>>,
    {
        let coef = self
            .coef
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| c.clone() * (i as u32).into())
            .collect();

        Pol::new(coef)
    }

    /// the general antiderivative of the polynomial, symbolic, with constant term `c`
    pub fn integrate(&self, c: Complex<T>) -> Pol<T>
    where
        T: From<u32>,
        Complex<T>: Clone + Div<T, Output = Complex<T>>,
    {
        let mut coef = self
            .coef
            .iter()
            .enumerate()
            .map(|(i, c)| c.clone() / (i as u32 + 1).into())
            .collect::<Vec<_>>();

        coef.insert(0, c);

        Pol::new(coef)
    }

    /// attempt to find all the complex roots of the polynomial with radii `n` * its degree, maximum iterations of newton's method `m`, coefficient of rounding for eliminating identical solutions `r_coef`, and `eps`, passed to sampling and newton's method, and used for rounding
    /// a degree k polynomial will have k roots (although they may be the same) and is not trivially solved
    /// differentiating this a times will yield a polynomial with k - a roots
    /// if k - a = 1 or 2, this can be trivially solved for its zeros
    /// the zeros of the derivative of a polynomial lie inside the convex hull of its zeros by the Gauss-Lucas theorem
    /// therefore, we begin newton's method at the points of the derivative's zeros radiating out
    /// to attempt to land initial points in the basins of convergence of every root
    pub fn roots(&self, n: usize, m: usize, r_coef: T, eps: T) -> Vec<Complex<T>>
    where
        T: Clone + Num + Neg + Float + Send + Sync + From<u32> + From<f64>,
        Complex<T>: Mul<Complex<T>, Output = Complex<T>>
            + Div<Complex<T>, Output = Complex<T>>
            + Mul<T, Output = Complex<T>>
            + Div<T, Output = Complex<T>>
            + From<T>,
    {
        match self.coef[..] {
            [] => vec![],
            [b, a] => vec![(-b / a).into()],
            [c, b, a] => {
                let d: Complex<T> = Complex::from(From::<f64>::from(4.)) * a * c;
                vec![
                    (-b - (b.powu(2) - d).sqrt()) / (Complex::from(From::<f64>::from(2.)) * a),
                    (-b + (b.powu(2) - d).sqrt()) / (Complex::from(From::<f64>::from(2.)) * a),
                ]
            }
            _ => newt(
                self,
                samp(
                    &self.diff().roots(n, m, r_coef, eps),
                    n * (self.coef.len() - 1),
                    eps,
                ),
                m,
                eps,
            ),
        }
        .into_iter()
        .unique_by(|a| {
            Complex::new(
                (a.re * (r_coef * eps).recip()).round().to_i64(),
                (a.im * (r_coef * eps).recip()).round().to_i64(),
            )
        })
        .collect()
    }
}
