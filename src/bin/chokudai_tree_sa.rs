#![allow(unused_imports)]
use std::{collections::BinaryHeap, net::SocketAddr};

use aead::NewAead;
use icfpc2023::{
    self, candidate::get_candidate2, candidate_tree::get_candidate_tree, compute_score,
    compute_score_fast, compute_score_for_a_musician_fast, compute_score_for_instruments, get_time,
    mcf::weighted_matching, read_input, write_output, Input, P,
};
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

    active_list: Vec<usize>,
    active_pos: Vec<usize>,

    wait_list: Vec<usize>,
    wait_pos: Vec<usize>,

    state: Vec<usize>,
    dup: Vec<usize>,
    active_parent: Vec<usize>,
    rng: ThreadRng,

    rlist: Vec<(usize, bool)>,
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

            dbg!(
                self.state[t],
                self.dup[t],
                self.active_parent[t],
                self.parent[t].len()
            );
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
    }

    fn random_remove(&mut self) {
        let t = self.rng.gen_range(0, self.active_list.len());
        self.remove_active(self.active_list[t], true);
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

        active_list: vec![],
        active_pos: vec![ng_num; n],

        wait_list: vec![],
        wait_pos: vec![ng_num; n],

        state: vec![0; n],
        dup: vec![0; n],
        active_parent: vec![0; n],
        rng: rand::thread_rng(),

        rlist: vec![],
    };

    let tl: f64 = std::env::var("TL")
        .map(|a| a.parse().unwrap())
        .unwrap_or(600.0);
    let stime = get_time();

    let mut iter = 0;

    let mut best_score = -999999999999999;
    let mut best_ret = vec![];

    for i in 0..st.parent.len() {
        if st.parent[i].len() == 0 && st.valid[i] {
            st.set_wait(i);
        }
    }

    let m = inp.musicians.len();

    let mut sum = 1.0;
    let mut cnt = 0;

    loop {
        let t = (get_time() - stime) / tl;
        iter += 1;
        if t >= 1.0 {
            eprintln!("Iter = {}", iter);
            break;
        }
        st.reset_list();

        while st.active_list.len() >= m {
            st.random_remove();
        }

        while st.active_list.len() < m {
            st.random_add(10);
        }

        let mut cand = vec![];
        for ii in 0..st.active_list.len() {
            let i = st.active_list[ii];
            cand.push(st.ps[i]);
        }

        let pos_to_music = compute_score_for_instruments(&inp, &cand);

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
        for i in 0..inp.musicians.len() {
            let mut br = Vec::new();
            for j in 0..cand.len() {
                br.push(pos_to_music[j][inp.musicians[i]]);
            }
            ar.push(br);
        }

        let ans = weighted_matching(&ar);
        let mut ret = Vec::new();
        for i in 0..inp.musicians.len() {
            ret.push(P(cand[ans.1[i]].0, cand[ans.1[i]].1));
        }

        let score = ans.0;

        let diff = score - best_score;

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

        dbg!(score);
        //if score > best_score{

        //dbg!(diff, T, (diff as f64 / T).exp());

        if diff >= 0 || st.rng.gen_bool((diff as f64 / T).exp()) {
            best_ret = ret.clone();
            best_score = score;
            eprintln!("{} {}", t, best_score);
        } else {
            st.rollback();
        }

        dbg!(st.wait_list.len());
        //break;
    }

    //dbg!(compute_score(&inp, &best_ret));
    write_output(&best_ret);
}
