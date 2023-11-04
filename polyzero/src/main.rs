use polyzero::{newt, samp, Complex, Pol};

fn main() {
    let im = |re, im| Complex::new(re, im);

    let n = 8;
    let m = 10000;
    let eps = (2_f64).powi(9) * f64::EPSILON;
    let r_coef = (2_f64).powi(4);

    // let p = Pol::new(vec![
    //     im(-20736., 0.),
    //     im(0., 0.),
    //     im(19440., 0.),
    //     im(0., 0.),
    //     im(2848., 0.),
    //     im(0., 0.),
    //     im(-1455., 0.),
    //     im(0., 0.),
    //     im(-113., 0.),
    //     im(0., 0.),
    //     im(16., 0.),
    //     im(0., 0.),
    //     im(1., 0.),
    // ]);

    // let p = Pol::new(vec![
    //     im(518400., 0.),
    //     im(0., 0.),
    //     im(-773136., 0.),
    //     im(0., 0.),
    //     im(296296., 0.),
    //     im(0., 0.),
    //     im(-44473., 0.),
    //     im(0., 0.),
    //     im(3003., 0.),
    //     im(0., 0.),
    //     im(-91., 0.),
    //     im(0., 0.),
    //     im(1., 0.),
    // ]);

    let p = Pol::new(vec![
        im(4., 0.),
        im(0., 0.),
        im(5., 0.),
        im(0., 0.),
        im(1., 0.),
    ]);

    let z = p.roots(n, m, r_coef, eps);
    dbg!(&z);

    // dbg!(z.iter().map(|z| p.eval(z.clone())).collect::<Vec<_>>());
    // dbg!(z
    //     .iter()
    //     .map(|a| Complex::new(
    //         (a.re * eps.recip()).round() as i64,
    //         (a.im * eps.recip()).round() as i64,
    //     ))
    //     .collect::<Vec<_>>());
}
