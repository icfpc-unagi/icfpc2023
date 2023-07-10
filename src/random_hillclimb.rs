use crate::simple_hillclimb::simple_hillclimb;

use super::*;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn hillclimb_random_move(
    input: &Input,
    mut output: Output,
    save_dir: &str,
    time_limit: Option<f64>,
) -> Output {
    let save_dir = simple_hillclimb::prepare_output_dir(&input, save_dir);
    let time_limit = time_limit.unwrap_or(1e9);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let mut scorerer = DynamicScorer::new_with_output(&input, &output);
    let score_original = scorerer.get_score();
    let mut iter_last_update = 0;
    println!("{}", score_original);

    let mut d: f64 = 1.0;
    let time_start = get_time();

    let mut musicians_order = (0..input.n_musicians()).collect::<Vec<_>>();

    for iter in 0.. {
        if iter % input.n_musicians() == 0 {
            musicians_order.shuffle(&mut rng);
        }
        let musician_id = musicians_order[iter % input.n_musicians()];

        let current_score = scorerer.get_score();
        if iter > 0 && iter % 1000 == 0 {
            simple_hillclimb::dump_output(&output, &save_dir, current_score);

            if get_time() > time_start + time_limit {
                break;
            }
        }

        if iter - iter_last_update > 10000 {
            d *= 0.1;
            if d < 0.001 + 1e-9 {
                break;
            }
            iter_last_update = iter;
            println!("New D: {} (iter={})", d, iter);
        }

        let vec; // 移動する方向のベクトル
        let is_orthogonal = rng.gen::<f64>() < 0.5;
        if is_orthogonal {
            let dir = rng.gen_range(0, 4);
            vec = match dir {
                0 => P(1.0, 0.0),
                1 => P(-1.0, 0.0),
                2 => P(0.0, 1.0),
                3 => P(0.0, -1.0),
                _ => unreachable!(),
            };
        } else {
            let th = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            vec = P(th.cos(), th.sin());
        }

        loop {
            let p_old = output.0[musician_id];
            let vol = output.1[musician_id];
            let mut p_new = p_old + vec * d * rng.gen::<f64>();
            p_new = P((p_new.0 as f32) as f64, (p_new.1 as f32) as f64);
            p_new.0 = p_new
                .0
                .max(input.stage0.0 + 10.0)
                .min(input.stage1.0 - 10.0);
            p_new.1 = p_new
                .1
                .max(input.stage0.0 + 10.0)
                .min(input.stage1.0 - 10.0);

            let score_old = scorerer.get_score();
            output.0[musician_id] = p_new;
            scorerer.move_musician(musician_id, p_new, vol);

            let mut is_improved = false;

            if is_valid_output(&input, &output, false) {
                // debug
                if iter % 1000 == 0 {
                    //assert_eq!(compute_score(&input, &output), scorerer.score);
                }

                let score_new = scorerer.get_score();
                if score_new > score_old {
                    is_improved = true;
                    let time_now = get_time();

                    eprintln!(
                        "UP-R t={:.1} iter={:10} {:10} -> {:10} --- {:+10} | {:+10}",
                        time_now - time_start,
                        iter,
                        score_old,
                        score_new,
                        score_new - score_old,
                        score_new - score_original,
                    );

                    /*
                    println!(
                        "{} {} -> ... -> {} -> {} (+{}) --- {} {}",
                        iter,
                        score_original,
                        score_old,
                        score_new,
                        score_new - score_original,
                        musician_id,
                        is_orthogonal
                    );
                    */
                }
            }

            if is_improved {
                iter_last_update = iter;
                continue;
            } else {
                output.0[musician_id] = p_old;
                scorerer.move_musician(musician_id, p_old, vol);
                break;
            }
        }
    }

    output
}
