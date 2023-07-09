#![allow(unused_imports)]
#![allow(unused)] // とりあえず警告でないようしました。完成したらこの行（←）を消してwarning出ないか見ても良いかも。
use std::{collections::BinaryHeap, net::SocketAddr, ops::Sub};

use aead::NewAead;

use crate::{is_blocked, is_blocked_by_circle, Input, P};

#[allow(non_upper_case_globals)]
const ng_num: usize = 9999999;

pub fn get_candidate_tree(
    inp: &Input,
) -> (
    Vec<P>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<bool>,
    Vec<Vec<i64>>,
    Vec<i64>,
    Vec<usize>,
    usize,
) {
    let mut ret_ps = vec![];
    let mut parent = vec![];
    let mut child = vec![];
    let mut connect = vec![];
    let mut valid = vec![];

    let mut points = vec![];
    let mut max_points = vec![];
    let mut pair = vec![];

    let mut base_num = vec![];
    let mut base_pat = vec![];

    let mut dist_ar = vec![];
    for i in 0..inp.pos.len() {
        let dist = get_stage_diff(inp.pos[i], inp.stage0, inp.stage1) as i64;
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

        if pattern & (pattern - 1) != 0 {
            continue;
        }

        dist_ar.push(dist);
    }
    dist_ar.sort();

    let target_num = 200;
    let target_dist = {
        if dist_ar.len() < target_num - 1 {
            99999999
        } else {
            dist_ar[target_num]
        }
    };

    dbg!(target_dist);
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

        //dbg!(pattern);

        if pattern & (pattern - 1) != 0 {
            continue;
        }

        let d2 = get_stage_diff(inp.pos[i], inp.stage0, inp.stage1);

        //dbg!(d2);

        let mut max_num = 4;

        if d2 > target_dist as f64 {
            max_num = 0;
        }

        let mut ps = vec![];
        let mut ids = vec![];

        for pat in 0..3 {
            for num in 0..max_num {
                let pre_ps = ps.clone();
                let pre_id = ids.clone();
                ps = vec![];
                ids = vec![];

                //5つ配置
                if pat == 0 {
                    if pattern == 1 {
                        let a = (inp.stage0.0 + 10.0) - inp.pos[i].0;
                        let b = calc_y_to_x(a);
                        let c = (a * a + b * b).sqrt() - a;
                        let p_0 = P(inp.stage0.0 + 10.0 + c, inp.pos[i].1);
                        let p_1 = P(inp.stage0.0 + 10.0, inp.pos[i].1 + b + 10.0 * num as f64);
                        let p_2 = P(inp.stage0.0 + 10.0, inp.pos[i].1 - b - 10.0 * num as f64);
                        let p_3 = p_1 + (p_0 - p_1).rot60();
                        let p_4 = p_0 + (p_2 - p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    } else if pattern == 4 {
                        let a = inp.pos[i].0 - (inp.stage1.0 - 10.0);
                        let b = calc_y_to_x(a);
                        let c = (a * a + b * b).sqrt() - a;
                        let p_0 = P(inp.stage1.0 - 10.0 - c, inp.pos[i].1);
                        let p_1 = P(inp.stage1.0 - 10.0, inp.pos[i].1 - b - 10.0 * num as f64);
                        let p_2 = P(inp.stage1.0 - 10.0, inp.pos[i].1 + b + 10.0 * num as f64);
                        let p_3 = p_1 + (p_0 - p_1).rot60();
                        let p_4 = p_0 + (p_2 - p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    } else if pattern == 2 {
                        let a = (inp.stage0.1 + 10.0) - inp.pos[i].1;
                        let b = calc_y_to_x(a);
                        let c = (a * a + b * b).sqrt() - a;
                        let p_0 = P(inp.pos[i].0, inp.stage0.1 + 10.0 + c);
                        let p_1 = P(inp.pos[i].0 + b + 10.0 * num as f64, inp.stage0.1 + 10.0);
                        let p_2 = P(inp.pos[i].0 - b - 10.0 * num as f64, inp.stage0.1 + 10.0);
                        let p_3 = p_1 + (p_0 - p_1).rot60();
                        let p_4 = p_0 + (p_2 - p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    } else if pattern == 8 {
                        let a = inp.pos[i].1 - (inp.stage1.1 - 10.0);
                        let b = calc_y_to_x(a);
                        let c = (a * a + b * b).sqrt() - a;
                        let p_0 = P(inp.pos[i].0, inp.stage1.1 - 10.0 - c);
                        let p_1 = P(inp.pos[i].0 + b + 10.0 * num as f64, inp.stage1.1 - 10.0);
                        let p_2 = P(inp.pos[i].0 - b - 10.0 * num as f64, inp.stage1.1 - 10.0);
                        let p_3 = p_1 + (p_0 - p_1).rot60();
                        let p_4 = p_0 + (p_2 - p_0).rot60();
                        ps = vec![p_0, p_1, p_2, p_3, p_4];
                    }

                    if num == 0 {
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[1]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        base_num.push(i);
                        base_pat.push(pat);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[2]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        base_num.push(i);
                        base_pat.push(pat);

                        pair.push(ids[1]);
                        pair.push(ids[0]);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[0]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, ids[0], ids[2]);
                        set_oyako(&mut parent, &mut child, ids[1], ids[2]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[3]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, ids[0], ids[3]);
                        set_oyako(&mut parent, &mut child, ids[2], ids[3]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[4]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, ids[1], ids[4]);
                        set_oyako(&mut parent, &mut child, ids[2], ids[4]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);
                    } else {
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[1]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[0], ids[0]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[2]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[1], ids[1]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);
                    }
                } else if pat == 1 {
                    //横置き
                    let r3 = 5.0 * 1.73205 + 0.1;
                    if pattern == 2 {
                        ps = vec![
                            P(inp.pos[i].0 + (10.0 * num as f64), inp.stage0.1 + 10.0),
                            P(inp.pos[i].0 - (10.0 * num as f64), inp.stage0.1 + 10.0),
                            P(
                                inp.pos[i].0 + (10.0 * num as f64) - 5.0,
                                inp.stage0.1 + 10.0 + r3,
                            ),
                            P(
                                inp.pos[i].0 - (10.0 * num as f64) + 5.0,
                                inp.stage0.1 + 10.0 + r3,
                            ),
                        ];
                    } else if pattern == 8 {
                        ps = vec![
                            P(inp.pos[i].0 + (10.0 * num as f64), inp.stage1.1 - 10.0),
                            P(inp.pos[i].0 - (10.0 * num as f64), inp.stage1.1 - 10.0),
                            P(
                                inp.pos[i].0 + (10.0 * num as f64) - 5.0,
                                inp.stage1.1 - 10.0 - r3,
                            ),
                            P(
                                inp.pos[i].0 - (10.0 * num as f64) + 5.0,
                                inp.stage1.1 - 10.0 - r3,
                            ),
                        ];
                    } else if pattern == 1 {
                        ps = vec![
                            P(inp.stage0.0 + 10.0, inp.pos[i].1 + (10.0 * num as f64)),
                            P(inp.stage0.0 + 10.0, inp.pos[i].1 - (10.0 * num as f64)),
                            P(
                                inp.stage1.0 + 10.0 + r3,
                                inp.pos[i].1 - (10.0 * num as f64) + 5.0,
                            ),
                            P(
                                inp.stage1.0 + 10.0 + r3,
                                inp.pos[i].1 + (10.0 * num as f64) - 5.0,
                            ),
                        ];
                    } else if pattern == 4 {
                        ps = vec![
                            P(inp.stage1.0 - 10.0, inp.pos[i].1 + (10.0 * num as f64)),
                            P(inp.stage1.0 - 10.0, inp.pos[i].1 - (10.0 * num as f64)),
                            P(
                                inp.stage1.1 - 10.0 - r3,
                                inp.pos[i].1 - (10.0 * num as f64) + 5.0,
                            ),
                            P(
                                inp.stage1.1 - 10.0 - r3,
                                inp.pos[i].1 + (10.0 * num as f64) - 5.0,
                            ),
                        ];
                    }

                    if num == 0 {
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[0]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        base_num.push(i);
                        base_pat.push(pat);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[1]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        base_num.push(i);
                        base_pat.push(pat);

                        pair.push(ids[1]);
                        pair.push(ids[0]);

                        /*
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[2]);
                        parent.push(vec![]);
                        child.push(vec![]);

                        set_oyako(&mut parent, &mut child, ids[0], ids[2]);
                        set_oyako(&mut parent, &mut child, ids[1], ids[2]);
                        */
                    } else {
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[0]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[0], ids[0]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[1]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[1], ids[1]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        /*
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[2]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[0], ids[2]);
                        set_oyako(&mut parent, &mut child, ids[0], ids[2]);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[3]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[1], ids[3]);
                        set_oyako(&mut parent, &mut child, ids[1], ids[3]);
                        */
                    }
                } else if pat == 2 {
                    let r3 = 5.0 * 1.73205 + 0.1;
                    if pattern == 2 {
                        ps = vec![
                            P(inp.pos[i].0 + (10.0 * num as f64), inp.stage0.1 + 10.0),
                            P(inp.pos[i].0 - (10.0 * num as f64), inp.stage0.1 + 10.0),
                            P(
                                inp.pos[i].0 + (10.0 * num as f64) - 5.0,
                                inp.stage0.1 + 10.0 + r3,
                            ),
                            P(
                                inp.pos[i].0 - (10.0 * num as f64) + 5.0,
                                inp.stage0.1 + 10.0 + r3,
                            ),
                        ];
                    } else if pattern == 8 {
                        ps = vec![
                            P(inp.pos[i].0 + (10.0 * num as f64), inp.stage1.1 - 10.0),
                            P(inp.pos[i].0 - (10.0 * num as f64), inp.stage1.1 - 10.0),
                            P(
                                inp.pos[i].0 + (10.0 * num as f64) - 5.0,
                                inp.stage1.1 - 10.0 - r3,
                            ),
                            P(
                                inp.pos[i].0 - (10.0 * num as f64) + 5.0,
                                inp.stage1.1 - 10.0 - r3,
                            ),
                        ];
                    } else if pattern == 1 {
                        ps = vec![
                            P(inp.stage0.0 + 10.0, inp.pos[i].1 + (10.0 * num as f64)),
                            P(inp.stage0.0 + 10.0, inp.pos[i].1 - (10.0 * num as f64)),
                            P(
                                inp.stage0.0 + 10.0 + r3,
                                inp.pos[i].1 + (10.0 * num as f64) - 5.0,
                            ),
                            P(
                                inp.stage0.0 + 10.0 + r3,
                                inp.pos[i].1 - (10.0 * num as f64) + 5.0,
                            ),
                        ];
                    } else if pattern == 4 {
                        ps = vec![
                            P(inp.stage1.0 - 10.0, inp.pos[i].1 + (10.0 * num as f64)),
                            P(inp.stage1.0 - 10.0, inp.pos[i].1 - (10.0 * num as f64)),
                            P(
                                inp.stage1.0 - 10.0 - r3,
                                inp.pos[i].1 + (10.0 * num as f64) - 5.0,
                            ),
                            P(
                                inp.stage1.0 - 10.0 - r3,
                                inp.pos[i].1 - (10.0 * num as f64) + 5.0,
                            ),
                        ];
                    }

                    if num == 0 {
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[0]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);
                    } else {
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[0]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[0], ids[0]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        let pre = pre_id.len() - 1;
                        ids.push(ret_ps.len());
                        ret_ps.push(ps[1]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[pre], ids[1]);
                        pair.push(ng_num);
                        base_num.push(i);
                        base_pat.push(pat);

                        /*

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[2]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[0], ids[2]);
                        set_oyako(&mut parent, &mut child, ids[0], ids[2]);

                        ids.push(ret_ps.len());
                        ret_ps.push(ps[3]);
                        parent.push(vec![]);
                        child.push(vec![]);
                        set_oyako(&mut parent, &mut child, pre_id[pre], ids[3]);
                        set_oyako(&mut parent, &mut child, ids[1], ids[3]);
                        */
                    }
                }
            }
        }
    }

    //四隅の配置

    for pattern in 0..4 {
        let mut ps = vec![];
        let mut ids = vec![];

        for pat in 0..3 {
            for num in 0..6 {
                let pre_ps = ps.clone();
                let pre_id = ids.clone();
                ps = vec![];
                ids = vec![];

                if pat == 0 {
                    ps = vec![
                        P(inp.stage0.0 + 10.0 + 10.0 * num as f64, inp.stage0.1 + 10.0),
                        P(inp.stage0.0 + 10.0, inp.stage0.1 + 10.0 + 10.0 * num as f64),
                    ]
                } else if pat == 1 {
                    ps = vec![
                        P(inp.stage0.0 + 10.0 + 10.0 * num as f64, inp.stage1.1 - 10.0),
                        P(inp.stage0.0 + 10.0, inp.stage1.1 - 10.0 - 10.0 * num as f64),
                    ]
                } else if pat == 2 {
                    ps = vec![
                        P(inp.stage1.0 - 10.0 - 10.0 * num as f64, inp.stage0.1 + 10.0),
                        P(inp.stage1.0 - 10.0, inp.stage0.1 + 10.0 + 10.0 * num as f64),
                    ]
                } else if pat == 3 {
                    ps = vec![
                        P(inp.stage1.0 - 10.0 - 10.0 * num as f64, inp.stage1.1 - 10.0),
                        P(inp.stage1.0 - 10.0, inp.stage1.1 - 10.0 - 10.0 * num as f64),
                    ]
                }

                if num == 0 {
                    ids.push(ret_ps.len());
                    ret_ps.push(ps[0]);
                    parent.push(vec![]);
                    child.push(vec![]);
                    pair.push(ng_num);

                    base_num.push(10000 + pat);
                    base_pat.push(0);
                } else {
                    ids.push(ret_ps.len());
                    ret_ps.push(ps[0]);
                    parent.push(vec![]);
                    child.push(vec![]);
                    pair.push(ng_num);
                    base_num.push(10000 + pat);
                    base_pat.push(0);

                    set_oyako(&mut parent, &mut child, ids[0], pre_id[0]);

                    ids.push(ret_ps.len());
                    ret_ps.push(ps[1]);
                    parent.push(vec![]);
                    child.push(vec![]);
                    pair.push(ng_num);
                    base_num.push(10000 + pat);
                    base_pat.push(0);

                    set_oyako(&mut parent, &mut child, ids[1], pre_id[pre_id.len() - 1]);
                }
            }
        }
    }

    dbg!(ret_ps.len());
    //dbg!(pair.len());

    valid = vec![true; ret_ps.len()];

    //valid1回目
    for i in 0..ret_ps.len() {
        if ret_ps[i].0 < inp.stage0.0 + 10.0
            || ret_ps[i].1 < inp.stage0.1 + 10.0
            || ret_ps[i].0 > inp.stage1.0 - 10.0
            || ret_ps[i].1 > inp.stage1.1 - 10.0
        {
            valid[i] = false;
        }
    }

    //二段目配置
    let now_n = ret_ps.len();
    for i in 0..now_n {
        if !valid[i] {
            continue;
        }
        'jloop: for j in i..now_n {
            if !valid[j] {
                continue;
            }
            if parent[i].len() >= 1 && parent[j].len() >= 1 && parent[i][0] == parent[j][0] {
                continue;
            }

            if (ret_ps[i] - ret_ps[j]).abs2() >= 400.0 {
                continue;
            }

            if (ret_ps[i] - ret_ps[j]).abs2() < 100.0 - 0.000000001 {
                continue;
            }

            //dbg!(base_num[i], base_num[j], base_pat[i], base_pat[j]);

            if base_num[i] == base_num[j] && base_pat[i] != base_pat[j] {
                //dbg!("find!")
                continue;
            }

            for t1 in 0..parent[i].len() {
                if (ret_ps[parent[i][t1]] - ret_ps[j]).abs2() < 100.0 - 0.000000001 {
                    continue 'jloop;
                }
            }

            for t1 in 0..parent[j].len() {
                if (ret_ps[parent[j][t1]] - ret_ps[i]).abs2() < 100.0 - 0.000000001 {
                    continue 'jloop;
                }
            }

            let d1 = (ret_ps[i] - ret_ps[j]).abs();
            let d2 = (10.0 * 10.0 - d1 * d1 / 4.0).sqrt() + 0.001;
            if ret_ps[i].0 == ret_ps[j].0 {
                let next_p = {
                    if ret_ps[i].0 <= inp.stage0.0 + 15.0 {
                        P(ret_ps[i].0 + d2, (ret_ps[i].1 + ret_ps[j].1) / 2.0)
                    } else {
                        P(ret_ps[i].0 - d2, (ret_ps[i].1 + ret_ps[j].1) / 2.0)
                    }
                };
                let now = ret_ps.len();
                ret_ps.push(next_p);
                parent.push(vec![]);
                child.push(vec![]);
                base_num.push(100000);
                base_pat.push(0);

                set_oyako(&mut parent, &mut child, i, now);
                set_oyako(&mut parent, &mut child, j, now);
                pair.push(ng_num);

                //dbg!((ret_ps[i] - next_p).abs2());
                //dbg!((ret_ps[j] - next_p).abs2());
            }

            if ret_ps[i].1 == ret_ps[j].1 {
                let next_p = {
                    if ret_ps[i].1 <= inp.stage0.1 + 15.0 {
                        P((ret_ps[i].0 + ret_ps[j].0) / 2.0, ret_ps[i].1 + d2)
                    } else {
                        P((ret_ps[i].0 + ret_ps[j].0) / 2.0, ret_ps[i].1 - d2)
                    }
                };
                let now = ret_ps.len();
                ret_ps.push(next_p);
                parent.push(vec![]);
                child.push(vec![]);
                set_oyako(&mut parent, &mut child, i, now);
                set_oyako(&mut parent, &mut child, j, now);
                pair.push(ng_num);
                base_num.push(100000);
                base_pat.push(0);

                //dbg!((ret_ps[i] - next_p).abs2());
                //dbg!((ret_ps[j] - next_p).abs2());
            }
        }
    }

    let mut add_cnt = 0;

    let xmax = (inp.stage1.0 - inp.stage0.0) / 10.0;
    let ymax = (inp.stage1.1 - inp.stage0.1) / 10.0;

    let mut center_parent = ng_num;

    let mut pre_parent = 9999999;
    for i in 4..(xmax as usize - 3) {
        for j in 4..(ymax as usize - 3) {
            if add_cnt < inp.musicians.len() {
                let p2 = P(
                    inp.stage0.0 + (i as f64 * 10.0),
                    inp.stage0.1 + (j as f64 * 10.0),
                );
                ret_ps.push(p2);
                parent.push(vec![]);
                child.push(vec![]);
                pair.push(ng_num);
                base_num.push(100000);
                base_pat.push(0);
                add_cnt += 1;

                if pre_parent != 9999999 {
                    set_oyako(&mut parent, &mut child, pre_parent, ret_ps.len() - 1);
                } else {
                    center_parent = ret_ps.len() - 1;
                }
                pre_parent = ret_ps.len() - 1;
            }
        }
    }

    dbg!(ret_ps.len());
    //dbg!(pair.len());

    connect = vec![vec![]; ret_ps.len()];
    valid = vec![true; ret_ps.len()];

    for i in 0..ret_ps.len() {
        if ret_ps[i].0 < inp.stage0.0 + 10.0
            || ret_ps[i].1 < inp.stage0.1 + 10.0
            || ret_ps[i].0 > inp.stage1.0 - 10.0
            || ret_ps[i].1 > inp.stage1.1 - 10.0
        {
            valid[i] = false;
        }
    }

    for i in 0..ret_ps.len() {
        if !valid[i] {
            points.push(vec![]);
            max_points.push(-999999999999);
            continue;
        }

        let mut point_ar = vec![0; inp.tastes[0].len()];

        'jloop: for j in 0..inp.pos.len() {
            if ret_ps[i].0 > inp.stage0.0 + 19.9 && inp.pos[j].0 <= inp.stage0.0 {
                continue;
            }
            if ret_ps[i].0 < inp.stage1.0 - 19.9 && inp.pos[j].0 >= inp.stage1.0 {
                continue;
            }

            if ret_ps[i].1 > inp.stage0.1 + 19.9 && inp.pos[j].1 <= inp.stage0.1 {
                continue;
            }
            if ret_ps[i].1 < inp.stage1.1 - 19.9 && inp.pos[j].1 >= inp.stage1.1 {
                continue;
            }

            for par_i in 0..parent[i].len() {
                let par = parent[i][par_i];
                if is_blocked(ret_ps[i], inp.pos[j], ret_ps[par]) {
                    continue 'jloop;
                }
            }

            //TODO: 仮想的な横ブロックによる除外入れる？要検討　入れないと楽観する
            let mut test_block = vec![];
            if ret_ps[i].0 < inp.stage0.0 + 19.9 || ret_ps[i].0 > inp.stage1.0 - 19.9 {
                if ret_ps[i].1 >= inp.stage0.1 + 10.0 {
                    test_block.push(P(ret_ps[i].0, ret_ps[i].1 - 10.0));
                }
                if ret_ps[i].1 <= inp.stage1.1 - 10.0 {
                    test_block.push(P(ret_ps[i].0, ret_ps[i].1 + 10.0));
                }
            }
            if ret_ps[i].1 < inp.stage0.1 + 19.9 || ret_ps[i].1 > inp.stage1.1 - 19.9 {
                if ret_ps[i].0 >= inp.stage0.0 + 10.0 {
                    test_block.push(P(ret_ps[i].0 - 10.0, ret_ps[i].1));
                }
                if ret_ps[i].0 <= inp.stage1.0 - 10.0 {
                    test_block.push(P(ret_ps[i].0 + 10.0, ret_ps[i].1));
                }
            }

            for t_i in 0..test_block.len() {
                if is_blocked(ret_ps[i], inp.pos[j], test_block[t_i]) {
                    continue 'jloop;
                }
            }

            let d = (ret_ps[i] - inp.pos[j]).abs2();

            for k in 0..inp.tastes[j].len() {
                point_ar[k] += (1000000.0 * inp.tastes[j][k] / d) as i64;
            }
        }
        let mut ma = -99999999999;
        for j in 0..point_ar.len() {
            if ma < point_ar[j] {
                ma = point_ar[j];
            }
        }
        points.push(point_ar);
        max_points.push(ma);
    }

    for i in 0..ret_ps.len() {
        if !valid[i] {
            continue;
        }
        for j in i + 1..ret_ps.len() {
            if !valid[j] {
                continue;
            }
            let diff = ret_ps[i] - ret_ps[j];
            if diff.abs2() < 100.0 - 0.0001 {
                connect[i].push(j);
                connect[j].push(i);
            }
        }
    }

    (
        ret_ps,
        parent,
        child,
        connect,
        valid,
        points,
        max_points,
        pair,
        center_parent,
    )
}

fn set_oyako(parent: &mut Vec<Vec<usize>>, child: &mut Vec<Vec<usize>>, p: usize, c: usize) {
    child[p].push(c);
    parent[c].push(p);
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
