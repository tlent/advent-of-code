use day_12::{self, INPUT};

fn main() {
    let grid = day_12::parse_input(INPUT);
    let part_one = day_12::part_one(&grid);
    let part_two = day_12::part_two(&grid);
    println!("{part_one}\n{part_two}");
}
