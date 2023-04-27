use day_19::{self, INPUT};
use std::env;

fn main() {
    let parse_result = day_19::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = day_19::part_one(&parse_result);
            println!("{part_one}");
            let part_two = day_19::part_two(&parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = day_19::part_one(&parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = day_19::part_two(&parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
