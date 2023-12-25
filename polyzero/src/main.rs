use num_complex::Complex;
use polynomial::Polynomial;
use polyzero::roots;

// worse at solving for reals than imaginary

fn main() {
    let im = Complex::new;

    let radii_coef = 2;
    let max_iter = 2usize.pow(12);
    let round_coef = 2_f64.powi(4);
    let eps = 2_f64.powi(2) * f64::EPSILON;

    let p = Polynomial::new(vec![
        im(-1., 0.),
        im(-1., 0.),
        im(0., 0.),
        im(0., 0.),
        im(0., 0.),
        im(1., 0.),
    ]);

    dbg!(&p);

    dbg!(roots(&p, radii_coef, max_iter, round_coef, eps));

    // let v = vec![im(5., 5.)];
    // dbg!(halley(&p, v, max_iter, eps));
}
