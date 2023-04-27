use day_03::{self, INPUT};
use std::env;

fn main() {
    let (part_one_sets, part_two_sets) = day_03::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = day_03::part_one(&part_one_sets);
            println!("{part_one}");
            let part_two = day_03::part_two(&part_two_sets);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = day_03::part_one(&part_one_sets);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = day_03::part_two(&part_two_sets);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
