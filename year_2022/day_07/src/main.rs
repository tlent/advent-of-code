use day_07::{self, INPUT};
use std::env;

fn main() {
    let (directory_sizes, root_size) = day_07::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = day_07::part_one(&directory_sizes);
            println!("{part_one}");
            let part_two = day_07::part_two(&directory_sizes, root_size);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = day_07::part_one(&directory_sizes);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = day_07::part_two(&directory_sizes, root_size);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
