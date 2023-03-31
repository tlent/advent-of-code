use advent_of_code_2022::day_10::{self, INPUT};

fn main() {
    let instructions = day_10::parse_input(INPUT);
    let (part_one, part_two) = day_10::solve(&instructions);
    println!("{}", part_one);
    println!("{}", part_two);
}
