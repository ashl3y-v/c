use eigenzeros::eigenzeros;
use nalgebra::{Complex, DMatrix, Dyn};

fn main() {
    let im = Complex::new;

    let p = vec![im(1., 0.), im(0., 0.), im(0., 0.), im(1., 0.)];

    dbg!(eigenzeros(p));
}
