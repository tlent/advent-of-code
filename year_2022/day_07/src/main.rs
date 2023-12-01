use year_2022_day_07::INPUT;
use std::env;

fn main() {
    let (directory_sizes, root_size) = year_2022_day_07::parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = year_2022_day_07::part_one(&directory_sizes);
            println!("{part_one}");
            let part_two = year_2022_day_07::part_two(&directory_sizes, root_size);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = year_2022_day_07::part_one(&directory_sizes);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = year_2022_day_07::part_two(&directory_sizes, root_size);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}
