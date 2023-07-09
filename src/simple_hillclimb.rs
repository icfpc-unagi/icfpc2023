use super::*;

/*
fn prepare_output_dir(input: &Input, save_dir: &str) -> String {
    let save_dir = format!("{}/problem-{}/", args.save_dir.to_owned(), input.problem_id.unwrap())
    std::fs::create_dir_all(save_dir.to_owned()).unwrap();
    save_dir
}

fn hillclimb_candidate(input: &Input, output: Output, save_dir: &str, candidate_limit: usize) -> Output {
    let mut scorer = DynamicScorer::new_with_output(&input, &output);
    dbg!(scorer.get_score());

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
            limit_pattern3: Some(candidate_limit / 10),  // tekitou
            limit_pattern23: None,
            filter_by_reach: true,
            pattern2_disallow_blocked: true,
        },
    );


}

pub fn simple_hillclimb(input: &Input, output: Output, save_dir: &str) -> Output {
    let save_dir = prepare_output_dir(input, save_dir);

    let mut scorer = DynamicScorer::new_with_output(&input, &output);
    let score_original = scorer.get_score();

    unimplemented!();
}
*/
