use icfpc2023::{compute_score_for_instruments, read_output_from_file, *};

fn main() {
    let input = read_input_from_file(&std::env::args().nth(1).unwrap());
    let output = read_output_from_file(&std::env::args().nth(2).unwrap());
    let score_pos_inst = compute_score_for_instruments(&input, &output.0);
    let mut ws = mat![0; input.musicians.len(); output.0.len()];
    for i in 0..input.musicians.len() {
        for j in 0..output.0.len() {
            ws[i][j] = score_pos_inst[j][input.musicians[i]].max(0);
        }
    }
    let (score, to) = icfpc2023::mcf::weighted_matching(&ws);
    let old_score =
        compute_score_fast(&input, &(output.0.clone(), vec![1.0; input.n_musicians()])).0;
    eprintln!("{} -> {}", old_score, score);
    let mut out = vec![];
    let mut volumes = vec![0.0; input.musicians.len()];
    if old_score < score {
        eprintln!("Update!!!!!!!!!!!!!!!!!!");
        for i in 0..input.musicians.len() {
            out.push(output.0[to[i]]);
            if score_pos_inst[to[i]][input.musicians[i]] < 0 {
                volumes[i] = 0.0;
            } else {
                volumes[i] = 10.0;
            }
        }
    } else {
        out = output.0.clone();
        for i in 0..input.musicians.len() {
            if score_pos_inst[i][input.musicians[i]] < 0 {
                volumes[i] = 0.0;
            } else {
                volumes[i] = 10.0;
            }
        }
    }
    write_output(&(out, volumes));
}
