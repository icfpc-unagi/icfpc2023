#![allow(non_snake_case)]

use icfpc2023::*;
use itertools::Itertools;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::BTreeSet;

fn compute_dist2_from_stage(input: &Input, p: P) -> f64 {
    let stage0 = P(input.stage0.0 + 10.0, input.stage0.1 + 10.0);
    let stage1 = P(input.stage1.0 - 10.0, input.stage1.1 - 10.0);
    if p.0 < stage0.0 {
        if p.1 < stage0.1 {
            (p - stage0).abs2()
        } else if p.1 > stage1.1 {
            (p - P(stage0.0, stage1.1)).abs2()
        } else {
            (stage0.0 - p.0) * (stage0.0 - p.0)
        }
    } else if p.0 > stage1.0 {
        if p.1 < stage0.1 {
            (p - P(stage1.0, stage0.0)).abs2()
        } else if p.1 > stage1.1 {
            (p - stage1).abs2()
        } else {
            (stage1.0 - p.0) * (stage1.0 - p.0)
        }
    } else {
        if p.1 <= stage0.1 {
            (stage0.1 - p.1) * (stage0.1 - p.1)
        } else {
            (stage1.1 - p.1) * (stage1.1 - p.1)
        }
    }
}

fn preprocess(mut input: Input) -> Input {
    if input.n_musicians() * input.n_attendees() < 1000 * 1000 {
        return input;
    }
    eprint!("{} -> ", input.pos.len());
    let mut sum = 0.0;
    for i in 0..input.pos.len() {
        let mut max = 0.0;
        let d = compute_dist2_from_stage(&input, input.pos[i]);
        for &t in &input.tastes[i] {
            max.setmax(t / d);
        }
        sum += max;
    }
    let mut ok = vec![false; input.pos.len()];
    for i in 0..input.pos.len() {
        let mut max = 0.0;
        let d = compute_dist2_from_stage(&input, input.pos[i]);
        for &t in &input.tastes[i] {
            max.setmax(t / d);
        }
        ok[i] = max * input.pos.len() as f64 > sum;
    }
    let mut pos = vec![];
    let mut tastes = vec![];
    for i in 0..input.pos.len() {
        if ok[i] {
            pos.push(input.pos[i]);
            tastes.push(input.tastes[i].clone());
        }
    }
    input.pos = pos;
    input.tastes = tastes;
    eprintln!("{}", input.pos.len());
    input
}

const EPS: f64 = 1e-8;

