#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use rand::Rng;

use crate::{Input, P};

pub fn get_candidate3(inp: &Input, first_cand: &Vec<P>, rand_flag: bool) -> Vec<P> {
    let mut candidate = Vec::new();

    let mut heap = BinaryHeap::new();

    let mut rng = rand::thread_rng();

    for i in 0..first_cand.len() {
        candidate.push(first_cand[i]);
    }

    let random_space = rng.gen_range(0.0, 3.0);

    let ten = {
        if rand_flag {
            10.0 + random_space
        } else {
            10.0
        }
    };

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

        for j in 0..3 {
            heap.push((
                (-dist as f64 * 100000.0 / maxpower) as i64 + rng.gen_range(0.0, 100.0) as i64,
                pattern,
                0,
                i,
                j,
            ));
        }
    }

    let r3 = 5.0 * 1.73205 + 0.1;

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
                let p_0 = P(inp.stage0.0 + 10.0 + c, inp.pos[id].1);
                let p_1 = P(inp.stage0.0 + 10.0, inp.pos[id].1 + b + ten * num as f64);
                let p_2 = P(inp.stage0.0 + 10.0, inp.pos[id].1 - b - ten * num as f64);
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            } else if pattern == 4 {
                let a = inp.pos[id].0 - (inp.stage1.0 - 10.0);
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(inp.stage1.0 - 10.0 - c, inp.pos[id].1);
                let p_1 = P(inp.stage1.0 - 10.0, inp.pos[id].1 - b - ten * num as f64);
                let p_2 = P(inp.stage1.0 - 10.0, inp.pos[id].1 + b + ten * num as f64);
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            } else if pattern == 2 {
                let a = (inp.stage0.1 + 10.0) - inp.pos[id].1;
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(inp.pos[id].0, inp.stage0.1 + 10.0 + c);
                let p_1 = P(inp.pos[id].0 + b + ten * num as f64, inp.stage0.1 + 10.0);
                let p_2 = P(inp.pos[id].0 - b - ten * num as f64, inp.stage0.1 + 10.0);
                let p_3 = p_1 + (p_0 - p_1).rot60();
                let p_4 = p_0 + (p_2 - p_0).rot60();
                ps = vec![p_0, p_1, p_2, p_3, p_4];
            } else if pattern == 8 {
                let a = inp.pos[id].1 - (inp.stage1.1 - 10.0);
                let b = calc_y_to_x(a);
                let c = (a * a + b * b).sqrt() - a;
                let p_0 = P(inp.pos[id].0, inp.stage1.1 - 10.0 - c);
                let p_1 = P(inp.pos[id].0 + b + ten * num as f64, inp.stage1.1 - 10.0);
                let p_2 = P(inp.pos[id].0 - b - ten * num as f64, inp.stage1.1 - 10.0);
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

            if pattern == 2 {
                ps = vec![
                    P(
                        inp.pos[id].0 + (5.00 + ten * num as f64),
                        inp.stage0.1 + 10.0,
                    ),
                    P(
                        inp.pos[id].0 - (5.00 + ten * num as f64),
                        inp.stage0.1 + 10.0,
                    ),
                ];
            } else if pattern == 8 {
                ps = vec![
                    P(
                        inp.pos[id].0 + (5.00 + ten * num as f64),
                        inp.stage1.1 - 10.0,
                    ),
                    P(
                        inp.pos[id].0 - (5.00 + ten * num as f64),
                        inp.stage1.1 - 10.0,
                    ),
                ];
            } else if pattern == 1 {
                ps = vec![
                    P(
                        inp.stage0.0 + 10.0,
                        inp.pos[id].1 + (5.00 + ten * num as f64),
                    ),
                    P(
                        inp.stage0.0 + 10.0,
                        inp.pos[id].1 - (5.00 + ten * num as f64),
                    ),
                ];
            } else if pattern == 4 {
                ps = vec![
                    P(
                        inp.stage1.0 - 10.0,
                        inp.pos[id].1 + (5.00 + ten * num as f64),
                    ),
                    P(
                        inp.stage1.0 - 10.0,
                        inp.pos[id].1 - (5.00 + ten * num as f64),
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
        } else {
            let mut ps = Vec::new();

            if pattern == 2 {
                ps = vec![
                    P(inp.pos[id].0 + (ten * num as f64), inp.stage0.1 + 10.0),
                    P(inp.pos[id].0 - (ten * num as f64), inp.stage0.1 + 10.0),
                ];
            } else if pattern == 8 {
                ps = vec![
                    P(inp.pos[id].0 + (ten * num as f64), inp.stage1.1 - 10.0),
                    P(inp.pos[id].0 - (ten * num as f64), inp.stage1.1 - 10.0),
                ];
            } else if pattern == 1 {
                ps = vec![
                    P(inp.stage0.0 + 10.0, inp.pos[id].1 + (ten * num as f64)),
                    P(inp.stage0.0 + 10.0, inp.pos[id].1 - (ten * num as f64)),
                ];
            } else if pattern == 4 {
                ps = vec![
                    P(inp.stage1.0 - 10.0, inp.pos[id].1 + (ten * num as f64)),
                    P(inp.stage1.0 - 10.0, inp.pos[id].1 - (ten * num as f64)),
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
        }
    }

    let now_n = candidate.len();
    for i in 0..now_n {
        'jloop: for j in i..now_n {
            if (candidate[i] - candidate[j]).abs2() >= 400.0 {
                continue;
            }

            if (candidate[i] - candidate[j]).abs2() < 100.0 - 0.000000001 {
                continue;
            }

            //dbg!(base_num[i], base_num[j], base_pat[i], base_pat[j]);

            let d1 = (candidate[i] - candidate[j]).abs();
            let d2 = (10.0 * 10.0 - d1 * d1 / 4.0).sqrt() + 0.001;

            if candidate[i].0 == candidate[j].0 {
                let next_p = {
                    if candidate[i].0 <= inp.stage0.0 + 15.0 {
                        P(candidate[i].0 + d2, (candidate[i].1 + candidate[j].1) / 2.0)
                    } else {
                        P(candidate[i].0 - d2, (candidate[i].1 + candidate[j].1) / 2.0)
                    }
                };

                if check_all_cand(&inp, &candidate, next_p) {
                    candidate.push(next_p);
                }

                //dbg!((ret_ps[i] - next_p).abs2());
                //dbg!((ret_ps[j] - next_p).abs2());
            }

            if candidate[i].1 == candidate[j].1 {
                let next_p = {
                    if candidate[i].1 <= inp.stage0.1 + 15.0 {
                        P((candidate[i].0 + candidate[j].0) / 2.0, candidate[i].1 + d2)
                    } else {
                        P((candidate[i].0 + candidate[j].0) / 2.0, candidate[i].1 - d2)
                    }
                };
                if check_all_cand(&inp, &candidate, next_p) {
                    candidate.push(next_p);
                }

                //dbg!((ret_ps[i] - next_p).abs2());
                //dbg!((ret_ps[j] - next_p).abs2());
            }
        }
    }

    //dbg!(candidate.len());

    /*
    let add_l = vec![
        0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, r3, 9.0,
    ];

    for add in add_l {
        //y=0
        for x in (inp.stage0.0 as i32 + 10)..(inp.stage1.0 as i32 - 10) {
            let nx = x as f64;
            let ny = inp.stage0.1 + 10.0 + add;

            if !check_all_cand(&inp, &candidate, P(nx, ny)) {
                continue;
            }
            candidate.push(P(nx, ny));
        }

        //x=0
        for y in (inp.stage0.1 as i32 + 10)..(inp.stage1.1 as i32 - 10) {
            let nx = inp.stage0.0 + 10.0 + add;
            let ny = y as f64;

            if !check_all_cand(&inp, &candidate, P(nx, ny)) {
                continue;
            }

            candidate.push(P(nx, ny));
        }

        //y=maxy
        for x in (inp.stage0.0 as i32 + 10)..(inp.stage1.0 as i32 - 10) {
            let nx = x as f64;
            let ny = inp.stage1.1 - 10.0 - add;

            if !check_all_cand(&inp, &candidate, P(nx, ny)) {
                continue;
            }
            candidate.push(P(nx, ny));
        }

        //x=maxx
        for y in (inp.stage0.1 as i32 + 10)..(inp.stage1.1 as i32 - 10) {
            let nx = inp.stage1.0 - 10.0 - add;
            let ny = y as f64;

            if !check_all_cand(&inp, &candidate, P(nx, ny)) {
                continue;
            }

            candidate.push(P(nx, ny));
        }
    }
    */

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
