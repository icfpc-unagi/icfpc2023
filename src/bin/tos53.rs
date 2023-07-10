use icfpc2023::*;

fn main() {
    let input = read_input_from_file("..//Dropbox/ICFPC2023/problems/problem-53.json");
    let mut output =
        read_output_from_file("best_submissions/53-232579960-64abee1d67169bacdb5f9b5d.json");
    let n = output.0.len();
    let indices: Vec<usize> = (0..n)
        .filter(|&i| (output.0[i] - P(6166.0, 1581.0)).abs() < 100.0)
        .collect();
    dbg!(&indices);
    dbg!(indices.len());
    let mut new_pos = vec![];
    let k = 3;
    let arc = candidate_arc::get_candidate(&input, 215, k);
    let mut x1 = arc[0].0;
    let mut x0 = arc[2*k].0;
    dbg!(&arc);
    new_pos.extend(arc);
    let m = (indices.len() - (2*k+1)) / 2;
    let y = 1573.0;
    for _ in 0..m {
        x0 -= 10.0 + 1e-8;
        new_pos.push(P(x0, y));
        x1 += 10.0 + 1e-8;
        new_pos.push(P(x1, y));
    }
    for i in 0..indices.len() {
        output.0[indices[i]] = new_pos[i];
    }
    let score = compute_score(&input, &output);
    dbg!(score);
    write_output(&output);

}
