use std::collections::BTreeSet;

use icfpc2023::read_input;

fn main() {
    let input = read_input();
    let stage_wh = input.stage1 - input.stage0;
    let stage_area = stage_wh.0 * stage_wh.1;
    let num_musician = input.n_musicians();
    let num_attendee = input.n_attendees();
    let num_instruments = BTreeSet::from_iter(input.musicians.clone()).len();
    assert_eq!(num_instruments, input.n_instruments());
    assert_eq!(num_instruments, input.musicians.iter().max().unwrap() + 1);
    println!(
        "{:?}",
        (stage_area, num_musician, num_attendee, num_instruments)
    );
}
