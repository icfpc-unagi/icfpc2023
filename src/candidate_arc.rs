use std::f64::consts::PI;

use crate::*;

pub fn solve(d: f64, k: usize) -> Vec<P> {
    // // binary search x in [d, \infty] s.t.
    // let mut lb = d;
    // let mut ub = d + 5.0 * k as f64;
    let inv_k = 1.0 / (k as f64);

    let theta = {
        // binary search theta s.t.
        // sin (theta / k) = 5.0 / x,
        // cos (theta) = d / x.
        let mut lb = 0.0;
        let mut ub = 0.5 * PI;
        for _ in 0..20 {
            let theta = (lb + ub) / 2.0;
            let sin = (inv_k * theta).sin();
            let cos = theta.cos();
            if d * sin < 5.0 * cos {
                lb = theta;
            } else {
                ub = theta;
            }
        }
        if k >= 2 {
            ub *= 1.0 + 1e-8;
        }
        ub
    };

    let x0 = d / theta.cos();
    let x1 = x0 * (inv_k * theta).cos() + (5.0 * 3.0_f64.sqrt() + 1e-8);
    let k = k as i32;
    (-k..=k)
        .map(|i| {
            let th = (i as f64) * inv_k * theta;
            let r = if (i + k) % 2 == 0 { x0 } else { x1 };
            P(r * th.cos(), r * th.sin())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        let k = 3;
        let points = solve(10.0, 3);
        let n = 2 * k + 1;
        assert_eq!(points.len(), n);
        dbg!(&points);
        let output = Output::from((points.clone(), vec![10.0; n]));
        write_output(&output);
        // assert!(false);
    }
}
