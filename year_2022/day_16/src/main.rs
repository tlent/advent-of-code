use day_16::{self, INPUT};

fn main() {
    let parse_result = day_16::parser::parse(INPUT).unwrap();
    let (distances, releasable_valve_ids) = day_16::preprocess(&parse_result);
    let part_one = day_16::part_one(&parse_result, &distances, &releasable_valve_ids);
    println!("{part_one}");
    let part_two = day_16::part_two(&parse_result, &distances, &releasable_valve_ids);
    println!("{part_two}");
}
