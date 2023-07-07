use icfpc2023::{input_stats::*, read_input};

fn main() {
    let input = read_input();
    let (musicians_info, attendees_info) = get_stats(&input);
    println!("{:?}", (musicians_info, attendees_info));
}
