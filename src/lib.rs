use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct P(f64, f64);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Input {
    pub room: P,
    pub stage0: P,
    pub stage1: P,
    pub musicians: Vec<usize>,
    pub pos: Vec<P>,
    pub tastes: Vec<Vec<f64>>,
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
    stage_bottom_left: (f64, f64),
    musicians: Vec<usize>,
    attendees: Vec<JsonAttendee>,
}

pub fn read_input() -> Input {
    let json: JsonConcert = serde_json::from_reader(std::io::stdin()).unwrap();
    Input {
        room: P(json.room_width, json.room_height),
        stage0: P(json.stage_bottom_left.0, json.stage_bottom_left.1),
        stage1: P(
            json.stage_bottom_left.0 + json.room_width,
            json.stage_bottom_left.1 + json.room_height,
        ),
        musicians: json.musicians,
        pos: json.attendees.iter().map(|a| P(a.x, a.y)).collect(),
        tastes: json.attendees.into_iter().map(|a| a.tastes).collect(),
    }
}

pub fn write_output(output: &Output) {
    #[derive(Serialize, Deserialize, Debug)]
    struct Out {
        placement: Vec<XY>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct XY {
        x: f64,
        y: f64,
    }
    let out = Out {
        placement: output.iter().map(|p| XY { x: p.0, y: p.1 }).collect(),
    };
    serde_json::to_writer(std::io::stdout(), &out).unwrap();
}
