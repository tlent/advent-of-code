use advent_of_code_2022::day_08::{self, INPUT};

fn main() {
    let grid = day_08::parse_input(INPUT);
    println!("{}", day_08::part_one(&grid));
    println!("{}", day_08::part_two(&grid));
}
