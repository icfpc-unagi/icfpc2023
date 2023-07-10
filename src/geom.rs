use crate::*;

const EPS: f64 = 1e-8;

// p+dir*epsを求める。途中でぶつかる場合はそこまで進める
// 最初からぶつかっている場合は法線方向の成分を消す
pub fn first_hit(
    input: &Input,
    musicians: &Vec<P>,
    powers: &mut Vec<P>,
    p: P,
    mut dir: P,
    mut eps: f64,
) -> P {
    let mut d;
    macro_rules! update {
        () => {
            d = dir.abs();
            if d < EPS {
                return p;
            }
        };
    }
    update!();
    if dir.0 < 0.0 {
        if (input.stage0.0 + 10.0 - p.0).abs() < EPS {
            dir.0 = 0.0;
            update!();
        }
        eps.setmin((input.stage0.0 + 10.0 - p.0) / dir.0);
    }
    if dir.0 > 0.0 {
        if (input.stage1.0 - 10.0 - p.0).abs() < EPS {
            dir.0 = 0.0;
            update!();
        }
        eps.setmin((input.stage1.0 - 10.0 - p.0) / dir.0);
    }
    if dir.1 < 0.0 {
        if (input.stage0.1 + 10.0 - p.1).abs() < EPS {
            dir.1 = 0.0;
            update!();
        }
        eps.setmin((input.stage0.1 + 10.0 - p.1) / dir.1);
    }
    if dir.1 > 0.0 {
        if (input.stage1.1 - 10.0 - p.1).abs() < EPS {
            dir.1 = 0.0;
            update!();
        }
        eps.setmin((input.stage1.1 - 10.0 - p.1) / dir.1);
    }
    for i in 0..musicians.len() {
        let c = musicians[i];
        if (c - p).dot(dir) > 0.0 {
            if (c - p).abs() < 10.0 + 2.0 * EPS {
                let r = (c - p) * 0.1;
                powers[i] = powers[i] + r * r.dot(dir);
                dir = dir - r * r.dot(dir);
                update!();
            } else {
                for q in P::pi_cl((c, 10.0 + EPS), (p, p + dir)) {
                    eps.setmin((q - p).dot(dir) / d);
                }
            }
        }
    }
    let mut q = p + dir * eps;
    if (q.0 - (input.stage0.0 + 10.0)).abs() < 1e-8 {
        q.0 = input.stage0.0 + 10.0;
    }
    if (q.0 - (input.stage1.0 - 10.0)).abs() < 1e-8 {
        q.0 = input.stage1.0 - 10.0;
    }
    if (q.1 - (input.stage0.1 + 10.0)).abs() < 1e-8 {
        q.1 = input.stage0.1 + 10.0;
    }
    if (q.1 - (input.stage1.1 - 10.0)).abs() < 1e-8 {
        q.1 = input.stage1.1 - 10.0;
    }
    q
}
