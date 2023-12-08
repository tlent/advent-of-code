use std::env;
use year_2023_day_08::INPUT;

fn main() {
    let input = year_2023_day_08::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2023_day_08::part_one(&input);
            println!("{part_one}");
            let part_two = year_2023_day_08::part_two(&input);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2023_day_08::part_one(&input);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2023_day_08::part_two(&input);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
