static INPUT: &str = include_str!("../input");

fn main() {
    let mut adapters: Vec<u32> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    println!("part one: {}", part_one(&adapters));
    println!("part two: {}", part_two(&adapters));
}

fn part_one(adapters: &[u32]) -> u32 {
    let mut one_count = 0;
    let mut three_count = 1;
    let mut prev = 0;
    for &x in adapters {
        match x - prev {
            1 => one_count += 1,
            2 => {}
            3 => three_count += 1,
            _ => unreachable!(),
        }
        prev = x;
    }
    one_count * three_count
}

fn part_two(adapters: &[u32]) -> u64 {
    let mut prevs = [(0, 0u64); 3];
    prevs[2] = (0, 1);
    for &v in adapters {
        let mut path_count = 0;
        for (p, c) in &prevs {
            if v - p <= 3 {
                path_count += c;
            }
        }
        prevs.rotate_left(1);
        prevs[2] = (v, path_count);
    }
    prevs[2].1
}
