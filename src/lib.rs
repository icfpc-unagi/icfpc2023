pub mod scoring;
use anyhow::Result;
pub use scoring::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;

pub mod candidate_positions;
pub mod secret;

#[cfg(feature = "tokio")]
#[cfg(feature = "reqwest")]
pub mod api;

#[cfg(feature = "tokio")]
#[cfg(feature = "reqwest")]
pub mod www;

#[cfg(feature = "mysql")]
pub mod sql;

#[cfg(feature = "resvg")]
pub mod svg_to_png;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
    ($($e:expr),*) => { vec![$($e),*] };
    ($($e:expr,)*) => { vec![$($e),*] };
    ($e:expr; $d:expr) => { vec![$e; $d] };
    ($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct P(pub f64, pub f64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Version {
    One,
    Two,
}

impl Version {
    pub fn from_problem_id(problem_id: i32) -> Self {
        if problem_id <= 55 {
            Version::One
        } else {
            Version::Two
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Input {
    pub room: P,
    pub stage0: P,
    pub stage1: P,
    pub musicians: Vec<usize>,
    pub inst_musicians: Vec<Vec<usize>>,
    pub pos: Vec<P>,
    pub tastes: Vec<Vec<f64>>,
    pub pillars: Vec<(P, f64)>,
    pub version: Version,
    pub problem_id: Option<i32>,
}

impl Input {
    pub fn n_musicians(&self) -> usize {
        self.musicians.len()
    }

    pub fn n_attendees(&self) -> usize {
        self.pos.len()
    }

    pub fn n_instruments(&self) -> usize {
        self.tastes[0].len()
    }

    pub fn is_same_instrument(&self, musician_id1: usize, musician_id2: usize) -> bool {
        self.musicians[musician_id1] == self.musicians[musician_id2]
    }

    pub fn in_stage(&self, p: P) -> bool {
        p.0 >= self.stage0.0 + 10.0
            && p.0 <= self.stage1.0 - 10.0
            && p.1 >= self.stage0.1 + 10.0
            && p.1 <= self.stage1.1 - 10.0
    }
}

pub type Output = (Vec<P>, Vec<f64>);

#[derive(Serialize, Deserialize, Debug)]
struct JsonAttendee {
    x: f64,
    y: f64,
    tastes: Vec<f64>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Pillar {
    center: P,
    radius: f64,
}

/// Corresponds to the input json format.
#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    room_width: f64,
    room_height: f64,
    stage_width: f64,
    stage_height: f64,
    stage_bottom_left: P,
    musicians: Vec<usize>,
    attendees: Vec<JsonAttendee>,
    pillars: Vec<Pillar>,
}

impl Problem {
    pub fn room_size(&self) -> P {
        P(self.room_width, self.room_height)
    }
    pub fn stage_size(&self) -> P {
        P(self.stage_width, self.stage_height)
    }
}

impl From<Problem> for Input {
    fn from(p: Problem) -> Self {
        let pillars = p
            .pillars
            .iter()
            .map(|p| (p.center, p.radius))
            .collect::<Vec<_>>();
        // We can assume that `pillers not empty` equals `full round problem with closeness scoring`.
        // https://discord.com/channels/1118159165060292668/1126853058186444942/1127235665067790398
        let version_guessed_from_input = if pillars.is_empty() {
            Version::One
        } else {
            Version::Two
        };
        let mut inst_musicians = vec![vec![]; p.attendees[0].tastes.len()];
        for i in 0..p.musicians.len() {
            inst_musicians[p.musicians[i]].push(i);
        }

        Input {
            room: p.room_size(),
            stage0: p.stage_bottom_left,
            stage1: p.stage_bottom_left + p.stage_size(),
            musicians: p.musicians,
            inst_musicians,
            pos: p.attendees.iter().map(|a| P(a.x, a.y)).collect(),
            tastes: p.attendees.into_iter().map(|a| a.tastes).collect(),
            pillars,
            version: version_guessed_from_input,
            problem_id: None,
        }
    }
}

pub fn problem_id_from_path(path: &str) -> i32 {
    let re = regex::Regex::new(r"problems?-([0-9]+)\.json").unwrap();
    let caps = re.captures(path).unwrap();
    let num_str = caps.get(1).map_or("", |m| m.as_str());
    num_str.parse::<i32>().unwrap()
}

// #[deprecated] // コンパイル時警告はかえって治安悪化。
pub fn read_input() -> Input {
    parse_input(&std::io::read_to_string(std::io::stdin()).unwrap())
}

pub fn read_input_from_file(path: &str) -> Input {
    let problem_id = problem_id_from_path(path);
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    let mut input = parse_input_with_version(&content, Version::from_problem_id(problem_id));
    input.problem_id = Some(problem_id);
    input
}

// #[deprecated] // コンパイル時警告はかえって治安悪化。
pub fn parse_input(s: &str) -> Input {
    eprintln!(
        "{}\n!!!!!! D E P R E C A T E D !!!!!!!\n{}",
        "=".repeat(80),
        "=".repeat(80),
    );
    // parse_input_with_version(s, Version::One)
    let json: Problem = serde_json::from_str(s).unwrap();
    json.into()
}

pub fn parse_input_with_version(s: &str, version: Version) -> Input {
    let json: Problem = serde_json::from_str(s).unwrap();
    let mut input: Input = json.into();
    // If conflicts with the given version, report warning to stderr.
    if input.version != version {
        eprintln!(
            "WARNING: Version mismatch: version {:?} guessed from the input, but version {:?} given",
            input.version, version
        );
        input.version = version;
    }
    input
}

/// Corresponds to the output json format.
#[derive(Serialize, Deserialize, Debug)]
struct Solution {
    placements: Vec<XY>,
    #[serde(default)]
    volumes: Vec<f64>,
}

impl From<&Output> for Solution {
    fn from(output: &Output) -> Self {
        Solution {
            placements: output.0.iter().map(|p| p.into()).collect(),
            volumes: output.1.clone(),
        }
    }
}

impl From<&Solution> for Output {
    fn from(solution: &Solution) -> Self {
        (
            solution.placements.iter().map(|p| P(p.x, p.y)).collect(),
            solution.volumes.clone(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct XY {
    x: f64,
    y: f64,
}

impl From<&P> for XY {
    fn from(p: &P) -> Self {
        XY { x: p.0, y: p.1 }
    }
}

impl Into<P> for XY {
    fn into(self) -> P {
        P(self.x, self.y)
    }
}

pub fn write_output(output: &Output) {
    let out: Solution = output.into();
    serde_json::to_writer(std::io::stdout(), &out).unwrap();
}

pub fn write_output_to_file(output: &Output, file_name: &str) {
    let out = Solution {
        placements: output.0.iter().map(|p| p.into()).collect(),
        volumes: output.1.clone(),
    };
    let file = File::create(file_name).expect("unable to create file");
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &out).expect("unable to write data");
}

pub fn parse_output(s: &str) -> Result<Output> {
    let out: Solution = serde_json::from_str(s)?;
    let n = out.placements.len();
    Ok((
        out.placements.into_iter().map(|p| P(p.x, p.y)).collect(),
        if out.volumes.is_empty() {
            vec![1.0; n]
        } else {
            out.volumes.clone()
        },
    ))
}

pub fn parse_output_or_die(s: &str) -> Output {
    parse_output(s).unwrap()
}

pub fn read_output_from_file(path: &str) -> Output {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    parse_output_or_die(&content)
}

use std::ops::*;

impl Add for P {
    type Output = P;
    fn add(self, a: P) -> P {
        P(self.0 + a.0, self.1 + a.1)
    }
}

impl Sub for P {
    type Output = P;
    fn sub(self, a: P) -> P {
        P(self.0 - a.0, self.1 - a.1)
    }
}

impl Mul<f64> for P {
    type Output = P;
    fn mul(self, a: f64) -> P {
        P(self.0 * a, self.1 * a)
    }
}

impl Eq for P {}

impl Ord for P {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl P {
    pub fn dot(self, a: P) -> f64 {
        (self.0 * a.0) + (self.1 * a.1)
    }
    pub fn det(self, a: P) -> f64 {
        (self.0 * a.1) - (self.1 * a.0)
    }
    pub fn abs2(self) -> f64 {
        self.dot(self)
    }
    pub fn rot(self) -> P {
        P(-self.1, self.0)
    }
    pub fn rot60(self) -> P {
        P(
            self.0 * 0.5 - self.1 * ((3.0 as f64).sqrt()) / 2.0,
            self.0 * ((3.0 as f64).sqrt()) / 2.0 + self.1 / 2.0,
        )
    }
}

impl P {
    /// Square distance between segment and point.
    pub fn dist2_sp((p1, p2): (P, P), q: P) -> f64 {
        if (p2 - p1).dot(q - p1) <= 0.0 {
            (q - p1).abs2()
        } else if (p1 - p2).dot(q - p2) <= 0.0 {
            (q - p2).abs2()
        } else {
            P::dist2_lp((p1, p2), q)
        }
    }
    /// Square distance between line and point.
    pub fn dist2_lp((p1, p2): (P, P), q: P) -> f64 {
        let det = (p2 - p1).det(q - p1);
        det * det / (p2 - p1).abs2()
    }
    pub fn abs(self) -> f64 {
        self.abs2().sqrt()
    }
    pub fn pi_ll((p1, p2): (P, P), (q1, q2): (P, P)) -> Option<P> {
        let d = (q2 - q1).det(p2 - p1);
        if d == 0.0 {
            return None;
        }
        let r = p1 * d + (p2 - p1) * (q2 - q1).det(q1 - p1);
        Some(P(r.0 / d, r.1 / d))
    }
    /// [p1側, p2側].
    pub fn pi_cl((c, r): (P, f64), (p1, p2): (P, P)) -> Vec<P> {
        let v = p2 - p1;
        let q1 = p1 + v * (v.dot(c - p1) / v.abs2());
        let d = r * r - (q1 - c).abs2();
        if d < 0.0 {
            return vec![];
        }
        let q2 = v * (d / v.abs2()).sqrt();
        vec![q1 - q2, q1 + q2]
    }
    /// c1->c2の [右側, 左側].
    pub fn pi_cc((c1, r1): (P, f64), (c2, r2): (P, f64)) -> Vec<P> {
        let v = c2 - c1;
        let d = v.abs2().sqrt();
        if d <= 0.0 {
            return vec![];
        }
        let x = (r1 * r1 - r2 * r2 + d * d) / (d + d);
        let y = r1 * r1 - x * x;
        if y < 0.0 {
            return vec![];
        }
        let q1 = c1 + v * (x / d);
        let q2 = v.rot() * (y.sqrt() / d);
        vec![q1 - q2, q1 + q2]
    }
    /// 接線の接点. c->p の [右側, 左側]
    pub fn tan_cp((c, r): (P, f64), p: P) -> Vec<P> {
        let v = p - c;
        let d2 = v.abs2();
        let y = d2 - r * r;
        if y < 0.0 {
            return vec![];
        }
        let q1 = c + v * (r * r / d2);
        let q2 = v.rot() * (r * y.sqrt() / d2);
        vec![q1 - q2, q1 + q2]
    }
}

pub mod mcf;

pub mod vis;

pub mod input_stats;

pub mod candidate;

pub mod bigint_scoring;

#[cfg(test)]
mod tests {
    #[test]
    fn test_f64_roundtrip() {
        // make sure that it doesn't lose precision.
        // NOTE: It fails without feature "float_roundtrip".
        let original = "111.99999998999999";
        let value = serde_json::from_str::<f64>(original).unwrap();
        let converted = serde_json::to_string(&value).unwrap();
        assert_eq!(original, &converted);
    }
}

pub mod candidate_tree;
