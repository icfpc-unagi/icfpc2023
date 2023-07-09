#![allow(non_snake_case)]

use icfpc2023::{candidate::get_all_candidate, mcf::weighted_matching_with_capacity, *};
use rand::prelude::*;
use rayon::prelude::*;

// ステージ外の点について、一番近いミュージシャンまでの距離の二乗
pub fn compute_dist2_from_stage(input: &Input, p: P) -> f64 {
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

// 環境変数 PREPROCESS=1 を設定するとステージから遠い客を無視する
// デフォルトは何もしない
fn preprocess(mut input: Input) -> Input {
    if std::env::var("PREP").unwrap_or(String::new()).len() == 0
        || input.n_musicians() * input.n_attendees() < 1000 * 1000
    {
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

// 候補点を追加する
fn add_cand(input: &Input, cand_list: &mut Vec<P>) {
    let stage0 = P(input.stage0.0 + 10.0, input.stage0.1 + 10.0);
    let stage1 = P(input.stage1.0 - 10.0, input.stage1.1 - 10.0);
    if std::env::var("CAND_CHOKUDAI")
        .unwrap_or("1".to_owned())
        .len()
        > 0
    {
        cand_list.extend(get_all_candidate(input));
    }
    // 内側に十分な量を追加しておく
    let mut count = 0;
    for w in 0i32.. {
        for d1 in -w..=w {
            for d2 in -w..=w {
                if d1.abs() == w || d2.abs() == w {
                    let x = (stage0.0 + stage1.0) * 0.5 + 10.0 * d1 as f64;
                    let y = (stage0.1 + stage1.1) * 0.5 + 10.0 * d2 as f64;
                    if stage0.0 <= x && x <= stage1.0 && stage0.1 <= y && y <= stage1.1 {
                        cand_list.push(P(x, y));
                        count += 1;
                    }
                }
            }
        }
        if count >= input.musicians.len() {
            break;
        }
    }
    cand_list.sort();
    cand_list.dedup();
    eprintln!("#cand = {}", cand_list.len());
}

// ceilは取らない
fn score1(input: &Input, p: P, inst: usize, a: usize) -> f64 {
    let d = (p - input.pos[a]).abs2();
    1e6 * input.tastes[a][inst] / d
}

struct State {
    /// [候補点][客]がブロックされた回数
    block_count: Vec<Vec<i32>>,
    /// [候補点]で演奏する音楽家
    musicians: Vec<usize>,
    /// [音楽家]が演奏する候補点番号
    to: Vec<usize>,
    /// [音楽家]のtastes/d^2の和
    coef_musicians: Vec<f64>,
    /// [音楽家]と同じ楽器について、1/dの和
    closeness: Vec<f64>,
    score: f64,
}

impl State {
    /// 指定された解 to の状態にする
    fn initialize(
        input: &Input,
        cand: &Vec<P>,
        block: &Vec<Vec<(usize, usize)>>,
        to: &Vec<usize>,
    ) -> Self {
        let mut block_count = mat![0; cand.len(); input.n_attendees()];
        let mut musicians = vec![!0; cand.len()];
        let to = to.clone();
        let mut coef_musicians = vec![0.0; input.n_musicians()];
        let mut closeness = vec![0.0; input.n_musicians()];
        let mut score = 0.0;
        for i in 0..cand.len() {
            for a in 0..input.n_attendees() {
                for &p in &input.pillars {
                    if is_blocked_by_circle(cand[i], input.pos[a], p) {
                        block_count[i][a] += 1;
                    }
                }
            }
        }
        for i in 0..input.musicians.len() {
            musicians[to[i]] = i;
            for &(p, j) in &block[to[i]] {
                block_count[p][j] += 1;
            }
        }
        for i in 0..input.musicians.len() {
            for j in 0..input.n_attendees() {
                if block_count[to[i]][j] == 0 {
                    coef_musicians[i] += score1(input, cand[to[i]], input.musicians[i], j);
                }
            }
            if input.version != Version::One {
                for &j in &input.inst_musicians[input.musicians[i]] {
                    if j != i {
                        closeness[i] += 1.0 / (cand[to[i]] - cand[to[j]]).abs();
                    }
                }
            }
            score += (1.0 + closeness[i]) * coef_musicians[i];
        }
        State {
            block_count,
            closeness,
            coef_musicians,
            musicians,
            to,
            score,
        }
    }
    /// 指定された音楽家のclosenessとスコアへの影響を取り除き、coef_musiciansを0にする
    fn forget_musician(&mut self, input: &Input, cand: &Vec<P>, musician: usize) {
        for &k in &input.inst_musicians[input.musicians[musician]] {
            self.score -= (1.0 + self.closeness[k]) * self.coef_musicians[k];
            if input.version != Version::One {
                if k == musician {
                    self.closeness[k] = 0.0;
                } else {
                    self.closeness[k] -= 1.0 / (cand[self.to[musician]] - cand[self.to[k]]).abs();
                }
            }
        }
        self.coef_musicians[musician] = 0.0;
    }
    /// 指定された音楽家のcoef_musiciansを計算し、closenessとスコアへの影響を反映させる
    fn apply_musician(&mut self, input: &Input, cand: &Vec<P>, musician: usize) {
        for &k in &input.inst_musicians[input.musicians[musician]] {
            if k != musician {
                if input.version != Version::One {
                    let c = 1.0 / (cand[self.to[musician]] - cand[self.to[k]]).abs();
                    self.closeness[k] += c;
                    self.closeness[musician] += c;
                }
                self.score += (1.0 + self.closeness[k]) * self.coef_musicians[k];
            }
        }
        let p = self.to[musician];
        for j in 0..input.pos.len() {
            if self.block_count[p][j] == 0 {
                self.coef_musicians[musician] +=
                    score1(input, cand[p], input.musicians[musician], j);
            }
        }
        self.score += (1.0 + self.closeness[musician]) * self.coef_musicians[musician];
    }
    // 位置i1とi2の音楽家を交換する
    fn swap(&mut self, input: &Input, cand: &Vec<P>, i1: usize, i2: usize) -> Option<f64> {
        let k1 = self.musicians[i1];
        let k2 = self.musicians[i2];
        if k1 == !0 || k2 == !0 || input.musicians[k1] == input.musicians[k2] {
            None
        } else {
            let old = self.score;
            self.forget_musician(input, cand, k1);
            self.forget_musician(input, cand, k2);
            self.musicians.swap(i1, i2);
            self.to[k1] = i2;
            self.to[k2] = i1;
            self.apply_musician(input, cand, k1);
            self.apply_musician(input, cand, k2);
            Some(self.score - old)
        }
    }
    /// 位置fromで演奏している音楽家を位置toに移動させる
    fn mov(
        &mut self,
        input: &Input,
        cand: &Vec<P>,
        block: &Vec<Vec<(usize, usize)>>,
        conflict: &Vec<Vec<usize>>,
        from: usize,
        to: usize,
    ) -> Option<f64> {
        if self.musicians[from] == !0
            || self.musicians[to] != !0
            || conflict[to]
                .iter()
                .any(|&p| p != from && self.musicians[p] != !0)
        {
            None
        } else {
            let k = self.musicians[from];
            let old = self.score;
            self.forget_musician(input, cand, k);
            for &(p, a) in &block[from] {
                self.block_count[p][a] -= 1;
                let k2 = self.musicians[p];
                if self.block_count[p][a] == 0 && k2 != !0 {
                    if input.musicians[k2] != input.musicians[k] {
                        self.score -= (1.0 + self.closeness[k2]) * self.coef_musicians[k2];
                    }
                    self.coef_musicians[k2] += score1(input, cand[p], input.musicians[k2], a);
                    if input.musicians[k2] != input.musicians[k] {
                        self.score += (1.0 + self.closeness[k2]) * self.coef_musicians[k2];
                    }
                }
            }
            self.musicians[from] = !0;
            self.musicians[to] = k;
            self.to[k] = to;
            for &(p, a) in &block[to] {
                let k2 = self.musicians[p];
                if self.block_count[p][a] == 0 && k2 != !0 {
                    if input.musicians[k2] != input.musicians[k] {
                        self.score -= (1.0 + self.closeness[k2]) * self.coef_musicians[k2];
                    }
                    self.coef_musicians[k2] -= score1(input, cand[p], input.musicians[k2], a);
                    if input.musicians[k2] != input.musicians[k] {
                        self.score += (1.0 + self.closeness[k2]) * self.coef_musicians[k2];
                    }
                }
                self.block_count[p][a] += 1;
            }
            self.apply_musician(input, cand, k);
            Some(self.score - old)
        }
    }
    /// 使う候補点集合は変えずに割り当て問題を解いて場所を最適化
    fn optimize_mcf(&mut self, input: &Input, cand: &Vec<P>) -> bool {
        let mut ws = mat![0; input.n_instruments(); input.musicians.len()];
        let mut cap = vec![0; input.n_instruments()];
        for i in 0..input.n_instruments() {
            cap[i] = input.inst_musicians[i].len();
            for j in 0..input.musicians.len() {
                let p = self.to[j];
                let mut coef = 0;
                for a in 0..input.n_attendees() {
                    if self.block_count[p][a] == 0 {
                        coef += score1(input, cand[p], i, a).ceil() as i64;
                    }
                }
                ws[i][j] = coef;
            }
        }
        let (_, tos) = weighted_matching_with_capacity(&ws, &cap);
        let mut ps = vec![0; input.n_instruments()];
        let mut to = vec![!0; input.n_musicians()];
        for i in 0..input.n_musicians() {
            to[i] = self.to[tos[input.musicians[i]][ps[input.musicians[i]]]];
            ps[input.musicians[i]] += 1;
        }
        let mut coef_musicians = vec![0.0; input.n_musicians()];
        let mut closeness = vec![0.0; input.n_musicians()];
        let mut score = 0.0;
        for i in 0..input.musicians.len() {
            for j in 0..input.n_attendees() {
                if self.block_count[to[i]][j] == 0 {
                    coef_musicians[i] += score1(input, cand[to[i]], input.musicians[i], j);
                }
            }
            if input.version != Version::One {
                for &j in &input.inst_musicians[input.musicians[i]] {
                    if j != i {
                        closeness[i] += 1.0 / (cand[to[i]] - cand[to[j]]).abs();
                    }
                }
            }
            score += (1.0 + closeness[i]) * coef_musicians[i];
        }
        if self.score >= score {
            false
        } else {
            for i in 0..input.n_musicians() {
                self.musicians[to[i]] = i;
            }
            self.score = score;
            self.to = to;
            self.coef_musicians = coef_musicians;
            self.closeness = closeness;
            true
        }
    }
}

// input outdir ansfiles..
fn main() {
    let inputfile = std::env::args().nth(1).unwrap();
    let outdir = std::env::args().nth(2).unwrap();
    if !std::fs::metadata(&outdir).is_ok() {
        std::fs::create_dir_all(&outdir).unwrap();
    }
    let input = preprocess(read_input_from_file(&inputfile));
    let mut cand = vec![];
    for ans in std::env::args().skip(3) {
        for p in read_output_from_file(&ans) {
            cand.push(p);
        }
    }
    cand.sort();
    cand.dedup();
    eprintln!("#candidates from ansfiles = {}", cand.len());
    add_cand(&input, &mut cand);
    // extend_cand(&input, &mut cand);
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
                        if is_blocked(cand[j], input.pos[k], cand[i]) {
                            tmp.push((j, k));
                        }
                    }
                }
            }
            tmp
        })
        .collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    let mut to = vec![!0; input.musicians.len()];
    let mut used = vec![false; cand.len()];
    for i in 0..input.musicians.len() {
        loop {
            let j = rng.gen_range(0, cand.len());
            if !used[j] {
                to[i] = j;
                for &k in &conflict[j] {
                    used[k] = true;
                }
                break;
            }
        }
    }
    let mut state = State::initialize(&input, &cand, &block, &to);
    let mut best = state.to.clone();
    let mut best_score = state.score;
    eprintln!("{:.3}: {}", get_time(), best_score);
    //const T0: f64 = 1e-2;
    //const T1: f64 = 1e-6;

    let mut sum = 1.0;
    let mut cnt = 0;

    let TL: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(600.0);
    let stime = get_time();
    for iter in 0.. {
        let t = (get_time() - stime) / TL;
        if t >= 1.0 {
            eprintln!("Iter = {}", iter);
            break;
        }
        let i1 = state.to[rng.gen_range(0, input.n_musicians())];
        let i2 = if rng.gen_bool(0.1) {
            rng.gen_range(0, cand.len())
        } else {
            near[i1].choose(&mut rng).unwrap().1
        };
        if state.musicians[i2] == !0 {
            if let Some(diff) = state.mov(&input, &cand, &block, &conflict, i1, i2) {
                sum += diff.abs() as f64;
                cnt += 1;

                let ave = sum / cnt as f64;
                let mut T = ave * (1.0 - t) * (1.0 - t);
                if T <= 1.0 {
                    T = 1.0;
                }

                if diff >= 0.0 || rng.gen_bool((diff as f64 / T).exp()) {
                } else {
                    state.mov(&input, &cand, &block, &conflict, i2, i1).unwrap();
                }
            }
        } else {
            if let Some(diff) = state.swap(&input, &cand, i1, i2) {
                sum += diff.abs() as f64;
                cnt += 1;

                let ave = sum / cnt as f64;
                let mut T = ave * (1.0 - t) * (1.0 - t);
                if T <= 1.0 {
                    T = 1.0;
                }
                if diff >= 0.0 || rng.gen_bool((diff as f64 / T).exp()) {
                } else {
                    state.swap(&input, &cand, i1, i2).unwrap();
                }
            }
        }
        if best_score.setmax(state.score) {
            if state.optimize_mcf(&input, &cand) {
                eprintln!("{:.0} -> {:.0}", best_score, state.score);
            }
            best_score.setmax(state.score);
            best = state.to.clone();
            eprintln!("{:.3}: {:.0}", get_time(), best_score);
        }
    }
    let out = best.into_iter().map(|p| cand[p]).collect();
    write_output(&out);
}
