fn eulers_method(x0: f64, y0: f64, dy: fn(f64) -> f64, h: f64, n: usize) -> Vec<f64> {
    let mut y = vec![y0];

    for i in 1..n {
        y.push(y[i - 1] + h * dy(x0 + (i as f64) * h));
    }

    y
}

fn main() {
    let x0 = 0.0;
    let y0 = 0.0;
    let dy = |x| 2.0 * x;
    let h = 0.01;
    let n = 60000;

    println!("{:?}", eulers_method(x0, y0, dy, h, n));
}
