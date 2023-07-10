#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr, ops::Mul};

use aead::NewAead;
use rand::Rng;

use crate::{candidate_arc, Input, P};

pub fn get_all_candidate2(inp: &Input) -> Vec<P> {
    let mut ret = vec![];
    for i in 0..8 {
        let vp: Vec<P> = vec![];
        let r2 = get_candidate3(inp, &vp, i, false);
        for r in r2 {
            ret.push(r);
        }
    }
    ret
}

pub fn get_candidate3(
    inp: &Input,
    first_cand: &Vec<P>,
    rand_flag: usize,
    has_pillar: bool,
) -> Vec<P> {
    let mut candidate = Vec::new();

    let mut heap = BinaryHeap::new();

    let mut rng = rand::thread_rng();

    for i in 0..first_cand.len() {
        candidate.push(first_cand[i]);
    }

    let random_space = rng.gen_range(0.0, 9.99);

    let ten = {
        if rand_flag % 2 == 0 {
            10.0 + random_space
        } else {
            10.0
        }
    };

    let base_pos = {
        if (rand_flag >> 1) % 2 == 0 {
            if rng.gen_bool(0.5) {
                rng.gen_range(-5, 6) as f64
            } else {
                rng.gen_range(-5.0, 5.0)
            }
        } else {
            0.0
        }
    };

    let wide = rng.gen_range(0.5, 9.5);

    for i in 0..inp.pos.len() {
        let dist = get_stage_diff(inp.pos[i], inp.stage0, inp.stage1) as i64;

        let mut maxpower = -1000.0;
        for power in &inp.tastes[i] {
            if maxpower < *power {
                maxpower = *power;
            }
        }
        if maxpower <= 0.0 {
            continue;
        }

        let mut pattern = 0;
        if inp.pos[i].0 < inp.stage0.0 {
            pattern += 1;
        }
        if inp.pos[i].1 < inp.stage0.1 {
            pattern += 2;
        }
        if inp.pos[i].0 > inp.stage1.0 {
            pattern += 4;
        }
        if inp.pos[i].1 > inp.stage1.1 {
            pattern += 8;
        }

        for j in 0..5 {
            if base_pos != 0.0 && j != 0 && j != 4 {
                continue;
            }

            heap.push((
                (-dist as f64 * 100000.0 / maxpower) as i64 + rng.gen_range(0.0, 1000.0) as i64,
                pattern,
                0,
                i,
                j,
            ));
        }
    }

    let _r3 = 5.0 * 1.73205 + 0.1;

    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        let dist = node.0;
        let pattern = node.1;
        let num = node.2;
        let id = node.3;
        let chal_type = node.4;

        let mut flag = false;

        if chal_type == 0 {
            let mut ps = Vec::new();

            if pattern == 1 {
                let a = (inp.stage0.0 + 10.0) - inp.pos[id].0;
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(inp.stage0.0 + 10.0 + c, base_pos + inp.pos[id].1);
                let p_1 = P(
                    inp.stage0.0 + 10.0,
                    base_pos + inp.pos[id].1 + b + ten * num as f64,
                );
                let p_2 = P(
                    inp.stage0.0 + 10.0,
                    base_pos + inp.pos[id].1 - b - ten * num as f64,
                );
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            } else if pattern == 4 {
                let a = inp.pos[id].0 - (inp.stage1.0 - 10.0);
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(inp.stage1.0 - 10.0 - c, base_pos + inp.pos[id].1);
                let p_1 = P(
                    inp.stage1.0 - 10.0,
                    base_pos + inp.pos[id].1 - b - ten * num as f64,
                );
                let p_2 = P(
                    inp.stage1.0 - 10.0,
                    base_pos + inp.pos[id].1 + b + ten * num as f64,
                );
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            } else if pattern == 2 {
                let a = (inp.stage0.1 + 10.0) - inp.pos[id].1;
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(base_pos + inp.pos[id].0, inp.stage0.1 + 10.0 + c);
                let p_1 = P(
                    base_pos + inp.pos[id].0 + b + ten * num as f64,
                    inp.stage0.1 + 10.0,
                );
                let p_2 = P(
                    base_pos + inp.pos[id].0 - b - ten * num as f64,
                    inp.stage0.1 + 10.0,
                );
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            } else if pattern == 8 {
                let a = inp.pos[id].1 - (inp.stage1.1 - 10.0);
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(base_pos + inp.pos[id].0, inp.stage1.1 - 10.0 - c);
                let p_1 = P(
                    base_pos + inp.pos[id].0 + b + ten * num as f64,
                    inp.stage1.1 - 10.0,
                );
                let p_2 = P(
                    base_pos + inp.pos[id].0 - b - ten * num as f64,
                    inp.stage1.1 - 10.0,
                );
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            }

            //dbg!(inp.pos[id]);

            if num == 0 {
                let mut point = 0;
                for i in 0..ps.len() {
                    let p = ps[i];

                    //dbg!(p);

                    if check_all_cand(&inp, &candidate, p) {
                        if i <= 2 {
                            point += 3;
                        } else {
                            point += 1;
                        }
                    }
                }

                //dbg!(point, pattern);

                if point >= 10 {
                    for i in 0..ps.len() {
                        let p = ps[i];
                        if check_all_cand(&inp, &candidate, p) {
                            candidate.push(p);
                        }
                    }
                    //dbg!(p2);
                    heap.push((dist - 100, pattern, num + 1, id, chal_type));
                } else {
                    heap.push((dist - 10, pattern, num, id, 1));
                }
            } else {
                let mut flag = false;

                for i in 1..3 {
                    let p = ps[i];
                    if check_all_cand(&inp, &candidate, p) {
                        candidate.push(p);
                        flag = true;
                    }
                }

                if flag {
                    heap.push((dist - 100, pattern, num + 1, id, chal_type));
                }
            }
        } else if chal_type == 1 {
            let mut ps = Vec::new();

            let add_mini = {
                if base_pos as i64 as f64 == base_pos {
                    0.0
                } else {
                    0.000000001
                }
            };

            if pattern == 2 {
                ps = vec![
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage0.1 + 10.0,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage0.1 + 10.0,
                    ),
                ];
            } else if pattern == 8 {
                ps = vec![
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage1.1 - 10.0,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage1.1 - 10.0,
                    ),
                ];
            } else if pattern == 1 {
                ps = vec![
                    P(
                        inp.stage0.0 + 10.0,
                        base_pos
                            + inp.pos[id].1
                            + (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                    P(
                        inp.stage0.0 + 10.0,
                        base_pos + inp.pos[id].1
                            - (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                ];
            } else if pattern == 4 {
                ps = vec![
                    P(
                        inp.stage1.0 - 10.0,
                        base_pos
                            + inp.pos[id].1
                            + (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                    P(
                        inp.stage1.0 - 10.0,
                        base_pos + inp.pos[id].1
                            - (5.00 + ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                ];
            }

            for p in ps {
                if check_all_cand(&inp, &candidate, p) {
                    candidate.push(p);
                    flag = true;
                }
            }

            if flag {
                heap.push((dist - 100, pattern, num + 1, id, chal_type));
            }
        } else if chal_type == 2 {
            let mut ps = Vec::new();

            let add_mini = {
                if base_pos as i64 as f64 == base_pos {
                    0.0
                } else {
                    0.000000001
                }
            };

            if pattern == 2 {
                ps = vec![
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage0.1 + 10.0,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage0.1 + 10.0,
                    ),
                ];
            } else if pattern == 8 {
                ps = vec![
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage1.1 - 10.0,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                        inp.stage1.1 - 10.0,
                    ),
                ];
            } else if pattern == 1 {
                ps = vec![
                    P(
                        inp.stage0.0 + 10.0,
                        base_pos
                            + inp.pos[id].1
                            + (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                    P(
                        inp.stage0.0 + 10.0,
                        base_pos + inp.pos[id].1
                            - (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                ];
            } else if pattern == 4 {
                ps = vec![
                    P(
                        inp.stage1.0 - 10.0,
                        base_pos
                            + inp.pos[id].1
                            + (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                    P(
                        inp.stage1.0 - 10.0,
                        base_pos + inp.pos[id].1
                            - (ten * num as f64 + add_mini * (num as f64 + 1.0)),
                    ),
                ];
            }

            for p in ps {
                if check_all_cand(&inp, &candidate, p) {
                    candidate.push(p);
                    flag = true;
                }
            }

            if flag {
                heap.push((dist - 100, pattern, num + 1, id, chal_type));
            }
        } else if chal_type == 3 {
            //あなをあける！
            let add_d = (wide + 10.0) / 2.0;
            let up = (100.0 as f64 - add_d * add_d).sqrt() + 0.001;

            let add_mini = {
                if base_pos as i64 as f64 == base_pos {
                    0.0
                } else {
                    0.000000001
                }
            };

            let mut ps = Vec::new();
            if pattern == 2 {
                ps = vec![
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            + add_d,
                        inp.stage0.1 + 10.0,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            - add_d,
                        inp.stage0.1 + 10.0,
                    ),
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                        inp.stage0.1 + 10.0 + up,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                        inp.stage0.1 + 10.0 + up,
                    ),
                ];
            } else if pattern == 8 {
                ps = vec![
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            + add_d,
                        inp.stage1.1 - 10.0,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            - add_d,
                        inp.stage1.1 - 10.0,
                    ),
                    P(
                        base_pos
                            + inp.pos[id].0
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                        inp.stage1.1 - 10.0 - up,
                    ),
                    P(
                        base_pos + inp.pos[id].0
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                        inp.stage1.1 - 10.0 - up,
                    ),
                ];
            } else if pattern == 1 {
                ps = vec![
                    P(
                        inp.stage0.0 + 10.0,
                        base_pos
                            + inp.pos[id].1
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            + add_d,
                    ),
                    P(
                        inp.stage0.0 + 10.0,
                        base_pos + inp.pos[id].1
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            - add_d,
                    ),
                    P(
                        inp.stage0.0 + 10.0 + up,
                        base_pos
                            + inp.pos[id].1
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                    ),
                    P(
                        inp.stage0.0 + 10.0 + up,
                        base_pos + inp.pos[id].1
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                    ),
                ];
            } else if pattern == 4 {
                ps = vec![
                    P(
                        inp.stage1.0 - 10.0,
                        base_pos
                            + inp.pos[id].1
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            + add_d,
                    ),
                    P(
                        inp.stage1.0 - 10.0,
                        base_pos + inp.pos[id].1
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64)
                            - add_d,
                    ),
                    P(
                        inp.stage1.0 - 10.0 - up,
                        base_pos
                            + inp.pos[id].1
                            + (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                    ),
                    P(
                        inp.stage1.0 - 10.0 - up,
                        base_pos + inp.pos[id].1
                            - (5.0 + add_mini * (num as f64 + 1.0) + ten * num as f64),
                    ),
                ];
            }

            for ii in 0..ps.len() {
                if num >= 1 && ii >= 2 {
                    break;
                }
                let p = ps[ii];
                if check_all_cand(&inp, &candidate, p) {
                    candidate.push(p);
                    flag = true;
                }
            }

            if flag {
                heap.push((dist - 100, pattern, num + 1, id, chal_type));
            }
        } else if chal_type == 4 {
            //7個おき

            let mut ps = candidate_arc::get_candidate(&inp, id, 3);

            if ps.len() != 7 {
                continue;
            }
            if pattern == 1 {
                ps[0] = P(ps[0].0 + ten * num as f64, ps[0].1);
                ps[6] = P(ps[6].0 - ten * num as f64, ps[6].1);
            } else if pattern == 4 {
                ps[0] = P(ps[0].0 - ten * num as f64, ps[0].1);
                ps[6] = P(ps[6].0 + ten * num as f64, ps[6].1);
            } else if pattern == 2 {
                ps[0] = P(ps[0].0, ps[0].1 + ten * num as f64);
                ps[6] = P(ps[6].0, ps[6].1 - ten * num as f64);
            } else if pattern == 8 {
                ps[0] = P(ps[0].0, ps[0].1 - ten * num as f64);
                ps[6] = P(ps[6].0, ps[6].1 + ten * num as f64);
            }

            for ii in 0..ps.len() {
                if num >= 1 && ii != 0 && ii != 6 {
                    break;
                }
                let p = ps[ii];
                if check_all_cand(&inp, &candidate, p) {
                    candidate.push(p);
                    flag = true;
                }
            }

            if flag {
                heap.push((dist - 100, pattern, num + 1, id, chal_type));
            }
        }
    }

    if true {
        let mut next = 0;

        let mut heap = BinaryHeap::new();
        let mut ps = vec![];
        loop {
            if next < candidate.len() {
                let i = next;
                for j in 0..next {
                    if (candidate[i] - candidate[j]).abs2() >= 400.0 {
                        continue;
                    }

                    if (candidate[i] - candidate[j]).abs2() < 100.0 - 0.000000001 {
                        continue;
                    }

                    //dbg!(base_num[i], base_num[j], base_pat[i], base_pat[j]);

                    let d1 = (candidate[i] - candidate[j]).abs();
                    let d2 = (10.0 * 10.0 - d1 * d1 / 4.0).sqrt() + 0.0001;

                    let v1 = candidate[j] - candidate[i];
                    let v2 = v1.rot();
                    let v2 = P(v2.0 / d1 * d2, v2.1 / d1 * d2);

                    let p = candidate[i] + v1.mul(0.5) + v2;

                    let d1 = get_stage_diff_inside(candidate[i], inp.stage0, inp.stage1);
                    let d2 = get_stage_diff_inside(candidate[j], inp.stage0, inp.stage1);
                    let d3 = get_stage_diff_inside(p, inp.stage0, inp.stage1);

                    if d1 + d2 <= d3 + d3 {
                        ps.push(p);
                        heap.push((
                            (-get_stage_diff_inside(p, inp.stage0, inp.stage1) * 100000000000.0)
                                as i64,
                            ps.len() - 1,
                        ));
                    }

                    let p = candidate[i] + v1.mul(0.5) - v2;
                    let d3 = get_stage_diff_inside(p, inp.stage0, inp.stage1);

                    if d1 + d2 <= d3 + d3 {
                        ps.push(p);
                        heap.push((
                            (-get_stage_diff_inside(p, inp.stage0, inp.stage1) * 100000000000.0)
                                as i64,
                            ps.len() - 1,
                        ));
                    }
                }

                next += 1;
            } else {
                if heap.is_empty() {
                    break;
                }
                while !heap.is_empty() {
                    let h = heap.pop().unwrap();

                    let p = ps[h.1];

                    if !has_pillar && h.0 as f64 >= 20.0 * 100000000000.0 {
                        continue;
                    }
                    if h.0 as f64 >= 700.0 * 100000000000.0 {
                        continue;
                    }

                    //dbg!(h.0, p);
                    if check_all_cand(&inp, &candidate, p) {
                        candidate.push(p);
                        break;
                    }
                }
            }
        }
    }

    candidate = set_more_candidate(&inp, candidate);

    candidate
}

fn set_more_candidate(inp: &Input, candidate: Vec<P>) -> Vec<P> {
    let mut ret = candidate.clone();
    let stage_x = ((inp.stage1.0 - inp.stage0.0) / 10.0) as usize;
    let stage_y = ((inp.stage1.1 - inp.stage0.1) / 10.0) as usize;
    for x in 2..stage_x - 1 {
        for y in 2..stage_y - 1 {
            if ret.len() < inp.musicians.len() * 4 / 3 {
                let nx = inp.stage0.0 + (x as f64) * 10.0;
                let ny = inp.stage0.1 + (y as f64) * 10.0;
                if check_all_cand(&inp, &ret, P(nx, ny)) {
                    ret.push(P(nx, ny));
                }
            }
        }
    }
    return ret;
}

fn calc_y_to_x(y: f64) -> f64 {
    let mut ok = 11.0;
    let mut ng = 0.0;
    for _ in 0..100 {
        let mid = (ok + ng) / 2.0;
        if f(y, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn f(y: f64, x: f64) -> bool {
    let a = (x * x + y * y).sqrt() - y;
    return a * a + x * x >= 100.1;
}

fn check_all_cand(inp: &Input, cand: &Vec<P>, pos: P) -> bool {
    if pos.0 < inp.stage0.0 + 10.0 {
        return false;
    }
    if pos.1 < inp.stage0.1 + 10.0 {
        return false;
    }
    if pos.0 > inp.stage1.0 - 10.0 {
        return false;
    }
    if pos.1 > inp.stage1.1 - 10.0 {
        return false;
    }

    for p2 in cand {
        if (pos - *p2).abs2() < 100.0 {
            return false;
        }
    }
    true
}

fn get_stage_diff(target: P, lb: P, ru: P) -> f64 {
    let xdiff = {
        if target.0 < lb.0 {
            lb.0 - target.0
        } else if target.0 > ru.0 {
            target.0 - ru.0
        } else {
            0.0
        }
    };

    let ydiff = {
        if target.1 < lb.1 {
            lb.1 - target.1
        } else if target.1 > ru.1 {
            target.1 - ru.1
        } else {
            0.0
        }
    };
    xdiff + ydiff
}

fn get_stage_diff_inside(target: P, lb: P, ru: P) -> f64 {
    let mut ans = 9999999999999.9;
    let a = target.0 - lb.0;
    if ans > a {
        ans = a;
    }
    let a = target.1 - lb.1;
    if ans > a {
        ans = a;
    }

    let a = ru.0 - target.0;
    if ans > a {
        ans = a;
    }

    let a = ru.1 - target.1;
    if ans > a {
        ans = a;
    }

    ans
}
