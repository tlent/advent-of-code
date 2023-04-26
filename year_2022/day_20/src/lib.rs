pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(numbers: &[i64]) -> i64 {
    let len = numbers.len();
    let mut positions = (0..len).collect::<Vec<_>>();
    mix(numbers, &mut positions);
    let zero_position = positions.iter().position(|&p| numbers[p] == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| numbers[positions[(zero_position + i) % len]])
        .sum()
}

pub fn part_two(numbers: &[i64]) -> i64 {
    const MULTIPLIER: i64 = 811_589_153;
    let len = numbers.len();
    let numbers = numbers.iter().map(|v| v * MULTIPLIER).collect::<Vec<_>>();
    let mut positions = (0..len).collect::<Vec<_>>();
    for _ in 0..10 {
        mix(&numbers, &mut positions);
    }
    let zero_position = positions.iter().position(|&p| numbers[p] == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| numbers[positions[(zero_position + i) % len]])
        .sum()
}

fn mix(numbers: &[i64], positions: &mut Vec<usize>) {
    for (i, &value) in numbers.iter().enumerate() {
        if value == 0 {
            continue;
        }
        let position = positions.iter().position(|&p| p == i).unwrap();
        positions.remove(position);
        let steps = value.unsigned_abs() as usize % positions.len();
        let new_position = if value > 0 {
            (position + steps) % positions.len()
        } else if position > steps {
            position - steps
        } else {
            positions.len() - (steps - position)
        };
        positions.insert(new_position, i);
    }
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