fn compute_cand(input: &Input) -> Vec<P> {
    const K: usize = 5;
    let mut list = vec![];
    for i in 0..input.pos.len() {
        list.push((compute_dist2_from_stage(input, input.pos[i]), i));
    }
    list.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let stage0 = P(input.stage0.0 + 10.0, input.stage0.1 + 10.0);
    let stage1 = P(input.stage1.0 - 10.0, input.stage1.1 - 10.0);
    let mut cand = vec![];
    for (_, i) in list {
        let p = input.pos[i];
        let mut tmp = vec![];
        if p.0 < stage0.0 {
            tmp.push(P(stage0.0, p.1));
            tmp.push(P(stage0.0 + f64::sqrt(3.0) * 5.0, p.1));
            for k in 1..=K {
                tmp.push(P(stage0.0, p.1 - 10.0 * k as f64));
                tmp.push(P(stage0.0, p.1 + 10.0 * k as f64));
                tmp.push(P(stage0.0, p.1 - 5.0 - EPS - 10.0 * k as f64));
                tmp.push(P(stage0.0, p.1 + 5.0 + EPS + 10.0 * k as f64));
            }
        } else if p.0 > stage1.0 {
            tmp.push(P(stage1.0, p.1));
            tmp.push(P(stage1.0 - f64::sqrt(3.0) * 5.0, p.1));
            for k in 1..=K {
                tmp.push(P(stage1.0, p.1 - 10.0 * k as f64));
                tmp.push(P(stage1.0, p.1 + 10.0 * k as f64));
                tmp.push(P(stage1.0, p.1 - 5.0 - EPS - 10.0 * k as f64));
                tmp.push(P(stage1.0, p.1 + 5.0 + EPS + 10.0 * k as f64));
            }
        } else if p.1 <= stage0.1 {
            tmp.push(P(p.0, stage0.1));
            tmp.push(P(p.0, stage0.1 + f64::sqrt(3.0) * 5.0));
            for k in 1..=K {
                tmp.push(P(p.0 - 10.0 * k as f64, stage0.1));
                tmp.push(P(p.0 + 10.0 * k as f64, stage0.1));
                tmp.push(P(p.0 - 5.0 - EPS - 10.0 * k as f64, stage0.1));
                tmp.push(P(p.0 + 5.0 + EPS + 10.0 * k as f64, stage0.1));
            }
        } else {
            tmp.push(P(p.0, stage1.1));
            tmp.push(P(p.0, stage1.1 - f64::sqrt(3.0) * 5.0));
            for k in 1..=K {
                tmp.push(P(p.0 - 10.0 * k as f64, stage1.1));
                tmp.push(P(p.0 + 10.0 * k as f64, stage1.1));
                tmp.push(P(p.0 - 5.0 - EPS - 10.0 * k as f64, stage1.1));
                tmp.push(P(p.0 + 5.0 + EPS + 10.0 * k as f64, stage1.1));
            }
        }
        for q in tmp {
            if stage0.0 <= q.0 && q.0 <= stage1.0 && stage0.1 <= q.1 && q.1 <= stage1.1 {
                cand.push(((p - q).abs2(), q));
            }
        }
    }
    cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut cand_set = BTreeSet::new();
    for (_, p) in cand {
        cand_set.insert(p);
        if cand_set.len() >= 10000 {
            break;
        }
    }
    for d in 0.. {
        let x = stage0.0 + 10.0 * d as f64;
        if x > stage1.0 {
            break;
        }
        cand_set.insert(P(x, stage0.1));
        cand_set.insert(P(x, stage1.1));
    }
    for d in 0.. {
        let y = stage0.1 + 10.0 * d as f64;
        if y > stage1.1 {
            break;
        }
        cand_set.insert(P(stage0.0, y));
        cand_set.insert(P(stage1.0, y));
    }
    let mut count = 0;
    for w in 0i32.. {
        for d1 in -w..=w {
            for d2 in -w..=w {
                if d1.abs() == w || d2.abs() == w {
                    let x = (stage0.0 + stage1.0) * 0.5 + 10.0 * d1 as f64;
                    let y = (stage0.1 + stage1.1) * 0.5 + 10.0 * d2 as f64;
                    if stage0.0 <= x && x <= stage1.0 && stage0.1 <= y && y <= stage1.1 {
                        cand_set.insert(P(x, y));
                        count += 1;
                    }
                }
            }
        }
        if count >= input.musicians.len() {
            break;
        }
    }
    cand_set.into_iter().collect_vec()
}

fn score1(input: &Input, p: P, inst: usize, a: usize) -> i64 {
    let d = (p - input.pos[a]).abs2();
    (1e6 * input.tastes[a][inst] / d).ceil() as i64
}

struct State {
    block_count: Vec<Vec<i32>>,
    insts: Vec<usize>,
    score: i64,
}

impl State {
    fn new(n: usize, m: usize) -> Self {
        Self {
            block_count: mat![0; n; m],
            insts: vec![!0; n],
            score: 0,
        }
    }
    fn swap_inst(&mut self, input: &Input, cand: &Vec<P>, i1: usize, i2: usize) -> Option<i64> {
        if self.insts[i1] == self.insts[i2] || self.insts[i1] == !0 || self.insts[i2] == !0 {
            None
        } else {
            let k1 = self.insts[i1];
            let k2 = self.insts[i2];
            self.insts.swap(i1, i2);
            let old = self.score;
            for j in 0..input.pos.len() {
                if self.block_count[i1][j] == 0 {
                    self.score -= score1(input, cand[i1], k1, j);
                    self.score += score1(input, cand[i1], k2, j);
                }
                if self.block_count[i2][j] == 0 {
                    self.score -= score1(input, cand[i2], k2, j);
                    self.score += score1(input, cand[i2], k1, j);
                }
            }
            Some(self.score - old)
        }
    }
    fn add(
        &mut self,
        input: &Input,
        cand: &Vec<P>,
        block: &Vec<Vec<(usize, usize)>>,
        conflict: &Vec<Vec<usize>>,
        i: usize,
        k: usize,
    ) -> Option<i64> {
        if self.insts[i] != !0 || conflict[i].iter().any(|&j| self.insts[j] != !0) {
            None
        } else {
            let old = self.score;
            self.insts[i] = k;
            for &(i2, j) in &block[i] {
                if self.block_count[i2][j] == 0 && self.insts[i2] != !0 {
                    self.score -= score1(input, cand[i2], self.insts[i2], j);
                }
                self.block_count[i2][j] += 1;
            }
            for j in 0..input.pos.len() {
                if self.block_count[i][j] == 0 {
                    self.score += score1(input, cand[i], k, j);
                }
            }
            Some(self.score - old)
        }
    }
    fn remove(
        &mut self,
        input: &Input,
        cand: &Vec<P>,
        block: &Vec<Vec<(usize, usize)>>,
        i: usize,
    ) -> Option<i64> {
        if self.insts[i] == !0 {
            None
        } else {
            let old = self.score;
            let k = self.insts[i];
            self.insts[i] = !0;
            for &(i2, j) in &block[i] {
                self.block_count[i2][j] -= 1;
                if self.block_count[i2][j] == 0 && self.insts[i2] != !0 {
                    self.score += score1(input, cand[i2], self.insts[i2], j);
                }
            }
            for j in 0..input.pos.len() {
                if self.block_count[i][j] == 0 {
                    self.score -= score1(input, cand[i], k, j);
                }
            }
            Some(self.score - old)
        }
    }
    fn mov(
        &mut self,
        input: &Input,
        cand: &Vec<P>,
        block: &Vec<Vec<(usize, usize)>>,
        conflict: &Vec<Vec<usize>>,
        from: usize,
        to: usize,
    ) -> Option<i64> {
        if self.insts[from] == !0 || self.insts[to] != !0 {
            None
        } else {
            let k = self.insts[from];
            let old = self.score;
            self.remove(input, cand, block, from).unwrap();
            if self.add(input, cand, block, conflict, to, k).is_none() {
                self.add(input, cand, block, conflict, from, k);
                None
            } else {
                Some(self.score - old)
            }
        }
    }
}

