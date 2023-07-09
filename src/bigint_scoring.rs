use std::ops::*;

use num::traits::Zero;

use super::{Input, Output, P};

type BigF = num::BigRational;

fn from_f64(f: f64) -> BigF {
    BigF::from_float(f).unwrap()
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct BigP(BigF, BigF);

impl From<P> for BigP {
    fn from(p: P) -> Self {
        Self(from_f64(p.0), from_f64(p.1))
    }
}

impl Add for BigP {
    type Output = BigP;
    fn add(self, a: BigP) -> BigP {
        BigP(self.0 + a.0, self.1 + a.1)
    }
}

impl Add for &BigP {
    type Output = BigP;
    fn add(self, a: &BigP) -> BigP {
        BigP(&self.0 + &a.0, &self.1 + &a.1)
    }
}

impl Sub for BigP {
    type Output = BigP;
    fn sub(self, a: BigP) -> BigP {
        BigP(self.0 - a.0, self.1 - a.1)
    }
}

impl Sub for &BigP {
    type Output = BigP;
    fn sub(self, a: &BigP) -> BigP {
        BigP(&self.0 - &a.0, &self.1 - &a.1)
    }
}

// impl Mul<f64> for BigP {
//     type Output = BigP;
//     fn mul(self, a: f64) -> BigP {
//         BigP(self.0 * a, self.1 * a)
//     }
// }

impl BigP {
    pub fn dot(&self, a: &BigP) -> BigF {
        (&self.0 * &a.0) + (&self.1 * &a.1)
    }
    pub fn det(&self, a: &BigP) -> BigF {
        (&self.0 * &a.1) - (&self.1 * &a.0)
    }
    pub fn abs2(&self) -> BigF {
        self.clone().dot(self)
    }

    /// Square distance between segment and point.
    pub fn dist2_sp((p1, p2): (&BigP, &BigP), q: &BigP) -> BigF {
        if (p2 - p1).dot(&(q - p1)) <= BigF::zero() {
            (q - p1).abs2()
        } else if (p1 - p2).dot(&(q - p2)) <= BigF::zero() {
            (q - p2).abs2()
        } else {
            BigP::dist2_lp((p1, p2), q)
        }
    }

    /// Square distance between line and point.
    pub fn dist2_lp((p1, p2): (&BigP, &BigP), q: &BigP) -> BigF {
        let det = (p2 - p1).det(&(q - p1));
        det.pow(2) / (p2 - p1).abs2()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct BigInput {
    room: BigP,
    stage0: BigP,
    stage1: BigP,
    musicians: Vec<usize>,
    pos: Vec<BigP>,
    tastes: Vec<Vec<BigF>>,
}

impl BigInput {
    fn n_musicians(&self) -> usize {
        self.musicians.len()
    }

    fn n_attendees(&self) -> usize {
        self.pos.len()
    }
}

impl From<Input> for BigInput {
    fn from(input: Input) -> Self {
        Self {
            room: BigP::from(input.room),
            stage0: BigP::from(input.stage0),
            stage1: BigP::from(input.stage1),
            musicians: input.musicians,
            pos: input.pos.into_iter().map(BigP::from).collect(),
            tastes: input
                .tastes
                .into_iter()
                .map(|v| v.into_iter().map(from_f64).collect())
                .collect(),
        }
    }
}

type BigOutput = Vec<BigP>;

pub fn compute_score(input: &Input, output: &Output) -> i64 {
    let input = BigInput::from(input.clone());
    let output = output.iter().cloned().map(BigP::from).collect::<Vec<_>>();
    if !is_valid_output(&input, &output) {
        return 0;
    }

    let mut score = 0;
    for musician_id in 0..input.n_musicians() {
        for attendee_id in 0..input.n_attendees() {
            score += compute_score_for_pair(&input, &output, musician_id, attendee_id);
        }
    }
    score
}

fn is_valid_output(input: &BigInput, output: &BigOutput) -> bool {
    if output.len() != input.n_musicians() {
        eprintln!("Number of musicians is wrong");
        return false;
    }

    let _10 = from_f64(10.0);

    // musician VS stage bbox
    for i in 0..input.n_musicians() {
        let p = &output[i];
        if p.0 < &input.stage0.0 + &_10
            || p.0 > &input.stage1.0 - &_10
            || p.1 < &input.stage0.1 + &_10
            || p.1 > &input.stage1.1 - &_10
        {
            eprintln!("Musician {} out of stage bbox: {:?}", i, &p);
            return false;
        }
    }

    // musician VS musician
    for i in 0..input.n_musicians() {
        for j in 0..i {
            if (&output[i] - &output[j]).abs2() < _10.pow(2) {
                eprintln!(
                    "Musicians too close: {} and {} ({:?}, {:?})",
                    j, i, output[j], output[i]
                );
                return false;
            }
        }
    }

    true
}

fn is_blocked(musician: &BigP, attendee: &BigP, blocking_musician: &BigP) -> bool {
    let d2 = BigP::dist2_sp((musician, attendee), blocking_musician);
    d2 < from_f64(25.0)
}

fn is_blocked_by_someone(
    input: &BigInput,
    output: &BigOutput,
    musician_id: usize,
    attendee_id: usize,
) -> bool {
    let musician_pos = &output[musician_id];
    let attendee_pos = &input.pos[attendee_id];
    for i in 0..output.len() {
        if i == musician_id {
            continue;
        }
        if is_blocked(musician_pos, attendee_pos, &output[i]) {
            return true;
        }
    }

    return false;
}

pub fn score_fn(taste: &BigF, d2: BigF) -> i64 {
    (from_f64(1_000_000.0) * taste / d2)
        .ceil()
        .to_integer()
        .try_into()
        .unwrap()
}

fn compute_score_for_pair(
    input: &BigInput,
    output: &BigOutput,
    musician_id: usize,
    attendee_id: usize,
) -> i64 {
    if is_blocked_by_someone(input, output, musician_id, attendee_id) {
        return 0;
    } else {
        let d2 = (&input.pos[attendee_id] - &output[musician_id]).abs2();
        let instrument_id = input.musicians[musician_id];
        return score_fn(&input.tastes[attendee_id][instrument_id], d2);
    }
}

///////////////////////////////////////////////////////////////////////////////
// fast
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Event {
    MusicianEnter(usize),
    MusicianLeave(usize),
    Attendee(usize),
}

fn compute_score_for_a_musician_fast(
    input: &Input,
    output: &Output,
    big_input: &BigInput,
    big_output: &BigOutput,
    musician_id: usize,
) -> (i64, Vec<i64>) {
    const PI: f64 = std::f64::consts::PI;
    let eps = 1e-5;
    let p = output[musician_id];
    let big_p = &big_output[musician_id];
    let mut events = vec![];

    for i in 0..input.n_musicians() {
        if i == musician_id {
            continue;
        }

        let v = output[i] - p;
        let th = v.1.atan2(v.0);
        let dth = (5.0 / v.abs2().sqrt()).asin();

        // 一旦コンサバにして、後で正確なチェックをする
        let th0 = th - dth - eps;
        let th1 = th + dth + eps;

        events.push((th0, Event::MusicianEnter(i)));
        events.push((th1, Event::MusicianLeave(i)));

        if th0 < -PI {
            events.push((th0 + 2.0 * PI, Event::MusicianEnter(i)));
            events.push((th1 + 2.0 * PI, Event::MusicianLeave(i)));
        }
        if th1 > PI {
            events.push((th0 - 2.0 * PI, Event::MusicianEnter(i)));
            events.push((th1 - 2.0 * PI, Event::MusicianLeave(i)));
        }
    }

    for i in 0..input.n_attendees() {
        let v = input.pos[i] - p;
        let th = v.1.atan2(v.0);
        events.push((th, Event::Attendee(i)));
    }

    events.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut score = 0;
    let mut attendee_scores = vec![0; input.n_attendees()];
    let mut active_musicians = std::collections::HashSet::new();

    for (_, e) in events {
        match e {
            Event::MusicianEnter(i) => {
                active_musicians.insert(i);
            }
            Event::MusicianLeave(i) => {
                active_musicians.remove(&i);
            }
            Event::Attendee(attendee_id) => {
                let mut f = false;
                for i in &active_musicians {
                    let blocked_fast = crate::is_blocked(p, input.pos[attendee_id], output[*i]);
                    let blocked = is_blocked(big_p, &big_input.pos[attendee_id], &big_output[*i]);
                    if blocked != blocked_fast {
                        eprintln!("blocked_fast = {}, blocked = {}", blocked_fast, blocked);
                        dbg!((p, input.pos[attendee_id], output[*i]));
                    }
                    f |= blocked;
                    if f {
                        break;
                    }
                }
                if !f {
                    // let d2 = (&big_input.pos[attendee_id] - &big_output[musician_id]).abs2();
                    let instrument_id = input.musicians[musician_id];
                    // let s = score_fn(&big_input.tastes[attendee_id][instrument_id], d2);
                    // score のところだけ bigint 使っていない疑い!!!!
                    let d2 = (input.pos[attendee_id] - output[musician_id]).abs2();
                    // しかも sqrt している!!
                    let d2 = d2.sqrt().powf(2.0);
                    // 2乗の実装は不明。以下でもテストした範囲の挙動は一致。
                    // let d = d2.sqrt();
                    // let d2 = d * d;
                    let s: i64 = crate::score_fn(input.tastes[attendee_id][instrument_id], d2);
                    score += s;
                    attendee_scores[attendee_id] = s;
                }
            }
        }
    }

    (score, attendee_scores)
}

/// Returns (score, musician_scores, attendee_scores)
pub fn compute_score_fast(input: &Input, output: &Output) -> (i64, Vec<i64>, Vec<i64>) {
    let big_input = BigInput::from(input.clone());
    let big_output = output.iter().cloned().map(BigP::from).collect::<Vec<_>>();
    if !is_valid_output(&big_input, &big_output) {
        return (
            0,
            vec![0; input.n_musicians()],
            vec![0; input.n_attendees()],
        );
    }

    let mut score_total = 0;
    let mut score_musician = vec![0; input.n_musicians()];
    let mut score_attendee = vec![0; input.n_attendees()];
    for musician_id in 0..input.n_musicians() {
        let (s, sm) =
            compute_score_for_a_musician_fast(input, output, &big_input, &big_output, musician_id);
        score_total += s;
        score_musician[musician_id] = s;
        for attendee_id in 0..input.n_attendees() {
            score_attendee[attendee_id] += sm[attendee_id];
        }
    }

    (score_total, score_musician, score_attendee)
}
