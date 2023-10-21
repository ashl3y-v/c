use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn main() {
    let n_trials = 1000000;
    let n_values = 15;
    let v_range = 0..10;

    let rs = (0..n_trials)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();

            let mut vs = Vec::new();

            for _ in 0..n_values {
                let v = (
                    rng.gen_range(v_range.clone()),
                    rng.gen_range(v_range.clone()),
                );

                let v = match v {
                    (0, 0) => "100".to_string(),
                    (a, b) => format!("{}{}", a, b),
                };

                let v = v.parse().unwrap();

                vs.push(v);
            }

            let r = vs.iter().sum::<f64>() / vs.len() as f64;

            r
        })
        .collect::<Vec<f64>>();

    dbg!(rs.iter().filter(|x| **x >= 70.).count() as f64 / rs.len() as f64);
}
