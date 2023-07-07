use super::*;

const EXAMPLE_INPUT: &str = r#"
{
"room_width": 2000.0,
"room_height": 5000.0,
"stage_width": 1000.0,
"stage_height": 200.0,
"stage_bottom_left": [500.0, 0.0],
"musicians": [0, 1, 0],
"attendees": [
{ "x": 100.0, "y": 500.0, "tastes": [1000.0, -1000.0
] },
{ "x": 200.0, "y": 1000.0, "tastes": [200.0, 200.0]
},
{ "x": 1100.0, "y": 800.0, "tastes": [800.0, 1500.0]
}
]
}
"#;

const EXAMPLE_OUTPUT: &str = r#"
{
    "placements": [
    {"x": 590.0, "y": 10.0 },
    {"x": 1100.0, "y": 100.0 },
    {"x": 1100.0, "y": 150.0 }
    ]
    }
"#;

pub fn is_blocked(input: &Input, output: &Output, musician_id: usize, attendee_id: usize) -> bool {
    let musician_pos = output[musician_id];
    let attendee_pos = input.pos[attendee_id];

    for i in 0..input.n_musicians() {
        if i == musician_id {
            continue;
        }

        let d2 = P::dist2_sp((musician_pos, attendee_pos), output[i]);
        if d2 <= 25.0 {
            return true;
        }
    }

    return false;
}

pub fn compute_score_for_pair(
    input: &Input,
    output: &Output,
    musician_id: usize,
    attendee_id: usize,
) -> i64 {
    if is_blocked(input, output, musician_id, attendee_id) {
        return 0;
    } else {
        let d2 = (input.pos[attendee_id] - output[musician_id]).abs2();
        // dbg!(&attendee_id, &musician_id, &input.tastes.len());
        let instrument_id = input.musicians[musician_id];
        return (1_000_000.0 * input.tastes[attendee_id][instrument_id] / d2).ceil() as i64;
    }
}

pub fn is_valid_output(input: &Input, output: &Output) -> bool {
    if output.len() != input.n_musicians() {
        eprintln!("Number of musicians is wrong");
        return false;
    }

    // musician VS stage bbox
    for i in 0..input.n_musicians() {
        let p = &output[i];
        if p.0 < input.stage0.0 + 10.0
            || p.0 > input.stage1.0 - 10.0
            || p.1 < input.stage0.1 + 10.0
            || p.1 > input.stage1.1 - 10.0
        {
            eprintln!("Musician {} out of stage bbox: {:?}", i, &p);
            return false;
        }
    }

    // musician VS musician
    for i in 0..input.n_musicians() {
        for j in 0..i {
            if (output[i] - output[j]).abs2() <= 25.0 {
                eprintln!(
                    "Musicians too close: {} and {} ({:?}, {:?})",
                    j, i, output[j], output[i]
                );
                return false;
            }
        }
    }

    true
}

pub fn compute_score(input: &Input, output: &Output) -> i64 {
    if !is_valid_output(input, output) {
        return 0;
    }

    let mut score = 0;
    for musician_id in 0..input.n_musicians() {
        for attendee_id in 0..input.n_attendees() {
            score += compute_score_for_pair(input, output, musician_id, attendee_id);
        }
    }
    score
}
