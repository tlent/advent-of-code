use std::ptr;

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(numbers: &[i64]) -> i64 {
    let mut references = numbers.iter().collect::<Vec<_>>();
    mix(numbers, &mut references);
    coordinates_sum(&references)
}

pub fn part_two(numbers: &[i64]) -> i64 {
    const MULTIPLIER: i64 = 811_589_153;
    let numbers = numbers.iter().map(|v| v * MULTIPLIER).collect::<Vec<_>>();
    let mut references = numbers.iter().collect::<Vec<_>>();
    for _ in 0..10 {
        mix(&numbers, &mut references);
    }
    coordinates_sum(&references)
}

fn mix<'a>(numbers: &'a [i64], references: &mut Vec<&'a i64>) {
    for n in numbers {
        if *n == 0 {
            continue;
        }
        let index = references.iter().position(|&r| ptr::eq(r, n)).unwrap();
        references.remove(index);
        let new_index = (index as i64 + n).rem_euclid(references.len() as i64);
        references.insert(new_index as usize, n);
    }
}

fn coordinates_sum(references: &[&i64]) -> i64 {
    let zero_position = references.iter().position(|&r| *r == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| *references[(zero_position + i) % references.len()])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let numbers = parse_input(INPUT);
        assert_eq!(part_one(&numbers), 15297);
    }

    #[test]
    fn test_part_two() {
        let numbers = parse_input(INPUT);
        assert_eq!(part_two(&numbers), 2_897_373_276_210);
    }
}
