static INPUT: &str = include_str!("../input");

const TRANSFORM_DIVISOR: u64 = 20201227;

fn main() {
    let mut lines = INPUT.lines();
    let a = lines.next().unwrap().parse().unwrap();
    let b = lines.next().unwrap().parse().unwrap();
    println!("part one: {}", part_one(a, b));
}

fn transform_once(value: u64, subject_number: u64) -> u64 {
    (value * subject_number) % TRANSFORM_DIVISOR
}

fn find_loop_size(subject_number: u64, target: u64) -> u64 {
    let mut loop_size = 0;
    let mut value = 1;
    while value != target {
        value = transform_once(value, subject_number);
        loop_size += 1;
    }
    loop_size
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = transform_once(value, subject_number);
    }
    value
}

fn part_one(a: u64, b: u64) -> u64 {
    let a_loop_size = find_loop_size(7, a);
    transform(b, a_loop_size)
}
