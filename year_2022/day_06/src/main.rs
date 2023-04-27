use day_06::{self, INPUT};
use std::env;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = day_06::part_one(INPUT);
            println!("{part_one}");
            let part_two = day_06::part_two(INPUT);
            println!("{part_two}");
        }
        Some("one") => {
            let part_one = day_06::part_one(INPUT);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = day_06::part_two(INPUT);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, one, or two"),
    }
}
