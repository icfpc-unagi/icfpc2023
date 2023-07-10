#![allow(non_snake_case)]
use clap::Parser;
use icfpc2023::*;
use rand::Rng;

#[derive(Parser, Debug)]
struct Args {
    input_path: String,
    output_path: String,
    save_dir: String,
}

fn dump_output(output: &Output, save_dir: &str, score: i64) {
    let out_name = format!("{}.txt", score);
    let out_path = format!("{}/{}", save_dir, out_name);
    write_output_to_file(&output, &out_path);

    let latest_path = format!("{}/latest.txt", save_dir);
    let _ = std::fs::remove_file(&latest_path).ok();
    let ret = std::os::unix::fs::symlink(out_name, latest_path);
    if let Err(e) = ret {
        eprintln!("Failed to create symlink: {:?}", e);
    }
}

fn main() {
    let args = Args::parse();
    let input = read_input_from_file(&args.input_path);
    let mut output = read_output_from_file(&args.output_path);
    let mut rng = rand::thread_rng();

    // 出力ディレクトリの準備
    let problem_name = std::path::Path::new(&args.input_path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let save_dir = args.save_dir.to_owned() + "/" + problem_name;
    std::fs::create_dir_all(save_dir.to_owned()).unwrap();

    let mut scorerer = DynamicScorer::new_with_output(&input, &output);
    let score_original = scorerer.get_score();
    let mut iter_last_update = 0;
    println!("{}", score_original);

    let mut d: f64 = 1.0;
    let time_start = get_time();

    for iter in 0.. {
        let current_score = scorerer.get_score();
        if iter > 0 && iter % 1000 == 0 {
            dump_output(&output, &save_dir, current_score);
        }

        if iter - iter_last_update > 10000 {
            d *= 0.1;
            if d < 0.001 + 1e-9 {
                break;
            }
            iter_last_update = iter;
            println!("New D: {} (iter={})", d, iter);
        }

        let musician_id = iter % input.n_musicians();

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
                        "UP t={:.1} iter={:10} {:10} -> {:10} --- {:+10} | {:+10}",
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
}
