use eigenzeros::{eigenzeros, eigenzeros_complex};
use nalgebra::{Complex, DMatrix, Dyn};

fn main() {
    // let im = Complex::new;

    let p = vec![1., 0., 1.];

    dbg!(eigenzeros_complex(p));
}
