use bbp::chudnovsky;
use rug::Integer;

fn main() {
    let n = 0x40000;
    let p = 0x8000;
    let last = chudnovsky(Integer::from(n), p);
    let sec_to_last = chudnovsky(Integer::from(n - 1), p);
    let dif = last.clone() - sec_to_last.clone();

    println!("{}\n{}", last, dif);
}
