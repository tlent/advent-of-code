use advent_of_code_2022::day_09::{self, INPUT};

fn main() {
    let motions = day_09::parse_input(INPUT);
    println!("{}", day_09::part_one(&motions));
    println!("{}", day_09::part_two(&motions));
}
