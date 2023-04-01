use advent_of_code_2022::day_08::{self, INPUT};

fn main() {
    let grid = day_08::parse_input(INPUT);
    let part_one = day_08::part_one(&grid);
    let part_two = day_08::part_two(&grid);
    println!("{part_one}\n{part_two}");
}
