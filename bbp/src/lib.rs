use rug::ops::Pow;
use rug::Float;
use rug::Integer;

pub fn binary_split(a: Integer, b: Integer) -> (Integer, Integer, Integer) {
    if b == a.clone() + 1 {
        let pab: Integer = (6 * a.clone() + 5) * (2 * a.clone() - 1) * (6 * a.clone() - 1);
        let qab = Integer::from(10939058860032000_i64) * a.clone().pow(3);
        let rab = pab.clone() * (545140134 * a.clone() + 13591409);

        (pab, qab, rab)
    } else {
        let m: Integer = (a.clone() + b.clone()) / 2;
        let (pam, qam, ram) = binary_split(a.clone(), m.clone());
        let (pmb, qmb, rmb) = binary_split(m.clone(), b.clone());

        (pam.clone() * pmb, qam * qmb.clone(), qmb * ram + pam * rmb)
    }
}

pub fn chudnovsky(n: Integer, p: u32) -> Float {
    let (_p1n, q1n, r1n) = binary_split(1.into(), n);
    (426880 * Float::with_val(p, 10005).sqrt() * &q1n) / (13591409 * q1n + r1n)
}
