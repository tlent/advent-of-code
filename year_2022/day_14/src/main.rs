use day_14::{self, INPUT};
use std::env;

fn main() {
    let mut parse_result = day_14::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = day_14::part_one(&mut parse_result.clone());
            println!("{part_one}");
            let part_two = day_14::part_two(&mut parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = day_14::part_one(&mut parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = day_14::part_two(&mut parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
