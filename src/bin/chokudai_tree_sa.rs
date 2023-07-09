#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use handlebars::Output;
use icfpc2023::{
    self,
    candidate::get_candidate2,
    candidate_tree::get_candidate_tree,
    compute_score, compute_score_fast, compute_score_for_a_musician_fast,
    compute_score_for_instruments, get_time,
    mcf::{weighted_matching, weighted_matching_with_capacity},
    read_input, write_output, Input, P,
};
use num::complex::ComplexFloat;
use rand::{rngs::ThreadRng, Rng};

#[allow(non_upper_case_globals)]
const ng_num: usize = 9999999;

struct States {
    ps: Vec<P>,
    parent: Vec<Vec<usize>>,
    child: Vec<Vec<usize>>,
    connect: Vec<Vec<usize>>,
    valid: Vec<bool>,
    #[allow(dead_code)]
    points: Vec<Vec<i64>>,
    max_point: Vec<i64>,
    pair: Vec<usize>,

    active_list: Vec<usize>,
    active_pos: Vec<usize>,

    wait_list: Vec<usize>,
    wait_pos: Vec<usize>,

    state: Vec<usize>,
    dup: Vec<usize>,
    active_parent: Vec<usize>,
    rng: ThreadRng,

    rlist: Vec<(usize, bool)>,

    target_num: usize,
}
impl States {
    fn set_wait(&mut self, a: usize) {
        self.wait_list.push(a);
        self.wait_pos[a] = self.wait_list.len() - 1;
        self.state[a] = 1;
    }

    fn remove_wait(&mut self, a: usize) {
        let b = self.wait_list.pop().unwrap();
        if a != b {
            self.wait_list[self.wait_pos[a]] = b;
            self.wait_pos[b] = self.wait_pos[a];
        }
        self.wait_pos[a] = ng_num;
        self.state[a] = 0;
    }

    fn set_active(&mut self, a: usize) {
        self.remove_wait(a);

        self.active_list.push(a);
        self.active_pos[a] = self.active_list.len() - 1;
        self.state[a] = 2;
        //self.alist.push(a);
        self.rlist.push((a, true));

        //connect処理
        for i in 0..self.connect[a].len() {
            let t = self.connect[a][i];
            self.dup[t] += 1;
            if self.state[t] == 1 {
                self.remove_wait(t);
            }
        }

        //child処理
        for i in 0..self.child[a].len() {
            let t = self.child[a][i];
            self.active_parent[t] += 1;

            /*
            dbg!(
                self.state[t],
                self.dup[t],
                self.active_parent[t],
                self.parent[t].len()
            );
             */
            if self.state[t] == 0
                && self.dup[t] == 0
                && self.active_parent[t] == self.parent[t].len()
                && self.valid[t]
            {
                //dbg!("get!");
                self.set_wait(t);
            }
        }
    }

    fn remove_active(&mut self, a: usize, flag: bool) {
        //remove伝搬処理

        if flag {
            for i in 0..self.child[a].len() {
                let t = self.child[a][i];
                if self.state[t] == 2 {
                    self.remove_active(t, flag);
                }
            }
        }

        let b = self.active_list.pop().unwrap();
        if a != b {
            self.active_list[self.active_pos[a]] = b;
            self.active_pos[b] = self.active_pos[a];
        }
        self.active_pos[a] = ng_num;
        self.state[a] = 0;

        if self.state[a] == 0
            && self.dup[a] == 0
            && self.active_parent[a] == self.parent[a].len()
            && self.valid[a]
        {
            self.set_wait(a);
        }

        //self.rlist.push(a);
        self.rlist.push((a, false));

        //child処理
        for i in 0..self.child[a].len() {
            let t = self.child[a][i];
            self.active_parent[t] -= 1;
            if self.state[t] == 1 {
                self.remove_wait(t);
            }
        }

        //connect処理
        for i in 0..self.connect[a].len() {
            let t = self.connect[a][i];
            self.dup[t] -= 1;
            if self.state[t] == 0
                && self.dup[t] == 0
                && self.active_parent[t] == self.parent[t].len()
                && self.valid[t]
            {
                self.set_wait(t);
            }
        }
    }

