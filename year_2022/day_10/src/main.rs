use year_2022_day_10::{self, INPUT};
use std::env;

fn main() {
    let parse_result = year_2022_day_10::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let (part_one, part_two) = year_2022_day_10::solve(&parse_result);
            println!("{part_one}\n{part_two}");
        }
        Some("parse") => {}
        _ => println!("Invalid argument: must be one of all or parse"),
    }
}
