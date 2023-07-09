use icfpc2023::{compute_score_for_instruments, read_input, read_output_from_file, *};

fn main() {
    let input = read_input();
    let output = read_output_from_file(&std::env::args().nth(1).unwrap());
    let score_pos_inst = compute_score_for_instruments(&input, &output.0);
    let mut ws = mat![0; input.musicians.len(); output.0.len()];
    for i in 0..input.musicians.len() {
        for j in 0..output.0.len() {
            ws[i][j] = score_pos_inst[j][input.musicians[i]];
        }
    }
    let (score, to) = icfpc2023::mcf::weighted_matching(&ws);
    let old_score = compute_score_fast(&input, &output).0;
    eprintln!("{} -> {}", old_score, score);
    if old_score < score {
        eprintln!("Update!!!!!!!!!!!!!!!!!!");
    }
    let mut out = (vec![], output.1);
    for i in 0..input.musicians.len() {
        out.0.push(output.0[to[i]]);
    }
    write_output(&out);
}
