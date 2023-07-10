use std::f64::consts::PI;

use crate::*;

/// returns pos of 2k+1 musicians
pub fn get_candidate(input: &Input, attendee_index: usize, k: usize) -> Vec<P> {
    let p = input.pos[attendee_index];
    let s0 = input.stage0 + P(10.0, 10.0);
    let s1 = input.stage1 - P(10.0, 10.0);
    let signs = [p.0 > s0.0, p.0 > s1.0, p.1 > s0.1, p.1 > s1.1];
    let (rot, neg, d) = match signs {
        [false, false, true, false] => (false, false, s0.0 - p.0),
        [true, true, true, false] => (false, true, p.0 - s1.0),
        [true, false, false, false] => (true, false, s0.1 - p.1),
        [true, false, true, true] => (true, true, p.1 - s1.1),
        _ => return vec![],
    };
    debug_assert!(d >= 10.0);
    solve(d, k)
        .into_iter()
        .map(|v| if rot { v.rot() } else { v })
        .map(|v| if neg { p - v } else { p + v })
        .collect()
}

pub fn solve(d: f64, k: usize) -> Vec<P> {
    if k == 0 {
        return vec![P(d, 0.0)];
    }
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

    // #[test]
    // fn test_candidate() {
    //     let input = EXAMPLE_INPUT;
    // }
}
