use advent_of_code_2022::day_09::{self, INPUT};

fn main() {
    let motions = day_09::parse_input(INPUT);
    let part_one = day_09::part_one(&motions);
    let part_two = day_09::part_two(&motions);
    println!("{part_one}\n{part_two}");
}
