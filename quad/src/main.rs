use num::{Complex, Signed};
use std::env;
use std::fmt::Display;

fn quad(a: f64, b: f64, c: f64) -> (Complex<f64>, Complex<f64>) {
    let m = -b / (2. * a);
    let d: Complex<f64> = (m.powi(2) - c / a).into();
    let d = d.sqrt();

    (m - d, m + d)
}

fn pretty_complex<T: Signed + Display>(z: Complex<T>) -> String {
    let a = z.re;
    let b = z.im;

    let mut op = "+";
    if b.is_negative() {
        op = "-";
    }
    let b = b.abs();

    format!("{a} {op} {b}i")
}

fn main() {
    let args = env::args().skip(1);
    let args: Vec<f64> = args.map(|x| x.parse::<f64>().unwrap()).collect();

    let sols = quad(args[0], args[1], args[2]);

    println!(
        " _\n| {}\n| {}",
        pretty_complex(sols.0),
        pretty_complex(sols.1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_test() {
        assert_eq!(pretty_complex(Complex::new(3, -4)), "3 - 4i");
    }

    #[test]
    fn quad_test() {
        assert_eq!(
            quad(1., 0., 1.),
            (Complex::new(0., -1.), Complex::new(0., 1.))
        );
    }

    #[test]
    fn quad_pretty_unit_test() {
        let sols = quad(1., 0., 1.);

        assert_eq!(pretty_complex(sols.0), "0 - 1i");
        assert_eq!(pretty_complex(sols.1), "0 + 1i");
    }
}
