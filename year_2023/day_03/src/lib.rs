use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub const INPUT: &str = include_str!("../input.txt");

pub struct Schematic {
    numbers: Vec<Number>,
    gear_positions: Vec<Coordinate>,
    symbol_positions: HashSet<Coordinate>,
}

pub struct Number {
    value: u32,
    positions: Vec<Coordinate>,
}

pub type Coordinate = (usize, usize);

pub fn parse_input(input: &str) -> Schematic {
    let mut schematic = Schematic {
        numbers: vec![],
        gear_positions: vec![],
        symbol_positions: HashSet::default(),
    };
    for (y, line) in input.lines().enumerate() {
        let mut iter = line.bytes().enumerate().peekable();
        while let Some((x, b)) = iter.next() {
            if b.is_ascii_digit() {
                let mut positions = vec![(x, y)];
                let start = x;
                let mut end = x;
                while let Some((_, b'0'..=b'9')) = iter.peek() {
                    iter.next();
                    end += 1;
                    positions.push((end, y));
                }
                let value = line[start..=end].parse().unwrap();
                schematic.numbers.push(Number { value, positions });
            } else if b == b'*' {
                schematic.gear_positions.push((x, y));
                schematic.symbol_positions.insert((x, y));
            } else if b != b'.' {
                schematic.symbol_positions.insert((x, y));
            }
        }
    }
    schematic
}

pub fn part_one(schematic: &Schematic) -> u32 {
    schematic
        .numbers
        .iter()
        .filter_map(|number| {
            if number.positions.iter().any(|&p| {
                adjacent_coordinates(p)
                    .any(|adjacent| schematic.symbol_positions.contains(&adjacent))
            }) {
                Some(number.value)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_two(schematic: &Schematic) -> u32 {
    let number_at_position: HashMap<Coordinate, (usize, u32)> = schematic
        .numbers
        .iter()
        .enumerate()
        .flat_map(|(i, number)| {
            number
                .positions
                .iter()
                .map(move |&p| (p, (i, number.value)))
        })
        .collect();
    schematic
        .gear_positions
        .iter()
        .filter_map(|&gear_position| {
            let adjacent_numbers: HashSet<_> = adjacent_coordinates(gear_position)
                .filter_map(|adjacent| number_at_position.get(&adjacent).copied())
                .collect();
            if adjacent_numbers.len() == 2 {
                Some(
                    adjacent_numbers
                        .iter()
                        .map(|(_, value)| value)
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn adjacent_coordinates((x, y): Coordinate) -> impl Iterator<Item = Coordinate> {
    (x.saturating_sub(1)..=x + 1)
        .flat_map(move |x| (y.saturating_sub(1)..=y + 1).map(move |y| (x, y)))
        .filter(move |&coord| coord != (x, y))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = parse_input(INPUT);
        assert_eq!(part_one(&lines), 544_664);
    }

    #[test]
    fn test_part_two() {
        let lines = parse_input(INPUT);
        assert_eq!(part_two(&lines), 84_495_585);
    }
}
