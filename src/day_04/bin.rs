use advent_of_code_2022::day_04::{self, INPUT};

fn main() {
    let pairs = day_04::parse_input(INPUT);
    let part_one = day_04::part_one(&pairs);
    let part_two = day_04::part_two(&pairs);
    println!("{part_one}\n{part_two}");
}
