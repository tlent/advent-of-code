use year_2022_day_11::{self, INPUT};
use std::env;

fn main() {
    let mut parse_result = year_2022_day_11::parser::parse(INPUT).unwrap();
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2022_day_11::part_one(&mut parse_result.clone());
            println!("{part_one}");
            let part_two = year_2022_day_11::part_two(&mut parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2022_day_11::part_one(&mut parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2022_day_11::part_two(&mut parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
