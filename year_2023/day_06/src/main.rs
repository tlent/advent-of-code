use std::env;
use year_2023_day_06::INPUT;

fn main() {
    let (part_one_races, part_two_race) = year_2023_day_06::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2023_day_06::part_one(&part_one_races);
            println!("{part_one}");
            let part_two = year_2023_day_06::part_two(&part_two_race);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2023_day_06::part_one(&part_one_races);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2023_day_06::part_two(&part_two_race);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
