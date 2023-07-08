#![allow(unused_imports)]
use icfpc2023;

fn main() {
    let value: i32 = icfpc2023::mysql::cell("SELECT 1 + 1").unwrap();
    dbg!("Result: {}", value);
}
