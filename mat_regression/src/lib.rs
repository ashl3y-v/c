use nalgebra::{DMatrix, DVector};

pub fn arg_data(args: &[String], n: usize) -> Vec<f64> {
    args[n]
        .split(", ")
        .map(|x| x.parse::<f64>().expect("Failed to parse data, {n}"))
        .collect::<Vec<f64>>()
}

pub fn eval(x: f64, coeffs: &DVector<f64>) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(ci, cn)| cn * x.powi(ci as i32))
        .sum::<f64>()
}

pub fn cost(x: &DVector<f64>, y: &DVector<f64>, coeffs: &DVector<f64>) -> f64 {
    x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - eval(*xi, coeffs)).powi(2))
        .sum::<f64>()
        / x.len() as f64
}

pub fn fit(x: &DVector<f64>, y: &DVector<f64>, degree: usize) -> DVector<f64> {
    let n = x.len();

    // Create the design matrix
    let mut dm = DMatrix::zeros(n, degree + 1);
    for i in 0..n {
        for j in 0..degree + 1 {
            dm[(i, j)] = x[i].powi(j as i32);
        }
    }

    (dm.transpose() * &dm)
        .try_inverse()
        .expect("Inverse failed")
        * &dm.transpose()
        * y
}

pub fn search(mut cost_func: impl FnMut(i64) -> f64, range: (i64, i64)) -> i64 {
    let range = Vec::from_iter(range.0..range.1);
    let mut costs = Vec::new();

    for s in &range {
        costs.push(cost_func(*s));
    }

    let min = costs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    *range
        .get(costs.iter().position(|&o| o == min).unwrap())
        .unwrap()
}

pub fn pretty(coefficients: &DVector<f64>) -> String {
    let raw_eq: String = coefficients
        .iter()
        .enumerate()
        .map(|(i, t)| {
            if i < coefficients.len() - 1 {
                format!("{t:.3} x^{i} + ")
            } else {
                format!("{t:.3} x^{i}")
            }
        })
        .collect();
    raw_eq.replace(" x^0", "").replace("x^1", "x")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_test() {
        assert_eq!(
            pretty(&DVector::from_vec(vec![1.0, 2.0, 3.0])),
            "1.000 + 2.000 x + 3.000 x^2"
        );
    }
}
