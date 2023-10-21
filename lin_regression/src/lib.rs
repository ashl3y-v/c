use ndarray::Array;

pub fn arg_data(args: &[String], n: usize) -> Vec<f64> {
    args[n]
        .split(", ")
        .map(|x| x.parse::<f64>().expect("Failed to parse data, {n}"))
        .collect::<Vec<f64>>()
}

pub fn eval(x: &[f64], coeffs: &Array<f64>) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(ci, cn)| cn * x.powi(ci as i32))
        .sum::<f64>()
}

pub fn cost(x: &Array<f64>, y: &Array<f64>, coeffs: &Array<f64>) -> f64 {
    x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - eval(*xi, coeffs)).powi(2))
        .sum::<f64>()
        / x.len() as f64
}

pub fn fit(x: &Array<f64>, y: &Array<f64>) -> Array<f64> {
    let n = x.len();

    // Create the design matrix
    let mut dm = Array::zeros(n, degree + 1);
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

pub fn pretty(coefficients: &Array<f64>) -> String {
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
