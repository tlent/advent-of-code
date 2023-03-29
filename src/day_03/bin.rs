use advent_of_code_2022::day_03::{self, INPUT};

fn main() {
    let lines: Vec<_> = INPUT.lines().collect();
    println!("{}", day_03::part_one(&lines));
    println!("{}", day_03::part_two(&lines));
}
