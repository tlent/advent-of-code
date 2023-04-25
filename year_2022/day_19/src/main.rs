use day_19::{self, INPUT};

fn main() {
    let parse_result = day_19::parse_input(INPUT);
    // let part_one = day_19::part_one(&parse_result);
    // println!("{part_one}");
    let part_two = day_19::part_two(&parse_result);
    println!("{part_two}");
}
