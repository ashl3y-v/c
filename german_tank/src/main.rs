use nalgebra::{Const, DMatrix, DVector, Dyn};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::ops::Range;

fn gen_data(n: &Range<usize>, k: usize, l: usize) -> (DMatrix<f64>, DVector<f64>) {
    let d = n
        .to_owned()
        .into_par_iter()
        .flat_map(|n_i| {
            let mut rng = thread_rng();

            (0..l)
                .flat_map(|_| {
                    let mut s = Vec::new();

                    for _ in 1..k {
                        let t = rng.gen_range(1..=n_i);

                        if !s.contains(&t) {
                            s.push(t);
                        }
                    }

                    let m = s.into_iter().max().unwrap() as f64;

                    [1., m]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x = DMatrix::from_row_slice_generic(Dyn(n.len() * l), Dyn(2), d.as_slice());

    let b = DVector::from_iterator_generic(
        Dyn(n.len() * l),
        Const::<1>,
        n.clone()
            .flat_map(|x| [x as f64].repeat(l))
            .collect::<Vec<_>>(),
    );

    (x, b)
}

fn main() {
    let n = 1..10000;
    let k = 4;
    let l = 100;

    let (x, b) = gen_data(&n, k, l);

    let coef = x
        .clone()
        .pseudo_inverse(f64::EPSILON)
        .expect("Pseudoinverse failed")
        * b.clone();

    let their_coef = [-1., 1. + 1. / k as f64];
    let their_coef =
        DVector::from_column_slice_generic(Dyn(their_coef.len()), Const::<1>, &their_coef);

    println!("{}", coef);
    println!("{}", their_coef);

    let err = (&b - &x * &coef).abs();
    let their_err = (&b - &x * &their_coef).abs();

    dbg!(err.mean());

    println!("generated dist");
    dbg!(err.mean() - their_err.mean());
    dbg!(err.variance().sqrt() - their_err.variance().sqrt());

    let (x, b) = gen_data(&(990..1000), k, l);

    let err = (&b - &x * &coef).abs();
    let their_err = (&b - &x * &their_coef).abs();

    println!("test dist");
    dbg!(err.mean() - their_err.mean());
    dbg!(err.variance().sqrt() - their_err.variance().sqrt());
}
