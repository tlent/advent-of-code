pub const INPUT: &str = include_str!("../input.txt");

pub fn solve(input: &str) -> (u32, u32) {
    let mut part_one_sum = 0;
    let mut part_two_sum = 0;
    let iter = input.as_bytes().chunks_exact(4); // 4 bytes per line "A X\n"
    for chunk in iter {
        let (part_one_score, part_two_score) = match (chunk[0], chunk[2]) {
            // Opponent: Paper, part one: Rock(1) -> Lose(0), part two: Lose(0) -> Rock(1)
            (b'B', b'X') => (1, 1),

            // Opponent: Scissors, part one: Paper(2) -> Lose(0), part two: Draw(3) -> Scissors(3)
            (b'C', b'Y') => (2, 6),

            // Opponent: Rock, part one: Scissors(3) -> Lose(0), part two: Win(6) -> Paper(2)
            (b'A', b'Z') => (3, 8),

            // Opponent: Rock, part one: Rock(1) -> Draw(3), part two: Lose(0) -> Scissors(3)
            (b'A', b'X') => (4, 3),

            // Opponent: Paper, part one: Paper(2) -> Draw(3), part two: Draw(3) -> Paper(2)
            (b'B', b'Y') => (5, 5),

            // Opponent: Scissors, part one: Scissors(3) -> Draw(3), part two: Win(6) -> Rock(1)
            (b'C', b'Z') => (6, 7),

            // Opponent: Scissors, part one: Rock(1) -> Win(6), part two: Lose(0) -> Paper(2)
            (b'C', b'X') => (7, 2),

            // Opponent: Rock, part one: Paper(2) -> Win(6), part two: Draw(3) -> Rock(1)
            (b'A', b'Y') => (8, 4),

            // Opponent: Paper, part one: Scissors(3) -> Win(6), part two: Win(6) -> Scissors(3)
            (b'B', b'Z') => (9, 9),

            _ => panic!("Invalid round"),
        };
        part_one_sum += part_one_score;
        part_two_sum += part_two_score;
    }
    (part_one_sum, part_two_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), (14_531, 11_258));
    }
}
