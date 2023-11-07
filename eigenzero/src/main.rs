#![feature(float_gamma)]

use aberth::AberthSolver;
use nalgebra::{Complex, DMatrix, Dyn};
use ndarr

fn main() {
    // let im = Complex::new;

    let p = vec![4., 0., 5., 0.];

    let leading_coef = p[p.len() - 1];

    let p = p.into_iter().map(|c| c / leading_coef).collect::<Vec<_>>();

    let a = DMatrix::from_fn_generic(Dyn(p.len()), Dyn(p.len()), |i, j| {
        // hi
        if j == p.len() - 1 {
            -p[i]
        } else if i < 1 {
            0.
        } else if i - 1 == j {
            1.
        } else {
            0.
        }
    });

    let e = a.complex_eigenvalues();

    let r = e
        .iter()
        .map(|v| {
            p.iter()
                .enumerate()
                .map(|(i, c)| c * v.powi(i as i32))
                .sum::<Complex<f64>>()
        })
        .collect::<Vec<_>>();

    println!("{}", a);
    println!("{}", e);
    println!("{:?}", r);

    // let mut solver = AberthSolver::new();
    // solver.epsilon = f64::EPSILON;
    // solver.max_iterations = 128;
}
