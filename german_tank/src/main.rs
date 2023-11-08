/*
n: true number of tanks
n_hat: our prediction
k: number observed
m: maximum observed

axioms:
the number of tanks will be between 1000 and 1000000
instead of cheating, add nat log terms for fit
formula does not contain an explicit term for k except in modifying other terms
n_hat prop m
n_hat - m grows inv k

n_hat = c_0 m + c_1 ln(m) + c_2 ln(k) + c_3 ln(σ)
*/

use std::ops::Range;

use nalgebra::{Const, DMatrix, DVector, Dyn};
use rand::{thread_rng, Rng};

fn gen_data(n: usize, k: usize, rng: &mut impl Rng) -> DMatrix<f64> {
    let mut d = Vec::new();

    for _ in 0..n {
        let mut s = Vec::new();

        for _ in 0..k {
            let t = rng.gen_range(0..n);

            if !s.contains(&t) {
                s.push(t);
            }
        }

        d.extend_from_slice(&[s.into_iter().max().unwrap() as f64])
    }

    dbg!(&d);

    dbg!(&d.len());

    DMatrix::from_row_slice_generic(Dyn(n), Dyn(1), d.as_slice())
}

fn main() {
    let mut rng = thread_rng();

    let n = 10000;
    let k = 4;

    let b = DVector::from_iterator_generic(Dyn(n), Const::<1>, 0..n);

    let x = gen_data(n, k, &mut rng);

    let coef = x.clone().pseudo_inverse(f64::EPSILON).unwrap() * b.clone();

    println!("{}", coef);
}
