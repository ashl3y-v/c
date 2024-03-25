use repetend_len::repetend_len;
use rug::Integer;

fn main() {
    let base = Integer::from(16);

    let lens = (1..=10)
        .map(|n| repetend_len(&Integer::from(n), &base).unwrap())
        .collect::<Vec<_>>();

    dbg!(lens);
}
