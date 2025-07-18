static INPUT: &str = include_str!("../input");

fn main() {
    //let (target, ids) = parse_input(INPUT);
    //println!("part one: {}", part_one(target, &ids));
    println!("part two: {}", part_two(INPUT));
    //println!("part two simple: {}", part_two_simple(INPUT));
}

fn parse_input(input: &str) -> (u32, Vec<u32>) {
    let mut lines = input.lines();
    let target = lines.next().unwrap().parse().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();
    (target, ids)
}

fn part_one(target: u32, ids: &[u32]) -> u32 {
    let id = ids.iter().min_by_key(|&x| x - (target % x)).unwrap();
    let wait = id - (target % id);
    id * wait
}

// my solution using extended euclidean algorithm, chinese remainder theorem
fn part_two(input: &str) -> i128 {
    let ids: Vec<(i128, i128)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|v| ((v - (i as i128)) % v, v)))
        .collect();
    let (mut prev_remainder, mut prev_value) = ids[0];
    for &(remainder, value) in &ids[1..] {
        // (r, s, t) for extended euclidean algorithm
        let (high_value, high_remainder, low_value, low_remainder) = if prev_value > value {
            (prev_value, prev_remainder, value, remainder)
        } else {
            (value, remainder, prev_value, prev_remainder)
        };
        let mut a = (high_value, 1, 0);
        let mut b = (low_value, 0, 1);
        while b.0 > 0 {
            let q = a.0 / b.0;
            let r = a.0 - q * b.0;
            let s = a.1 - q * b.1;
            let t = a.2 - q * b.2;
            a = b;
            b = (r, s, t);
        }
        let (gcd, s, t) = a;
        assert_eq!(gcd, 1);
        assert_eq!(s * high_value + t * low_value, 1);
        prev_remainder = low_remainder * s * high_value + high_remainder * t * low_value;
        prev_value = prev_value * value;
        if prev_remainder < 0 {
            prev_remainder += ((prev_remainder / prev_value).abs() + 1) * prev_value;
        }
        if prev_remainder - prev_value > 0 {
            prev_remainder -= (prev_remainder / prev_value) * prev_value;
        }
    }
    prev_remainder
}

// simpler solution found after solving
fn part_two_simple(input: &str) -> u64 {
    let ids: Vec<(u64, u64)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|v| (i as u64, v)))
        .collect();
    let mut n = ids[0].1;
    let mut step = ids[0].1;
    for (offset, id) in &ids[1..] {
        while (n + offset) % id != 0 {
            n += step;
        }
        step *= id;
    }
    n
}
