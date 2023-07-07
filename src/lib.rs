pub mod scoring;
pub use scoring::*;
use serde::{Deserialize, Serialize};

pub mod secret;

#[cfg(feature = "tokio")]
#[cfg(feature = "reqwest")]
pub mod api;

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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct P(pub f64, pub f64);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Input {
    pub room: P,
    pub stage0: P,
    pub stage1: P,
    pub musicians: Vec<usize>,
    pub pos: Vec<P>,
    pub tastes: Vec<Vec<f64>>,
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
}

pub type Output = Vec<P>;

#[derive(Serialize, Deserialize, Debug)]
struct JsonAttendee {
    x: f64,
    y: f64,
    tastes: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonConcert {
    room_width: f64,
    room_height: f64,
    stage_width: f64,
    stage_height: f64,
    stage_bottom_left: P,
    musicians: Vec<usize>,
    attendees: Vec<JsonAttendee>,
}

pub fn read_input() -> Input {
    parse_input(&std::io::read_to_string(std::io::stdin()).unwrap())
}

pub fn read_input_from_file(path: &str) -> Input {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    parse_input(&content)
}

pub fn parse_input(s: &str) -> Input {
    let json: JsonConcert = serde_json::from_str(s).unwrap();
    Input {
        room: P(json.room_width, json.room_height),
        stage0: json.stage_bottom_left,
        stage1: P(
            json.stage_bottom_left.0 + json.stage_width,
            json.stage_bottom_left.1 + json.stage_height,
        ),
        musicians: json.musicians,
        pos: json.attendees.iter().map(|a| P(a.x, a.y)).collect(),
        tastes: json.attendees.into_iter().map(|a| a.tastes).collect(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Out {
    placements: Vec<XY>,
}

#[derive(Serialize, Deserialize, Debug)]
struct XY {
    x: f64,
    y: f64,
}

pub fn write_output(output: &Output) {
    let out = Out {
        placements: output.iter().map(|p| XY { x: p.0, y: p.1 }).collect(),
    };
    serde_json::to_writer(std::io::stdout(), &out).unwrap();
}

pub fn parse_output(s: &str) -> Output {
    let out: Out = serde_json::from_str(s).unwrap();
    out.placements.into_iter().map(|p| P(p.x, p.y)).collect()
}

pub fn read_output_from_file(path: &str) -> Output {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    parse_output(&content)
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
}

pub mod mcf;

pub mod vis;

pub mod input_stats;
