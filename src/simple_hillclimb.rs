use super::*;
use rand::seq::SliceRandom;

pub fn prepare_output_dir(input: &Input, save_dir: &str) -> String {
    let save_dir = format!(
        "{}/problem-{}/",
        save_dir.to_owned(),
        input.problem_id.unwrap()
    );
    std::fs::create_dir_all(save_dir.to_owned()).unwrap();
    save_dir
}

fn dump_output(output: &Output, save_dir: &str, score: i64) {
    let out_name = format!("{}.txt", score);
    let out_path = format!("{}/{}", save_dir, out_name);
    write_output_to_file(&output, &out_path);

    let latest_path = format!("{}/latest.txt", save_dir);
    write_output_to_file(&output, &latest_path);

    eprintln!("Saved: {} {}", out_path, latest_path);
}

/*
/// 更新できたらすぐスワップする版
fn hillclimb_candidate(
    input: &Input,
    mut output: Output,
    save_dir: &str,
    mut candidate_limit: usize,
    max_iters: i64,
) -> Output {
    let mut rng = rand::thread_rng();
    let mut scorer = DynamicScorer::new_with_output(&input, &output);
    let mut iter = 0;
    dbg!(scorer.get_score());

    loop {
        let mut candidate_poss = candidate_positions::enumerate_candidate_positions_with_config(
            &input,
            &output,
            &candidate_positions::CandidateConfig {
                use_pattern1: true,
                use_pattern2: true,
                use_pattern3: true,
                use_pattern4: true,
                use_pattern23: false,
                limit_pattern2: Some(candidate_limit),
                limit_pattern3: Some(candidate_limit / 10), // tekitou
                limit_pattern23: None,
                filter_by_intersections1: Some(0),
                filter_by_reach14: true,
                filter_by_reach23: true,
                pattern2_disallow_blocked: true,
            },
        );
        candidate_poss.shuffle(&mut rng);
        if candidate_poss.len() > candidate_limit {
            candidate_poss = candidate_poss[..candidate_limit].to_vec();
        }
        eprintln!("Candidate set size: {}", candidate_poss.len());

        let mut updated = false;
        'outer: loop {
            let score_old = scorer.get_score();
            let mut musician_ids: Vec<usize> = (0..input.n_musicians()).collect();
            musician_ids.shuffle(&mut rng);

            for musician_id in musician_ids {
                if iter >= max_iters {
                    return output;
                }

                let vol = output.1[musician_id];
                let pos_old = output.0[musician_id];
                for pos in &candidate_poss {
                    iter += 1;
                    if iter % 10000 == 0 {
                        dump_output(&output, &save_dir, scorer.get_score());
                    }

                    output.0[musician_id] = *pos;
                    if !is_valid_output(&input, &output, false) {
                        continue;
                    }
                    scorer.move_musician(musician_id, *pos, vol);
                    let score_new = scorer.get_score();
                    if score_new > score_old {
                        println!("iter={} {:10} -> {:10}", iter, score_old, score_new);

                        let score_naive = compute_score(&input, &output);
                        if score_naive != score_new {
                            eprintln!("Score mismatch: {} vs {}", score_naive, score_new);
                        }

                        updated = true;
                        continue 'outer;
                    }
                }
                scorer.move_musician(musician_id, pos_old, vol);
                output.0[musician_id] = pos_old;
            }
            break;
        }

        if !updated {
            break;
        }
        // candidate_limit *= 2;
    }

    output
}
*/

/// 改善できたら移動するんじゃなくて、一番いいとこに移動する
pub fn hillclimb_candidate_findbest(
    input: &Input,
    mut output: Output,
    save_dir: &str,
    candidate_limit: usize,
    time_limit: f64,
) -> Output {
    let time_start = get_time();
    let mut rng = rand::thread_rng();
    let mut scorer = DynamicScorer::new_with_output(&input, &output);
    let mut iter = 0;
    dbg!(scorer.get_score());

    loop {
        let candidate_poss = candidate_positions::enumerate_candidate_positions_with_config(
            &input,
            &output,
            &candidate_positions::CandidateConfig {
                use_pattern1: true,
                use_pattern2: true,
                use_pattern3: true,
                use_pattern4: true,
                use_pattern23: false,
                limit_pattern2: Some(candidate_limit),
                limit_pattern3: Some(candidate_limit / 10), // tekitou
                limit_pattern23: None,
                filter_by_intersections1: Some(1),
                filter_by_intersections234: Some(1),
                filter_by_reach23: true,
                filter_by_reach14: false,
                pattern2_disallow_blocked: true,
            },
        );
        /*
        candidate_poss.shuffle(&mut rng);
        if candidate_poss.len() > candidate_limit {
            candidate_poss = candidate_poss[..candidate_limit].to_vec();
        }
        */
        eprintln!("Candidate set size: {}", candidate_poss.len());

        let mut updated = false;
        let mut time_last_save = time_start;
        loop {
            let mut musician_ids: Vec<usize> = (0..input.n_musicians()).collect();
            musician_ids.shuffle(&mut rng);

            for musician_id in musician_ids {
                let time_now = get_time();
                if time_now > time_start + time_limit {
                    dump_output(&output, &save_dir, scorer.get_score());
                    return output;
                }
                if time_now - time_last_save > 15.0 {
                    dump_output(&output, &save_dir, scorer.get_score());
                    time_last_save = time_now;
                }

                let vol = output.1[musician_id];
                let pos_old = output.0[musician_id];
                let score_old = scorer.get_score();
                let mut score_best = score_old;
                let mut pos_best = pos_old;

                for pos in &candidate_poss {
                    iter += 1;

                    output.0[musician_id] = *pos;
                    if !is_valid_output(&input, &output, false) {
                        continue;
                    }
                    scorer.move_musician(musician_id, *pos, vol);
                    let score_new = scorer.get_score();
                    if score_new > score_best {
                        score_best = score_new;
                        pos_best = *pos;
                    }
                }

                scorer.move_musician(musician_id, pos_best, vol);
                output.0[musician_id] = pos_best;
                if score_best > score_old {
                    let time_now = get_time();
                    println!(
                        "t={:.1} iter={} {:10} -> {:10} (+{:10})",
                        time_now - time_start,
                        iter,
                        score_old,
                        score_best,
                        score_best - score_old
                    );

                    let score_naive = compute_score(&input, &output);
                    if score_naive != score_best {
                        eprintln!("Score mismatch: {} vs {}", score_naive, score_best);
                    }
                    if time_now - time_last_save > 15.0 {
                        dump_output(&output, &save_dir, scorer.get_score());
                        time_last_save = time_now;
                    }
                    updated = true;
                }
            }
            break;
        }

        if !updated {
            break;
        }
        // candidate_limit *= 2;
    }

    output
}

pub fn simple_hillclimb(input: &Input, mut output: Output, save_dir: &str) -> Output {
    let save_dir = prepare_output_dir(input, save_dir);

    // let max_iters = 10000;
    let candidate_limit = 1000;
    loop {
        // output = hillclimb_candidate(input, output, &save_dir, candidate_limit, max_iters);
        output = hillclimb_candidate_findbest(input, output, &save_dir, candidate_limit, 60.0);
        dump_output(&output, &save_dir, compute_score(input, &output));

        // candidate_limit *= 2;
        // max_iters *= 2;
    }

    // unimplemented!();
}