    fn random_add(&mut self, l: usize) {
        let mut t = 99999999;
        let mut best = -99999999999;
        for _ in 0..l {
            let t2 = self.rng.gen_range(0, self.wait_list.len());
            let t3 = self.wait_list[t2];
            if self.max_point[t3] > best {
                t = t3;
                best = self.max_point[t3];
            }
        }
        self.set_active(t);

        if self.active_list.len() <= self.target_num
            && self.pair[t] != ng_num
            && self.state[self.pair[t]] == 1
        {
            self.set_active(self.pair[t]);
        }
    }

    #[allow(dead_code)]
    fn random_remove(&mut self) {
        let mut t;
        loop {
            t = self.rng.gen_range(0, self.active_list.len());
            if self.parent[t].len() == 0 {
                break;
            }
        }
        self.remove_active(self.active_list[t], true);

        if self.pair[t] != ng_num && self.state[self.pair[t]] == 2 {
            self.remove_active(self.pair[t], true);
        }
    }

    fn try_remove(&mut self, a: usize) {
        if self.state[a] == 2 {
            self.remove_active(a, true);
        }
    }

    fn reset_list(&mut self) {
        self.rlist = vec![];
    }

    fn rollback(&mut self) {
        let rr = self.rlist.len();
        for i in 0..rr {
            let r = self.rlist[rr - 1 - i];

            if !r.1 {
                self.set_active(r.0);
            } else {
                self.remove_active(r.0, false);
            }
        }

        /*
        let mut atode = vec![];

        for i in 0.. self.rlist.len(){
            let r = self.alist[i];
            if self.state[r] == 2{
                atode.push(r);
            }
            else{
                self.set_active(r);
            }
        }
        for i in 0.. self.alist.len(){
            let a = self.alist[i];
            self.remove_active(a);
        }
        for i in 0.. atode.len(){
            let r = atode[i];
            self.set_active(r);
        }
        */
    }
}

