#![feature(iter_map_windows)]

const EPSILON: f64 = 128. * f64::EPSILON;

pub fn newt(p: &Pol, x: Vec<f64>) -> Vec<f64> {
    let dp = p.diff();

    x.into_iter()
        .map(|mut x| {
            // stupid, (x - x + 2*ε) > ε
            let mut x_l = x + 2. * EPSILON;
            while (x - x_l).abs() > EPSILON {
                x_l = x;
                x -= p.eval(x) / dp.eval(x);
            }
            x
        })
        .collect()
}

pub fn samp(x: &Vec<f64>) -> Vec<f64> {
    let mut v = x
        .iter()
        .map_windows(|[a, b]| (*a + *b) / 2.)
        .collect::<Vec<_>>();
    v.insert(0, x[0] - EPSILON);
    v.push(x[x.len() - 1] + EPSILON);

    v
}

#[derive(Debug)]
pub struct Pol {
    coeff: Vec<f64>,
}

impl Pol {
    pub fn new(coeff: Vec<f64>) -> Self {
        Pol { coeff }
    }

    pub fn eval(&self, x: f64) -> f64 {
        self.coeff
            .iter()
            .enumerate()
            .map(|(i, c)| c * x.powi(i as i32))
            .sum()
    }

    pub fn diff(&self) -> Pol {
        let coeff = self
            .coeff
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| i as f64 * c)
            .collect();

        Pol::new(coeff)
    }

    pub fn integrate(&self, c: f64) -> Pol {
        let mut coeff = self
            .coeff
            .iter()
            .enumerate()
            .map(|(i, c)| c / (i + 1) as f64)
            .collect::<Vec<_>>();

        coeff.insert(0, c);

        Pol::new(coeff)
    }

    pub fn zeros(&self) -> Vec<f64> {
        match self.coeff.len() {
            1 => vec![],
            2 => vec![-self.coeff[0] / self.coeff[1]],
            _ => newt(self, samp(&self.diff().zeros())),
        }
    }
}

fn main() {
    let p = Pol::new(vec![
        518400., 0., -773136., 0., 296296., 0., -44473., 0., 3003., 0., -91., 0., 1.,
    ]);

    dbg!(p.zeros());
}
