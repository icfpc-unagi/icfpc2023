#![allow(unused_imports)]
/*
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::{NewAead, consts::P2};
use rand::Rng;

use crate::{Input, P, is_blocked_by_circle};

#[derive(Debug, Clone)]
struct Beam {
    score: i64,
    left: f64,
    ps: Vec<P>,
}

use std::cmp::Ordering;

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Beam {}

impl PartialOrd for Beam {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Beam {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

pub fn get_candidate_line(inp: &Input, pattern: usize) -> Vec<P> {
    let mut candidate = Vec::new();
    let maxheap = 500;
    let heaps: Vec<BinaryHeap<Beam>> = vec![BinaryHeap::new(); maxheap]; //Vec<BinaryHeap<beam>>

    let mut first_state = Beam(score: 0, left: 0.0, ps: vec![]);
    let mut best_beam = vec![first_state; maxheap];
    let music_n = inp.tastes[0].len();

    best_beam[0].push(first_state);

    let stage_size = inp.stage1 - inp.stage0;
    let mut pos = inp.pos.clone();

    for i in 0..pos.len() {
        pos[i] = change_p_pos(pos[i], pattern, inp.stage0, inp.stage1);
    }


    let mut pt = vec![0; pos.len()];

    for i in 0..pos.len() {
        for j in 0..music_n {
            if pt[i] < (inp.tastes[i][j] * 1000000.0) as i64{
                pt[i] = (inp.tastes[i][j] * 1000000.0) as i64;
            }
        }
    }

    let mut rng = rand::thread_rng();

    loop {
        for i in 0..maxheap {
            if heaps[i].is_empty() {
                continue;
            }

            let now = heaps[i].pop().unwrap();

            //ランダムな距離進む系
            for tt in 0..20 {
                let next_left = {
                    if tt == 0{
                        now.left + 10.0
                    }
                    else{
                        now.left + 10.0 + rng.gen_range(0.001, 9.999)
                    }
                };
                if next_left - now.left < 10.0{
                    next_left += 0.0000000001;
                }
                let half = (next_left - now.left) / 2.0;
                let up = (100.0 as f64 - half * half).sqrt() + 0.00001;

                let p1 = P(next_left, 10.0);
                let p2 = P(now.left + half, 10.0 + up);

                let mut next_beam = now.clone();
                next_beam.ps.push(p2);
                next_beam.ps.push(p1);

                let mut tp = vec![];
                if now.ps.len() != 0{
                    tp.push(now.ps[now.ps.len() - 1]);
                }
                next_beam.score += get_point(&p1, &tp, &pos, &pt);
                tp.push(p1.clone());
                next_beam.score += get_point(&p2, &tp, &pos, &pt);

                let a = next_beam.ps.len();
                heaps[a].push(next_beam);
            }
        }
    }

    return candidate;
}


fn get_point(p: &P, ps: &Vec<P>, pos: &Vec<P>, pt: &Vec<i64>) -> i64{
    let mut ans = 0;
    'l: for i in 0..pos.len() {
        let p2 = pos[i];
        if p.1 >= 0.0 {
            continue;
        }
        for q in ps {
            if is_blocked_by_circle(*p, p2, (q.clone(), 5.0)){
                continue 'l;
            }
        }
        let d = (*p - p2).abs();
        ans += (pt[i] as f64/(d*d)) as i64;
    }
    ans
}


fn change_p_pos(p: P, pattern: usize, stage0: P, stage1: P) -> p{
    let stage_size = stage1 - stage0;
    let mut ret = p.clone();
    ret = ret - stage0;
    if pattern % 2 == 1 {
        ret = P(stage_size.0 - ret.0, ret.1);
    }
    if (pattern / 2) % 2 == 1 {
        ret = P(ret.0, stage_size.1 - ret.1);
    }
    ret
}


fn back_p_pos(p: P, pattern: usize, stage0: P, stage1: P) -> p{
    let stage_size = stage1 - stage0;
    let mut ret = p.clone();


    if (pattern / 2) % 2 == 1 {
        ret = P(ret.0, stage_size.1 - ret.1);
    }
    if pattern % 2 == 1 {
        ret = P(stage_size.0 - ret.0, ret.1);
    }
    ret = ret + stage0;
    ret
}
*/
