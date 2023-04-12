use day_01::{self, INPUT};

fn main() {
    let group_sums = day_01::parse_input(INPUT).unwrap();
    let (part_one, part_two) = day_01::solve(group_sums);
    println!("{part_one}\n{part_two}");
}
