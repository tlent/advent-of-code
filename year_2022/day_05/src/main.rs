use day_05::{self, INPUT};
use std::env;

fn main() {
    let (stacks, moves) = day_05::parse_input(INPUT).unwrap();
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = day_05::part_one(stacks.clone(), &moves);
            println!("{part_one}");
            let part_two = day_05::part_two(stacks, &moves);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = day_05::part_one(stacks, &moves);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = day_05::part_two(stacks, &moves);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
