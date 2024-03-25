use std::collections::HashMap;

use rug::{Complete, Integer};

fn discrete_log(b: &Integer, a: &Integer, m: &Integer, x_max_exp: usize) -> Integer {
    let B: usize = 1 << (x_max_exp / 2);
    let b = Integer::from(B as u64);

    // Build table.
    let mut table = HashMap::with_capacity(B);
    let g_inv = Integer::from(b.invert_ref(m).unwrap());
    let mut lhs = Integer::from(a % m);
    for x1 in 0..B {
        if x1 > 0 {
            lhs = (lhs * &g_inv) % m;
        }
        table.insert(lhs.clone(), x1);
    }

    // Find collision.
    let g_b = b.pow_mod(&b, m).unwrap();
    let mut rhs = Integer::from(1);
    for x0 in 0..B {
        if x0 > 0 {
            rhs = (rhs * &g_b) % m;
        }

        if let Some(&x1) = table.get(&rhs) {
            return Integer::from(x0 * B + x1);
        }
    }
    panic!("`x_max_exp` is too small");
}

pub fn repetend_len(n: &Integer, base: &Integer) -> Integer {
    let (n, _) = n.clone().remove_factor(&n.gcd_ref(&base).complete());

    if n == 1 {
        return Integer::from(0);
    }

    discrete_log(&base, &Integer::from(1), &n, 40)
}