fn main() {
    let inp = read_input();

    let ret = get_candidate_tree(&inp);
    let n = ret.0.len();

    let mut st = States {
        ps: ret.0,
        parent: ret.1,
        child: ret.2,
        connect: ret.3,
        valid: ret.4,
        points: ret.5,
        max_point: ret.6,
        pair: ret.7,

        active_list: vec![],
        active_pos: vec![ng_num; n],

        wait_list: vec![],
        wait_pos: vec![ng_num; n],

        state: vec![0; n],
        dup: vec![0; n],
        active_parent: vec![0; n],
        rng: rand::thread_rng(),

        rlist: vec![],
        target_num: inp.musicians.len(),
    };

    let chokudai_ret = chokudai_solve(inp.clone());

    let mut iter = 0;

    let mut now_score = -999999999999999;
    let mut best_score = -999999999999999;
    let mut best_ret = vec![];

    for i in 0..st.parent.len() {
        if st.parent[i].len() == 0 && st.valid[i] {
            st.set_wait(i);
        }
    }

    let m = inp.musicians.len();

    let music_n = inp.tastes[0].len();
    let mut music_num = vec![0; music_n];
    let mut music_index = vec![vec![]; music_n];
    for i in 0..inp.musicians.len() {
        music_num[inp.musicians[i]] += 1;
        music_index[inp.musicians[i]].push(i);
    }
    let music_num = music_num;

    let mut sum = 1.0;
    let mut cnt = 0;

    let center_parent = ret.8;

    let mut fast_mode = true;

    'outloop: loop {
        for i in 0..st.wait_list.len() {
            let t = st.wait_list[i];
            for j in 0..chokudai_ret.len() {
                if (chokudai_ret[j].0 - st.ps[t].0).abs() < 0.01
                    && (chokudai_ret[j].1 - st.ps[t].1).abs() < 0.01
                {
                    st.set_active(t);
                    continue 'outloop;
                }
            }
        }
        break;
    }

    dbg!(st.active_list.len());
    dbg!(inp.musicians.len());

    'outloop: for j in 0..chokudai_ret.len() {
        for t in 0..st.ps.len() {
            if (chokudai_ret[j].0 - st.ps[t].0).abs() < 0.01
                && (chokudai_ret[j].1 - st.ps[t].1).abs() < 0.01
            {
                eprintln!("Found {} {}", chokudai_ret[j].0, chokudai_ret[j].1);
                continue 'outloop;
            }
        }
        eprintln!("Not Found {} {}", chokudai_ret[j].0, chokudai_ret[j].1);
    }
    eprintln!("matching end");

    let tl: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(600.0);
    let stime = get_time();

    let volume_all_zero = vec![1.0; m];

    loop {
        let t = (get_time() - stime) / tl;
        iter += 1;

        if t >= 0.8 && fast_mode {
            eprintln!("Iter = {}, NowScore = {}", iter, now_score);
            fast_mode = false;
            now_score = -999999999999999;
            best_score = -999999999999999;
            cnt = 0;
            sum = 1.0;
        }

        if t >= 1.0 {
            eprintln!("Iter = {}", iter);
            break;
        }
        st.reset_list();

        /*
        while st.active_list.len() >= m {
            st.random_remove();
        }
        */

        if center_parent != ng_num && st.state[center_parent] == 2 && cnt != 0 {
            st.remove_active(center_parent, true);
        }

        if st.active_list.len() >= 1 && cnt != 0 {
            let act_list = st.active_list.clone();

            let target_p = st.ps[act_list[st.rng.gen_range(0, act_list.len())]];
            let target_range = st.rng.gen_range(30.0, 70.0);

            for i in 0..act_list.len() {
                let diff = target_p - st.ps[act_list[i]];
                if diff.abs2() <= target_range * target_range {
                    st.try_remove(act_list[i]);
                }
            }
        }

        /*
        if st.rng.gen_bool(0.5) {
            let range = (inp.stage1.0 - inp.stage0.0) / st.rng.gen_range(5.0, 30.0);

            let min_l = st
                .rng
                .gen_range(inp.stage0.0 - range / 2.0, inp.stage1.0 - range / 2.0);
            for t in act_list {
                if min_l <= st.ps[t].0 && min_l + range >= st.ps[t].0 {
                    rem_list.push(t);
                }
            }
        } else {
            let range = (inp.stage1.1 - inp.stage0.1) / st.rng.gen_range(5.0, 30.0);

            let min_l = st
                .rng
                .gen_range(inp.stage0.1 - range / 2.0, inp.stage1.1 - range / 2.0);
            for t in act_list {
                if min_l <= st.ps[t].1 && min_l + range >= st.ps[t].1 {
                    rem_list.push(t);
                }
            }
        }
        for t in rem_list {
            st.try_remove(t);
        }
        */

        while st.active_list.len() < m {
            st.random_add(3);
        }

        let mut cand = vec![];
        for ii in 0..st.active_list.len() {
            let i = st.active_list[ii];
            cand.push(st.ps[i]);
        }

        let pos_to_music = {
            if !fast_mode {
                compute_score_for_instruments(&inp, &cand)
            } else {
                let mut ret = vec![vec![0; music_n]; m];
                for i in 0..m {
                    ret[i] = st.points[st.active_list[i]].clone();
                }
                ret
            }
        };

        /*
        for i in 0..pos_to_music.len() {
            for j in 0..pos_to_music[0].len() {
                eprintln!(
                    "{} {} {} {}",
                    i, j, pos_to_music[i][j], st.points[st.active_list[i]][j]
                )
            }
        }
        */

        let mut ar = Vec::new();
        for i in 0..pos_to_music[0].len() {
            let mut br = Vec::new();
            for j in 0..pos_to_music.len() {
                br.push(pos_to_music[j][i]);
            }
            ar.push(br);
        }

        let ans = weighted_matching_with_capacity(&ar, &music_num);

        let mut ret = vec![P(0.0, 0.0); m];
        for i in 0..ans.1.len() {
            for j in 0..ans.1[i].len() {
                ret[music_index[i][j]] = cand[ans.1[i][j]];
            }
        }

        let score = ans.0;

        let diff = score - now_score;

        sum += diff.abs() as f64;
        cnt += 1;

        if cnt == 1 {
            sum = 1.0;
        }

        let ave = sum / cnt as f64 / 10.0;
        #[allow(non_snake_case)]
        let mut T = ave * (1.0 - t) * (1.0 - t);
        if T <= 1.0 {
            T = 1.0;
        }

        //let score2 = compute_score(&inp, &ret);

        //dbg!(score);A
        //if score > best_score{

        //dbg!(diff, T, (diff as f64 / T).exp());

        if diff >= 0 || st.rng.gen_bool((diff as f64 / T).exp()) {
            now_score = score;
            if best_score < score {
                best_ret = ret.clone();
                best_score = score;
                eprintln!("{} {} {}", t, best_score, iter);
                if fast_mode && t > 0.3 {
                    let real_score =
                        compute_score_fast(&inp, &(best_ret.clone(), volume_all_zero.clone())).0;
                    eprintln!("real: {}", real_score);
                }
            }
        } else {
            st.rollback();
        }

        //dbg!(st.wait_list.len());
        //break;
    }

    //dbg!(compute_score(&inp, &best_ret));
    write_output(&(best_ret.clone(), volume_all_zero.clone()));
}

