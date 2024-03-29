use year_2022_day_05::INPUT;
use std::env;

fn main() {
    let (stacks, moves) = year_2022_day_05::parse_input(INPUT).unwrap();
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2022_day_05::part_one(stacks.clone(), &moves);
            println!("{part_one}");
            let part_two = year_2022_day_05::part_two(stacks, &moves);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2022_day_05::part_one(stacks, &moves);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2022_day_05::part_two(stacks, &moves);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
