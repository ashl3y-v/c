use complex_newton::{d, newton, newton_min};
use num::Complex;

fn main() {
    // let f = { |x: Complex<f64>| (x - Complex { re: 532., im: 124. }).powi(2) };
    // let x = vec![Complex { re: 1E-3, im: 1E-3 }];
    // let a = Complex {
    //     re: 1e-10,
    //     im: 1e-10,
    // };

    // let r = newton_min(f, x, a);

    // println!("{:?}", r);

    println!(
        "{}",
        d(
            &{ |x: Complex<f64>| x.powi(2) },
            Complex { re: 10., im: 0. },
            Complex { re: 0.001, im: 0. },
            1
        )
    );
}
