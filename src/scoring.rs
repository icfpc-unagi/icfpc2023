use super::*;
use std::f64::consts::PI;

pub const EXAMPLE_INPUT: &str = r#"
{
"room_width": 2000.0,
"room_height": 5000.0,
"stage_width": 1000.0,
"stage_height": 200.0,
"stage_bottom_left": [500.0, 0.0],
"musicians": [0, 1, 0],
"attendees": [
{ "x": 100.0, "y": 500.0, "tastes": [1000.0, -1000.0
] },
{ "x": 200.0, "y": 1000.0, "tastes": [200.0, 200.0]
},
{ "x": 1100.0, "y": 800.0, "tastes": [800.0, 1500.0]
}
]
}
"#;

pub const EXAMPLE_OUTPUT: &str = r#"
{
    "placements": [
    {"x": 590.0, "y": 10.0 },
    {"x": 1100.0, "y": 100.0 },
    {"x": 1100.0, "y": 150.0 }
    ]
    }
"#;

pub fn is_blocked(musician: P, attendee: P, blocking_musician: P) -> bool {
    let d2 = P::dist2_sp((musician, attendee), blocking_musician);
    d2 <= 25.0
}

pub fn is_blocked_by_someone(
    input: &Input,
    output: &Output,
    musician_id: usize,
    attendee_id: usize,
) -> bool {
    let musician_pos = output[musician_id];
    let attendee_pos = input.pos[attendee_id];
    for i in 0..output.len() {
        if i == musician_id {
            continue;
        }
        if is_blocked(musician_pos, attendee_pos, output[i]) {
            return true;
        }
    }

    return false;
}

pub fn score_fn(taste: f64, d2: f64) -> i64 {
    (1_000_000.0 * taste / d2).ceil() as i64
}

pub fn compute_score_for_pair(
    input: &Input,
    output: &Output,
    musician_id: usize,
    attendee_id: usize,
) -> i64 {
    if is_blocked_by_someone(input, output, musician_id, attendee_id) {
        return 0;
    } else {
        let d2 = (input.pos[attendee_id] - output[musician_id]).abs2();
        let instrument_id = input.musicians[musician_id];
        return score_fn(input.tastes[attendee_id][instrument_id], d2);
    }
}

