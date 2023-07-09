#![allow(non_snake_case)]
use clap::Parser;
use icfpc2023::*;
use rand::seq::SliceRandom;
use rand::Rng;

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

    'outer: loop {
        let score_old = scorer.get_score();
        let candidate_poss = candidate_positions::enumerate_candidate_positions(&input, &output);
        dbg!(candidate_poss.len());
        assert!(candidate_poss.len() > 0);

        let mut musician_ids: Vec<usize> = (0..input.n_musicians()).collect();
        musician_ids.shuffle(&mut rng);

        for musician_id in musician_ids {
            let pos_old = output[musician_id];
            for pos in &candidate_poss {
                output[musician_id] = *pos;
                if !is_valid_output(&input, &output, false) {
                    continue;
                }
                scorer.move_musician(musician_id, *pos);
                let score_new = scorer.get_score();
                if score_new > score_old {
                    println!("{} {:10} -> {:10}", musician_id, score_old, score_new);
                    dbg!(compute_score(&input, &output));
                    continue 'outer;
                }
            }
            scorer.move_musician(musician_id, pos_old);
            output[musician_id] = pos_old;
        }

        break;
    }

    write_output_to_file(&output, "tmp.json");
}
