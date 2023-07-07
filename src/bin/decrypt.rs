use std::env;
use icfpc2023::secret::decrypt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments, expected a string to decrypt");
    }
    let encrypted = &args[1];
    println!("{}", decrypt(encrypted).unwrap());
}
