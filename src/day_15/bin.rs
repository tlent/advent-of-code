use advent_of_code_2022::day_15::{self, INPUT};

fn main() {
    let sensors = day_15::parser::parse(INPUT).unwrap();
    let part_one = day_15::part_one(&sensors);
    let part_two = day_15::part_two(&sensors);
    println!("{part_one}\n{part_two}");
}
