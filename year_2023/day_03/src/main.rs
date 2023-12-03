use std::env;
use year_2023_day_03::INPUT;

fn main() {
    let parse_result = year_2023_day_03::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2023_day_03::part_one(&parse_result);
            println!("{part_one}");
            let part_two = year_2023_day_03::part_two(&parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2023_day_03::part_one(&parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2023_day_03::part_two(&parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
