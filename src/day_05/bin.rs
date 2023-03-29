use advent_of_code_2022::day_05::{self, INPUT};

fn main() {
    let (stacks, moves) = day_05::parse_input(INPUT);
    println!("{}", day_05::part_one(stacks.clone(), &moves));
    println!("{}", day_05::part_two(stacks, &moves));
}