pub fn is_valid_output(input: &Input, output: &Output) -> bool {
    if output.len() != input.n_musicians() {
        eprintln!("Number of musicians is wrong");
        return false;
    }

    // musician VS stage bbox
    for i in 0..input.n_musicians() {
        let p = &output[i];
        if p.0 < input.stage0.0 + 10.0
            || p.0 > input.stage1.0 - 10.0
            || p.1 < input.stage0.1 + 10.0
            || p.1 > input.stage1.1 - 10.0
        {
            eprintln!("Musician {} out of stage bbox: {:?}", i, &p);
            return false;
        }
    }

    // musician VS musician
    for i in 0..input.n_musicians() {
        for j in 0..i {
            if (output[i] - output[j]).abs2() <= 25.0 {
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

pub fn compute_score(input: &Input, output: &Output) -> i64 {
    if !is_valid_output(input, output) {
        return 0;
    }

    let mut score = 0;
    for musician_id in 0..input.n_musicians() {
        for attendee_id in 0..input.n_attendees() {
            score += compute_score_for_pair(input, output, musician_id, attendee_id);
        }
    }
    score
}

pub fn compute_score_for_musician(input: &Input, output: &Output) -> Vec<i64> {
    if !is_valid_output(input, output) {
        return vec![0; input.n_musicians()];
    }

    return (0..input.n_musicians())
        .map(|musician_id| {
            (0..input.n_attendees())
                .map(|attendee_id| compute_score_for_pair(input, output, musician_id, attendee_id))
                .sum()
        })
        .collect();
}

pub fn compute_score_for_attendees(input: &Input, output: &Output) -> Vec<i64> {
    if !is_valid_output(input, output) {
        return vec![0; input.n_attendees()];
    }

    return (0..input.n_attendees())
        .map(|attendee_id| {
            (0..input.n_musicians())
                .map(|musician_id| compute_score_for_pair(input, output, musician_id, attendee_id))
                .sum()
        })
        .collect();
}

/// score[pos_id][instrument_id]
pub fn compute_score_for_instruments(input: &Input, positions: &Vec<P>) -> Vec<Vec<i64>> {
    let mut score = vec![vec![0; input.n_instruments()]; positions.len()];

    for pos_id in 0..positions.len() {
        let mut bs = vec![false; input.n_attendees()];
        for attendee_id in 0..input.n_attendees() {
            bs[attendee_id] = is_blocked_by_someone(input, positions, pos_id, attendee_id);
        }

        for instrument_id in 0..input.n_instruments() {
            for attendee_id in 0..input.n_attendees() {
                if !bs[attendee_id] {
                    let d2 = (input.pos[attendee_id] - positions[pos_id]).abs2();
                    score[pos_id][instrument_id] +=
                        (1_000_000.0 * input.tastes[attendee_id][instrument_id] / d2).ceil() as i64;
                }
            }
        }
    }

    score
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

pub fn compute_score_for_a_musician_fast(
    input: &Input,
    output: &Output,
    musician_id: usize,
) -> (i64, Vec<i64>) {
    let eps = 1e-5;
    let p = output[musician_id];
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
                    f |= is_blocked(p, input.pos[attendee_id], output[*i]);
                    if f {
                        break;
                    }
                }
                if !f {
                    let d2 = (input.pos[attendee_id] - output[musician_id]).abs2();
                    let instrument_id = input.musicians[musician_id];
                    let s = score_fn(input.tastes[attendee_id][instrument_id], d2);
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
    if !is_valid_output(input, output) {
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
        let (s, sm) = compute_score_for_a_musician_fast(input, output, musician_id);
        score_total += s;
        score_musician[musician_id] = s;
        for attendee_id in 0..input.n_attendees() {
            score_attendee[attendee_id] += sm[attendee_id];
        }
    }

    (score_total, score_musician, score_attendee)
}

///////////////////////////////////////////////////////////////////////////////

pub struct Scorerer {
    pub input: Input,
    pub score: i64,
    pub musician_pos: Vec<Option<P>>,
    // n_blocking_musicians[musician_id][attendee_id] := # of other musicians bocking this edge
    pub n_blocking_musicians: Vec<Vec<usize>>,
}

impl Scorerer {
    pub fn new(input: &Input) -> Self {
        Self {
            input: input.clone(),
            score: 0,
            musician_pos: vec![None; input.n_musicians()],
            n_blocking_musicians: vec![vec![0; input.n_attendees()]; input.n_musicians()],
        }
    }

    fn bare_score_fn(&self, musician_id: usize, attendee_id: usize) -> i64 {
        let instrument_id = self.input.musicians[musician_id];
        let taste = self.input.tastes[attendee_id][instrument_id];
        let pos = self.musician_pos[musician_id].unwrap();
        let d2 = (self.input.pos[attendee_id] - pos).abs2();
        score_fn(taste, d2)
    }

    pub fn add_musician(&mut self, musician_id: usize, pos: P) -> i64 {
        assert_eq!(self.musician_pos[musician_id], None);
        self.musician_pos[musician_id] = Some(pos);

        let mut score_diff = 0;

        // Add new contributions
        for attendee_id in 0..self.input.n_attendees() {
            self.n_blocking_musicians[musician_id][attendee_id] = 0;
            for blocker_id in 0..self.input.n_musicians() {
                let blocker_pos = self.musician_pos[blocker_id];
                if blocker_id == musician_id || blocker_pos == None {
                    continue;
                }
                if is_blocked(pos, self.input.pos[attendee_id], blocker_pos.unwrap()) {
                    self.n_blocking_musicians[musician_id][attendee_id] += 1;
                }
            }
            if self.n_blocking_musicians[musician_id][attendee_id] == 0 {
                score_diff += self.bare_score_fn(musician_id, attendee_id);
            }
        }

        // Add new blocking
        for blocked_musician_id in 0..self.input.n_musicians() {
            let blocked_pos = self.musician_pos[blocked_musician_id];
            if blocked_pos == None || blocked_musician_id == musician_id {
                continue;
            }
            let blocked_pos = blocked_pos.unwrap();

            for attendee_id in 0..self.input.n_attendees() {
                if is_blocked(blocked_pos, self.input.pos[attendee_id], pos) {
                    self.n_blocking_musicians[blocked_musician_id][attendee_id] += 1;
                    if self.n_blocking_musicians[blocked_musician_id][attendee_id] == 1 {
                        score_diff -= self.bare_score_fn(blocked_musician_id, attendee_id);
                    }
                }
            }
        }

        self.score += score_diff;
        // dbg!(score_diff);
        score_diff
    }

    pub fn remove_musician(&mut self, musician_id: usize) -> i64 {
        assert_ne!(self.musician_pos[musician_id], None);
        let pos = self.musician_pos[musician_id].unwrap();
        let mut score_diff = 0;

        // Cancel the current contributions
        for attendee_id in 0..self.input.n_attendees() {
            if self.n_blocking_musicians[musician_id][attendee_id] == 0 {
                score_diff -= self.bare_score_fn(musician_id, attendee_id);
            }
            self.n_blocking_musicians[musician_id][attendee_id] = 0;
        }

        // Cancel the current blocking
        for blocked_musician_id in 0..self.input.n_musicians() {
            let blocked_pos = self.musician_pos[blocked_musician_id];
            if blocked_pos == None || blocked_musician_id == musician_id {
                continue;
            }
            let blocked_pos = blocked_pos.unwrap();

            for attendee_id in 0..self.input.n_attendees() {
                if is_blocked(blocked_pos, self.input.pos[attendee_id], pos) {
                    self.n_blocking_musicians[blocked_musician_id][attendee_id] -= 1;
                    if self.n_blocking_musicians[blocked_musician_id][attendee_id] == 0 {
                        score_diff += self.bare_score_fn(blocked_musician_id, attendee_id);
                    }
                }
            }
        }

        self.musician_pos[musician_id] = None;
        self.score += score_diff;
        score_diff
    }

    pub fn move_musician(&mut self, musician_id: usize, new_pos: P) -> i64 {
        let diff1 = self.remove_musician(musician_id);
        let diff2 = self.add_musician(musician_id, new_pos);
        diff1 + diff2
    }

    /*
    pub fn swap_musicians(&mut self, musician_id1: usize, musician_id2: usize) -> i64 {
        let pos1 = self.musician_pos[musician_id1];
        let pos2 = self.musician_pos[musician_id2];
        // この場合、ブロッキングは全く変わらない。単にこのmusicianたちが与えているスコアが変わる。

        let mut score_diff = 0;
        if let Some(pos1) = pos1 {
            for attendee_id in 0..self.input.n_attendees() {
                if self.n_blocking_musicians[musician_id1][attendee_id] == 0 {
                    score_diff -= self.bare_score_fn(musician_id1, attendee_id);
                }
            }
        }

        self.musician_pos[musician_id1] = pos2;
        self.musician_pos[musician_id2] = pos1;

        /*
        let diff1 = if pos1 == None {
            0
        } else {
            self.remove_musician(musician_id1)
        };
        let diff2 = if pos2 == None {
            0
        } else {
            self.remove_musician(musician_id2)
        };
        let diff3 = if pos1 == None {
            0
        } else {
            self.add_musician(musician_id2, pos1.unwrap())
        };
        let diff4 = if pos2 == None {
            0
        } else {
            self.add_musician(musician_id1, pos2.unwrap())
        };
        diff1 + diff2 + diff3 + diff4
        */
    }
    */
}
