use year_2022_day_03::INPUT;
use std::env;

fn main() {
    let (part_one_sets, part_two_sets) = year_2022_day_03::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2022_day_03::part_one(&part_one_sets);
            println!("{part_one}");
            let part_two = year_2022_day_03::part_two(&part_two_sets);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2022_day_03::part_one(&part_one_sets);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2022_day_03::part_two(&part_two_sets);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
