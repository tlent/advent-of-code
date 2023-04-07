use advent_of_code_2022::day_13::{self, INPUT};

fn main() {
    let pairs = day_13::parse_input(INPUT);
    let part_one = day_13::part_one(&pairs);
    let part_two = day_13::part_two(&pairs);
    println!("{part_one}\n{part_two}");
}
