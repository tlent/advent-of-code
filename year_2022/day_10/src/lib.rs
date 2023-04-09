pub const INPUT: &str = include_str!("../input.txt");

pub enum Instruction {
    Noop,
    Add(i32),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with('a') {
                Instruction::Add(line[5..].parse().unwrap())
            } else {
                Instruction::Noop
            }
        })
        .collect()
}

pub fn solve(instructions: &[Instruction]) -> (i32, String) {
    const CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    let mut display = String::new();
    let mut sprite_position = 1;
    let mut display_position = 0;
    let mut stored_add = None;
    let mut instruction_iter = instructions.iter().peekable();

    for cycle in 1.. {
        if instruction_iter.peek().is_none() {
            break;
        }
        if CYCLES.contains(&cycle) {
            sum += sprite_position * cycle;
        }

        if display_position == 40 {
            display.push('\n');
            display_position = 0;
        }
        if (display_position - sprite_position).abs() <= 1 {
            display.push('#');
        } else {
            display.push('.');
        }
        display_position += 1;

        match stored_add.take() {
            Some(v) => sprite_position += v,
            None => match instruction_iter.next() {
                Some(Instruction::Noop) => {}
                Some(Instruction::Add(v)) => {
                    stored_add = Some(*v);
                }
                None => unreachable!(),
            },
        }
    }
    (sum, display)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let instructions = parse_input(INPUT);
        let (part_one, part_two) = solve(&instructions);
        assert_eq!(part_one, 15680);
        assert_eq!(
            part_two,
            "####.####.###..####.#..#..##..#..#.###..\n\
             ...#.#....#..#.#....#..#.#..#.#..#.#..#.\n\
             ..#..###..###..###..####.#....#..#.#..#.\n\
             .#...#....#..#.#....#..#.#.##.#..#.###..\n\
             #....#....#..#.#....#..#.#..#.#..#.#....\n\
             ####.#....###..#....#..#..###..##..#...."
        );
    }
}
