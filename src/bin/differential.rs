#![allow(non_snake_case)]
use clap::Parser;
use icfpc2023::{simple_hillclimb::simple_hillclimb, *};
use rand::Rng;

#[derive(Parser, Debug)]
struct Args {
    input_path: String,
    output_path: String,
    save_dir: String,
}

fn compute_grad(input: &Input, scorer: &DynamicScorer, musician_id: usize) -> P {
    let instrument_id = input.musicians[musician_id];
    let m = scorer.musician_pos[musician_id].unwrap();

    let closeness_factor = scorer.get_closeness_factor();

    // TODO: volume

    // Factor 1
    let mut grad1 = P(0.0, 0.0);
    for attendee_id in 0..input.n_attendees() {
        if !scorer.is_visible(musician_id, attendee_id) {
            continue;
        }

        // f(x) = t / dist^2 (where t = (1_000_000.0 * taste / (d * d)).ceil() as i64)
        let a = input.pos[attendee_id];
        let t = 1_000_000.0 * input.tastes[attendee_id][instrument_id];
        let d = a - m;
        grad1 = grad1 + d * (2.0 * t / d.abs2() * closeness_factor[musician_id]);
    }

    return grad1;

    if input.version == Version::One {
        return grad1;
    }

    // Factor 2
    let mut grad2 = P(0.0, 0.0);
    let mut bare_score_sum = 0;
    for attendee_id in 0..input.n_attendees() {
        if scorer.is_visible(musician_id, attendee_id) {
            bare_score_sum += scorer.pair_score[musician_id][attendee_id];
        }
    }
    for other_musician_id in 0..input.n_musicians() {
        if other_musician_id == musician_id || input.musicians[other_musician_id] != instrument_id {
            continue;
        }
        let vec = scorer.musician_pos[other_musician_id].unwrap() - m;
        let d = vec.abs();
        grad2 = grad2 + vec * (bare_score_sum as f64 / (d * d * d));
    }

    // Factor 3
    let mut grad3 = P(0.0, 0.0);
    for other_musician_id in 0..input.n_musicians() {
        if other_musician_id == musician_id || input.musicians[other_musician_id] != instrument_id {
            continue;
        }
        // TODO: ここどうにかしないと遅すぎィ！！！！！！！！
        let mut bare_score_sum = 0;
        for attendee_id in 0..input.n_attendees() {
            if scorer.is_visible(other_musician_id, attendee_id) {
                bare_score_sum += scorer.pair_score[other_musician_id][attendee_id];
            }
        }
        let vec = scorer.musician_pos[other_musician_id].unwrap() - m;
        let d = vec.abs();
        grad3 = grad3 + vec * (bare_score_sum as f64 / (d * d * d));
    }

    // dbg!(grad1, grad2, grad3);

    grad1 + grad2 + grad3
}

fn main() {
    let args = Args::parse();
    let input = read_input_from_file(&args.input_path);
    let mut output = read_output_from_file(&args.output_path);
    let save_dir = simple_hillclimb::prepare_output_dir(&input, &args.save_dir);
    let mut rng = rand::thread_rng();

    let mut scorer = DynamicScorer::new_with_output(&input, &output);
    let score_original = scorer.get_score();
    let mut iter_last_update = 0;
    println!("{}", score_original);

    let time_start = get_time();
    let mut lr: f64 = 1000.0;
    let mut touch_force = vec![P(0.0, 0.0); input.n_musicians()];

    for iter in 0.. {
        let musician_id = iter % input.n_musicians();

        if iter > 0 && iter % 1000 == 0 {
            let current_score = scorer.get_score();
            simple_hillclimb::dump_output(&output, &save_dir, current_score);
        }

        let grad = compute_grad(&input, &scorer, musician_id) * 1e-8 + touch_force[musician_id];
        touch_force[musician_id] = P(0.0, 0.0);

        // let grad = compute_grad(&input, &scorer, musician_id);
        if grad == P(0.0, 0.0) {
            continue;
        }

        let vec = grad;
        // dbg!(&grad);
        // let vec = grad * (1.0 / grad.abs()) + touch_force[musician_id];
        // touch_force[musician_id] = P(0.0, 0.0);

        //
        // ここから下はhillclimbコピペ
        //

        let p_old = output.0[musician_id];
        let vol = output.1[musician_id];

        let p_new = geom::first_hit(
            &input,
            &output.0,
            &mut touch_force,
            p_old,
            vec,
            lr * rng.gen::<f64>(),
        );

        let score_old = scorer.get_score();
        output.0[musician_id] = p_new;
        scorer.move_musician(musician_id, p_new, vol);

        let mut is_improved = false;

        if is_valid_output(&input, &output, false) {
            let score_new = scorer.get_score();
            if score_new > score_old {
                is_improved = true;
                let time_now = get_time();

                eprintln!(
                    "UP t={:.1} iter={:10} {:10} -> {:10} --- {:+10} | {:+10}",
                    time_now - time_start,
                    iter,
                    score_old,
                    score_new,
                    score_new - score_old,
                    score_new - score_original,
                );
            } else if score_new == score_old {
                is_improved = true;
            }
        }

        if is_improved {
            // output.1 = volume_new;
            iter_last_update = iter;
        } else {
            output.0[musician_id] = p_old;
            scorer.move_musician(musician_id, p_old, vol);
        }
    }
}
