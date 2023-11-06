#![feature(float_gamma)]

use num_complex::Complex;
use polynomial::Polynomial;
use polyzero::roots;

// worse at solving for reals than imaginary

fn main() {
    let im = Complex::new;

    im(0., 0.);

    let radii_coef = 2;
    let max_iter = 2usize.pow(12);
    let round_coef = 2_f64.powi(4);
    let eps = 2_f64.powi(2) * f64::EPSILON;

    let n = 30;
    let p = Polynomial::new(
        (0..n)
            .map(|n| match n % 2 {
                0 => Complex::new(0., 0.),
                _ => Complex::new(
                    (-1f64).powi(n as i32 / 2) * (n as f64 + 1.).gamma().recip(),
                    0.,
                ),
            })
            .collect::<Vec<_>>(),
    );

    // let p = Polynomial::new(vec![
    //     im(40320., 0.),
    //     im(0., 109584.),
    //     im(-118124., 0.),
    //     im(0., -67284.),
    //     im(22449., 0.),
    //     im(0., 4536.),
    //     im(-546., 0.),
    //     im(0., -36.),
    //     im(1., 0.),
    // ]);

    dbg!(&p);

    dbg!(roots(&p, radii_coef, max_iter, round_coef, eps));

    // let v = vec![im(5., 5.)];
    // dbg!(halley(&p, v, max_iter, eps));
}
