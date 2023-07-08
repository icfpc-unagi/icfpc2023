#![allow(unused_imports)]
use icfpc2023::{self, compute_score};
use icfpc2023::{Input, Output, P};

use glob::glob;
use std::path::Path;
use std::str::FromStr;

use std::time::{Duration, Instant};

#[allow(dead_code)]
fn measure_average_execution_time<F>(f: F, iterations: usize)
where
    F: Fn() -> (),
{
    let mut total_duration = Duration::new(0, 0);

    for _ in 0..iterations {
        let start = Instant::now();

        f();

        let duration = start.elapsed();
        total_duration += duration;
    }

    println!("{:?}", total_duration / (iterations as u32));
}

fn main() {
    let input = icfpc2023::read_input_from_file("../Dropbox/ICFPC2023/problems/problem-2.json");
    let output = icfpc2023::read_output_from_file(
        "../Dropbox/ICFPC2023/chokudai-out1/2.json",
        //"../Dropbox/ICFPC2023/chokudai-out1/1.json",
        //"..//Downloads/submission-p1-2023-07-07T17_29_13.529879303Z.json",
        // "..//Downloads/submission-p45-2023-07-07T15_26_09.822142655Z.json",
    );

    let mut scorerer = icfpc2023::Scorerer::new(&input);
    for i in 0..input.n_musicians() {
        scorerer.add_musician(i, output[i]);

        let remove_musician_id = (i * 12308120398123 + 120938102938) % (i + 1);
        let score_diff2 = scorerer.remove_musician(remove_musician_id);
        let score_diff3 = scorerer.add_musician(remove_musician_id, output[remove_musician_id]);
        assert_eq!(score_diff2, -score_diff3);

        if i > 0 {
            let swap_musician_id = (i * 12313414 + 20931023) % i;
            let score_diff2 = scorerer.swap_musicians(swap_musician_id, i);
            let score_diff3 = scorerer.swap_musicians(swap_musician_id, i);
            assert_eq!(score_diff2, -score_diff3);
        }

        // dbg!(scorerer.score);
    }
    dbg!(scorerer.score);
    dbg!(compute_score(&input, &output));
    return;

    /*
    //let output = icfpc2023::read_output_from_file();

    // let output =
    // dbg!(&input);
    // dbg!(&output);

    // let input = icfpc2023::parse_input(icfpc2023::EXAMPLE_INPUT);
    // let output = icfpc2023::parse_output(icfpc2023::EXAMPLE_OUTPUT);

    dbg!(input.n_musicians(), input.n_attendees());

    dbg!(icfpc2023::compute_score(&input, &output)); // 1342980889 for problem 1
    dbg!(icfpc2023::compute_score_fast(&input, &output).0);

    measure_average_execution_time(
        || {
            icfpc2023::compute_score(&input, &output);
        },
        3,
    );
    measure_average_execution_time(
        || {
            icfpc2023::compute_score_fast(&input, &output);
        },
        3,
    );

    return;
    /*
    dbg!(icfpc2023::compute_score_for_a_musician_fast(
        &input, &output, 1
    ));

    let mat = icfpc2023::compute_score_for_instruments(&input, &output);
    // dbg!(&mat);

    let mut s = 0;
    for i in 0..input.n_musicians() {
        s += mat[i][input.musicians[i]];
    }
    dbg!(&s);
    */
    */
}
