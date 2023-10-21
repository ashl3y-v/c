use num::{Complex, Num};

pub fn d<T: Num + Copy>(f: &impl Fn(T) -> T, x: T, a: T, o: usize) -> T {
    if o == 1 {
        (f(x + a) - f(x)) / a
    } else {
        (d(&f, x + a, a, o - 1) - d(&f, x, a, o - 1)) / a
    }
}

pub fn newton(
    f: impl Fn(Complex<f64>) -> Complex<f64>,
    z: Vec<Complex<f64>>,
    a: Complex<f64>,
) -> Vec<Complex<f64>> {
    z.into_iter()
        .map(|mut x| {
            let mut y = f(x);
            while y.norm() > a.norm() {
                y = f(x);
                x -= y / d(&f, &x, &a, 1);
            }
            x
        })
        .collect()
}

pub fn newton_min(
    f: impl Fn(Complex<f64>) -> Complex<f64>,
    z: Vec<Complex<f64>>,
    a: Complex<f64>,
) -> Vec<Complex<f64>> {
    let df = { |x| d(&f, &x, &a, 1) };
    newton(df, z, a)
}
