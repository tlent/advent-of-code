use day_02::{self, INPUT};

fn main() {
    let (part_one_rounds, part_two_rounds) = day_02::parse_input(INPUT);
    let part_one = day_02::part_one(&part_one_rounds);
    let part_two = day_02::part_two(&part_two_rounds);
    println!("{part_one}\n{part_two}");
}
