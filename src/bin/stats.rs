#![allow(unused_imports)]
use icfpc2023;
use icfpc2023::{Input, Output, P};

use glob::glob;
use std::path::Path;
use std::str::FromStr;

pub fn get_sorted_problem_paths(pattern: &str) -> Vec<String> {
    let mut paths: Vec<_> = glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .map(|path| path.to_str().unwrap().to_string())
        .collect();

    paths.sort_by(|a, b| {
        let a = extract_number_from_path(a);
        let b = extract_number_from_path(b);
        a.cmp(&b)
    });

    paths
}

fn extract_number_from_path(path: &str) -> i32 {
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();

    let start = filename.find('-').unwrap() + 1;
    let end = filename.find('.').unwrap();

    i32::from_str(&filename[start..end]).unwrap()
}

fn compute_density(input: &Input) -> f64 {
    let b = input.stage1 - input.stage0;
    let n_max = (b.0 + b.1) * 2.0 / 10.0;
    return input.n_musicians() as f64 / n_max;
}

fn main() {
    let paths = get_sorted_problem_paths("../Dropbox/ICFPC2023/problems/problem-*.json");

    println!("id\t#Mus\t#Att\tDensity");
    for path in paths {
        let input = icfpc2023::read_input_from_file(&path);
        let problem_id = extract_number_from_path(&path);
        println!(
            "{}\t{}\t{}\t{:.2}",
            problem_id,
            input.n_musicians(),
            input.n_attendees(),
            compute_density(&input),
        );
    }

    /*
    let input = icfpc2023::parse_input(EXAMPLE_INPUT);
    let output = icfpc2023::parse_output(EXAMPLE_OUTPUT);
    // let output =
    dbg!(&input);
    dbg!(&output);

    dbg!(input.n_musicians(), input.n_attendees());

    dbg!(icfpc2023::compute_score(&input, &output));
    */
}