fn main() {
    let input = preprocess(read_input());
    let cand = compute_cand(&input);
    let mut near = vec![vec![]; cand.len()];
    let mut conflict = vec![vec![]; cand.len()];
    for i in 0..cand.len() {
        let mut tmp = vec![];
        for j in 0..cand.len() {
            let d2 = (cand[i] - cand[j]).abs2();
            if d2 < 100.0 {
                conflict[i].push(j);
            }
            tmp.push((d2, j));
        }
        tmp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        tmp.truncate(20);
        near[i] = tmp;
    }
    let block = (0..cand.len())
        .into_par_iter()
        .map(|i| {
            let mut tmp = vec![];
            for j in 0..cand.len() {
                if (cand[i] - cand[j]).abs2() >= 100.0 {
                    for k in 0..input.pos.len() {
                        if P::dist2_sp((cand[j], input.pos[k]), cand[i]) <= 25.0 {
                            tmp.push((j, k));
                        }
                    }
                }
            }
            tmp
        })
        .collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    let mut state = State::new(cand.len(), input.n_attendees());
    for i in 0..input.musicians.len() {
        loop {
            let j = rng.gen_range(0, cand.len());
            if let Some(_) = state.add(&input, &cand, &block, &conflict, j, input.musicians[i]) {
                break;
            }
        }
    }
    let mut best = state.insts.clone();
    let mut best_score = state.score;
    eprintln!("{:.3}: {}", get_time(), best_score);
    const T0: f64 = 1e-2;
    const T1: f64 = 1e-6;
    let TL: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(600.0);
    let stime = get_time();
    for iter in 0.. {
        let i1 = rng.gen_range(0, cand.len());
        if state.insts[i1] == !0 {
            continue;
        }
        let t = (get_time() - stime) / TL;
        if t >= 1.0 {
            eprintln!("Iter = {}", iter);
            break;
        }
        let T = T0.powf(1.0 - t) * T1.powf(t);
        let i2 = if rng.gen_bool(0.1) {
            rng.gen_range(0, cand.len())
        } else {
            near[i1].choose(&mut rng).unwrap().1
        };
        if state.insts[i2] == !0 {
            if let Some(diff) = state.mov(&input, &cand, &block, &conflict, i1, i2) {
                if diff >= 0
                    || rng.gen_bool((diff as f64 / state.score.abs().max(1) as f64 / T).exp())
                {
                } else {
                    state.mov(&input, &cand, &block, &conflict, i2, i1).unwrap();
                }
            }
        } else {
            if let Some(diff) = state.swap_inst(&input, &cand, i1, i2) {
                if diff >= 0
                    || rng.gen_bool((diff as f64 / state.score.abs().max(1) as f64 / T).exp())
                {
                } else {
                    state.swap_inst(&input, &cand, i1, i2).unwrap();
                }
            }
        }
        if best_score.setmax(state.score) {
            best = state.insts.clone();
            eprintln!("{:.3}: {}", get_time(), best_score);
        }
    }
    let mut ms = vec![vec![]; input.n_instruments()];
    for i in 0..input.musicians.len() {
        ms[input.musicians[i]].push(i);
    }
    let mut out = vec![P(0.0, 0.0); input.musicians.len()];
    for i in 0..best.len() {
        if best[i] != !0 {
            let j = ms[best[i]].pop().unwrap();
            out[j] = cand[i];
        }
    }
    write_output(&out);
}
