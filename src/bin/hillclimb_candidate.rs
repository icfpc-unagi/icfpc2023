#![allow(non_snake_case)]
use clap::Parser;
use icfpc2023::*;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
struct Args {
    input_path: String,
    output_path: String,
    save_dir: String,
}

fn main() {
    let args = Args::parse();
    let input = read_input_from_file(&args.input_path);
    let mut output = read_output_from_file(&args.output_path);
    let mut rng = rand::thread_rng();

    let mut scorer = DynamicScorer::new_with_output(&input, &output);
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
                limit_pattern2: Some(100),
                limit_pattern3: Some(10),
                limit_pattern23: None,
                filter_by_reach: true,
                pattern2_disallow_blocked: true,
            },
        );
        dbg!(candidate_poss.len());
        assert!(candidate_poss.len() > 0);

        let mut updated = false;
        'outer: loop {
            let score_old = scorer.get_score();
            let mut musician_ids: Vec<usize> = (0..input.n_musicians()).collect();
            musician_ids.shuffle(&mut rng);

            for musician_id in musician_ids {
                let pos_old = output.0[musician_id];
                for pos in &candidate_poss {
                    output.0[musician_id] = *pos;
                    if !is_valid_output(&input, &output, false) {
                        continue;
                    }
                    scorer.move_musician(musician_id, *pos);
                    let score_new = scorer.get_score();
                    if score_new > score_old {
                        println!("{} {:10} -> {:10}", musician_id, score_old, score_new);
                        dbg!(compute_score(&input, &output));
                        updated = true;
                        continue 'outer;
                    }
                }
                scorer.move_musician(musician_id, pos_old);
                output.0[musician_id] = pos_old;
            }
            break;
        }

        if !updated {
            break;
        }
    }

    write_output_to_file(&output, "tmp.json");
}
