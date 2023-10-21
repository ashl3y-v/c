use lin_regression::{arg_data, cost, fit, pretty, search};
use ndarray::Array;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let x_arg = arg_data(&args, 1);
    let y_arg = arg_data(&args, 2);
    let degree_min = args
        .get(3)
        .expect("Could not get polynomial degree")
        .clone()
        .parse::<usize>()
        .expect("Polynomial degree parsing failed!");
    let degree_max = args.get(4);
    match degree_max {
        Some(x) => {
            let degree_max = x
                .clone()
                .parse::<usize>()
                .expect("Polynomial degree parsing failed");

            let x = Array::from_vec(x_arg);
            let y = Array::from_vec(y_arg);

            let mut cost_func = |degree| {
                let coefficients = fit(&x, &y, degree);
                cost(&x, &y, &coefficients)
            };

            println!("optimal: {}", search(cost_func, degree_min..degree_max));
        }
        None => {
            let x = Array::from_vec(x_arg);
            let y = Array::from_vec(y_arg);
            let degree = degree_min;

            let coefficients = fit(&x, &y, degree);
            println!(
                "expression: {:?}\nerror: {}",
                pretty(&coefficients),
                cost(&x, &y, &coefficients)
            );
        }
    }
}
