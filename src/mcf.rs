use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct E {
    pub to: usize,
    pub cap: i64,
    pub init: i64,
    pub cost: i64,
    pub rev: usize,
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub es: Vec<Vec<E>>,
    pub ex: Vec<i64>,
    pub p: Vec<i64>,
    iter: Vec<usize>,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        Graph {
            es: vec![vec![]; n],
            ex: vec![0; n],
            p: vec![0; n],
            iter: vec![0; n],
        }
    }
    pub fn add(&mut self, v: usize, to: usize, cap: i64, cost: i64) {
        let (fwd, rev) = (self.es[v].len(), self.es[to].len());
        self.es[v].push(E {
            to: to,
            cap: cap,
            init: cap,
            cost: cost,
            rev: rev,
        });
        self.es[to].push(E {
            to: v,
            cap: 0,
            init: 0,
            cost: -cost,
            rev: fwd,
        });
    }
    fn is_admissible(&self, v: usize, e: &E) -> bool {
        e.cap > 0 && e.cost + self.p[v] - self.p[e.to] < 0
    }
    /// Compute minimum cost circulation.
    /// Return whether there is a flow satisfying the demand constraints.
    /// flow(e) = init(e) - cap(e).
    /// For solving min cost s-t flow of value F, set ex(s)=F and ex(t)=-F.
    /// For every vertex, the total capacity of its incident edges must be fit in i64.
    /// Dual: minimize \sum_v ex(v)p(v) + \sum_{uv} cap(e) max(0, -cost(uv) - p(u) + p(v)).
    /// O(V^2 E log VC), where C=max(cost(e)). When cap=1, O(V E log VC).
    pub fn solve(&mut self) -> bool {
        let n = self.es.len();
        let mut eps = 0;
        for v in &mut self.es {
            for e in v {
                e.cost *= n as i64;
                eps.setmax(e.cost);
            }
        }
        let mut stack = vec![];
        let mut visit = vec![false; n];
        let mut ok = self.ex.iter().all(|&ex| ex == 0);
        'refine: loop {
            eps = (eps / 4).max(1);
            if ok && self.fitting() {
                break;
            }
            for v in 0..n {
                for i in 0..self.es[v].len() {
                    let e = self.es[v][i];
                    if self.is_admissible(v, &e) {
                        self.ex[e.to] += e.cap;
                        self.ex[v] -= e.cap;
                        self.es[e.to][e.rev].cap += e.cap;
                        self.es[v][i].cap = 0;
                    }
                }
            }
            loop {
                for v in 0..n {
                    self.iter[v] = 0;
                    if self.ex[v] > 0 {
                        visit[v] = true;
                        stack.push(v);
                    } else {
                        visit[v] = false;
                    }
                }
                if stack.len() == 0 {
                    break;
                }
                while let Some(v) = stack.pop() {
                    for e in &self.es[v] {
                        if !visit[e.to] && self.is_admissible(v, e) {
                            visit[e.to] = true;
                            stack.push(e.to);
                        }
                    }
                }
                if (0..n)
                    .filter(|&v| visit[v])
                    .flat_map(|v| self.es[v].iter())
                    .all(|e| e.cap <= 0 || visit[e.to])
                {
                    assert!(!ok);
                    break 'refine;
                }
                for v in (0..n).filter(|&v| visit[v]) {
                    self.p[v] -= eps
                }
                for v in 0..n {
                    while self.ex[v] > 0 {
                        let f = self.dfs(v, self.ex[v]);
                        if f == 0 {
                            break;
                        } else {
                            self.ex[v] -= f
                        }
                    }
                }
            }
            ok = true;
        }
        for v in &mut self.es {
            for e in v {
                e.cost /= n as i64;
            }
        }
        ok
    }
    fn dfs(&mut self, v: usize, f: i64) -> i64 {
        if self.ex[v] < 0 {
            let d = ::std::cmp::min(f, -self.ex[v]);
            self.ex[v] += d;
            return d;
        }
        while self.iter[v] < self.es[v].len() {
            let e = self.es[v][self.iter[v]];
            if self.is_admissible(v, &e) {
                let d = self.dfs(e.to, ::std::cmp::min(f, e.cap));
                if d > 0 {
                    self.es[v][self.iter[v]].cap -= d;
                    self.es[e.to][e.rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        0
    }
    fn fitting(&mut self) -> bool {
        let n = self.es.len();
        let mut d: Vec<i64> = self.p.iter().map(|&a| a / (n as i64)).collect(); // p must be non-positive.
        let mut d2: Vec<i64> = (0..n).map(|v| d[v] * (n as i64) - self.p[v] + 1).collect();
        let mut fixed = vec![false; n];
        let mut que = ::std::collections::BinaryHeap::new();
        for v in 0..n {
            que.push((-d2[v], v))
        }
        while let Some((_, v)) = que.pop() {
            if fixed[v] {
                continue;
            }
            fixed[v] = true;
            for e in &self.es[v] {
                if e.cap > 0 && {
                    let tmp = d2[v] + e.cost + self.p[v] - self.p[e.to] + 1;
                    d2[e.to].setmin(tmp)
                } {
                    if fixed[e.to] {
                        return false;
                    }
                    d[e.to] = d[v] + e.cost / (n as i64);
                    que.push((-d2[e.to], e.to));
                }
            }
        }
        self.p = d;
        true
    }
    pub fn val(&self) -> i64 {
        let mut tot = 0;
        for v in &self.es {
            for e in v {
                if e.cap < e.init {
                    tot += (e.init - e.cap) * e.cost;
                }
            }
        }
        tot
    }
}

pub fn weighted_matching(w: &Vec<Vec<i64>>) -> (i64, Vec<usize>) {
    let n = w.len();
    let m = w[0].len();
    let mut g = Graph::new(n + m + 1);
    for i in 0..n {
        g.ex[i] = 1;
        for j in 0..m {
            g.add(i, n + j, 1, -w[i][j]);
        }
    }
    for j in 0..m {
        g.add(n + j, n + m, 1, 0);
    }
    g.ex[n + m] = -(n as i64);
    g.solve();
    let mut to = vec![0; n];
    let mut score = 0;
    for i in 0..n {
        for e in &g.es[i] {
            if e.cap == 0 {
                to[i] = e.to - n;
            }
        }
        score += w[i][to[i]];
    }
    (score, to)
}

pub fn weighted_matching_with_capacity(
    w: &Vec<Vec<i64>>,
    cap: &Vec<usize>,
) -> (i64, Vec<Vec<usize>>) {
    let n = w.len();
    let m = w[0].len();
    let mut g = Graph::new(n + m + 1);
    for i in 0..n {
        g.ex[i] = cap[i] as i64;
        g.ex[n + m] -= cap[i] as i64;
        for j in 0..m {
            g.add(i, n + j, 1, -w[i][j]);
        }
    }
    for j in 0..m {
        g.add(n + j, n + m, 1, 0);
    }
    g.solve();
    let mut to = vec![vec![]; n];
    let mut score = 0;
    for i in 0..n {
        for e in &g.es[i] {
            if e.cap == 0 {
                to[i].push(e.to - n);
                score += w[i][e.to - n];
            }
        }
    }
    (score, to)
}
