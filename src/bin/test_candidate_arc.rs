use icfpc2023::*;

fn main() {
    let input = read_input_from_file("..//Dropbox/ICFPC2023/problems/problem-53.json");
    let mut output = vec![];
    for (k, i) in [283, 215, 121, 362, 73, 27].into_iter().enumerate() {
        output.extend(candidate_arc::get_candidate(&input, i, k));
    }
    output.extend(vec![P(0.0, 0.0); input.n_musicians() - output.len()]);
    let volumes = vec![10.0; output.len()];
    write_output(&(output, volumes));
}
