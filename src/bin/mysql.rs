#![allow(unused_imports)]
use icfpc2023;
use mysql::params;

fn main() {
    let value: i64 = icfpc2023::mysql::cell(
        "SELECT CAST(:a + :b AS SIGNED)",
        params! {"a" => 1, "b" => 20},
    )
    .unwrap().unwrap();
    dbg!("Result: {}", value);
}
