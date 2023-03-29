use advent_of_code_2022::day_04::{self, INPUT};

fn main() {
    let pairs = day_04::parse_input(INPUT);
    println!("{}", day_04::part_one(&pairs));
    println!("{}", day_04::part_two(&pairs));
}
