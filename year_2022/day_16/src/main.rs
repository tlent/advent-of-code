use day_16::{self, INPUT};
use std::env;

fn main() {
    let parse_result = day_16::parser::parse(INPUT).unwrap();
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let (processed_valves, initial_distances) = day_16::preprocess(parse_result);
            let part_one = day_16::part_one(&processed_valves, &initial_distances);
            println!("{part_one}");
            let part_two = day_16::part_two(&processed_valves, &initial_distances);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("preprocess") => {
            day_16::preprocess(parse_result);
        }
        Some("one") => {
            let (processed_valves, initial_distances) = day_16::preprocess(parse_result);
            let part_one = day_16::part_one(&processed_valves, &initial_distances);
            println!("{part_one}");
        }
        Some("two") => {
            let (processed_valves, initial_distances) = day_16::preprocess(parse_result);
            let part_two = day_16::part_two(&processed_valves, &initial_distances);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