fn chokudai_solve(inp: Input) -> Vec<P> {
    let mut start = vec![];
    for _i in 0..inp.pos.len() {
        start.push(1);
    }

    let mut best_score = 0;
    let mut best_ret = vec![];
    let mut best_start = start.clone();

    let volume_all_zero = vec![1.0; inp.musicians.len()];

    let mut rng = rand::thread_rng();

    let tl: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(10.0);
    let stime = get_time();

    let mut iter = 0;

    loop {
        let t = (get_time() - stime) / tl;
        if t >= 1.0 {
            eprintln!("Iter = {}", iter);
            break;
        }

        start = best_start.clone();
        let mut next_start = start.clone();
        let mut chflag = false;

        if iter != 0 {
            for i in 0..inp.pos.len() {
                if rng.gen_range(0, inp.pos.len()) <= 100 {
                    next_start[i] = rng.gen_range(0, 3);
                    chflag = true;
                } else {
                    next_start[i] = best_start[i];
                }
            }
        } else {
            chflag = true;
        }
        if !chflag {
            continue;
        }
        iter += 1;

        let candidate = get_candidate2(&inp, &next_start);

        let pos_to_music = compute_score_for_instruments(&inp, &candidate);

        dbg!(candidate.len());
        let mut ar = Vec::new();
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..candidate.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);
        let mut ret = Vec::new();
        for i in 0..inp.musicians.len() {
            ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
        }

        //let score = ans.0;

        let mut cand2 = Vec::new();
        for i in 0..inp.musicians.len() {
            cand2.push(ret[i]);
            //dbg!(pos_to_music[ans.1[i]][inp.musicians[i]]);
            //dbg!(compute_score_for_a_musician_fast(&inp, &ret, i).0);
        }
        let candidate = cand2;

        let pos_to_music = compute_score_for_instruments(&inp, &candidate);

        let mut ar = Vec::new();
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..candidate.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);

        let mut ret = Vec::new();
        for i in 0..inp.musicians.len() {
            ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
        }

        let score = compute_score_fast(&inp, &(ret.clone(), volume_all_zero.clone())).0;

        dbg!(score);
        if score > best_score {
            best_ret = ret.clone();
            best_score = score;
            best_start = next_start.clone();
            eprintln!("OK!");
        }
        //write_output(&best_ret);
    }

    {
        //dbg!(ans.0);
        //dbg!(compute_score_fast(&inp, &ret).0);

        let mut cand2 = Vec::new();
        for i in 0..inp.musicians.len() {
            cand2.push(best_ret[i]);
            //dbg!(pos_to_music[ans.1[i]][inp.musicians[i]]);
            //dbg!(compute_score_for_a_musician_fast(&inp, &ret, i).0);
        }
        let candidate = cand2;

        let pos_to_music = compute_score_for_instruments(&inp, &candidate);

        let mut ar = Vec::new();
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..candidate.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);

        let mut ret = Vec::new();
        for i in 0..inp.musicians.len() {
            ret.push(P(candidate[ans.1[i]].0, candidate[ans.1[i]].1));
        }

        let score = compute_score_fast(&inp, &(ret.clone(), volume_all_zero.clone())).0;

        best_ret = ret;

        dbg!(score);
    }

    //write_output(&best_ret);
    best_ret
    //dbg!(get_stage_diff(XY{x:inp.pos[0].0, y:inp.pos[0].1} , XY{x:inp.stage0.0, y:inp.stage0.1}, XY{x:inp.stage1.0, y:inp.stage1.1}));
}
