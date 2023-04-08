use advent_of_code_2022::day_14::{self, INPUT};

fn main() {
    let mut world = day_14::parse_input(INPUT);
    let part_one = day_14::part_one(&mut world.clone());
    let part_two = day_14::part_two(&mut world);
    println!("{part_one}\n{part_two}");
}
