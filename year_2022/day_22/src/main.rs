use year_2022_day_22::INPUT;
use std::env;

fn main() {
    let (map_regions, path) = year_2022_day_22::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2022_day_22::part_one(&map_regions, &path);
            println!("{part_one}");
            let part_two = year_2022_day_22::part_two(&map_regions, &path);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2022_day_22::part_one(&map_regions, &path);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2022_day_22::part_two(&map_regions, &path);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
