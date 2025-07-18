const INPUT: &str = include_str!("../input");

fn main() {
    let bingo = parse_input(INPUT);
    println!("{}", part_one(&bingo));
    println!("{}", part_two(&bingo));
}

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone, Default)]
struct Board {
    numbers: [u32; 25],
    marked: [bool; 25],
    marks_by_column: [u8; 5],
    marks_by_row: [u8; 5],
}

impl Board {
    fn is_winner(&self) -> bool {
        self.marks_by_column
            .iter()
            .chain(self.marks_by_row.iter())
            .any(|&c| c == 5)
    }

    fn unmarked_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .zip(self.marked.iter())
            .filter_map(|(&n, &mark)| if !mark { Some(n) } else { None })
            .collect()
    }

    fn mark_number(&mut self, marked_number: u32) {
        for i in 0..25 {
            let number = self.numbers[i];
            let is_marked = self.marked[i];
            if number == marked_number && !is_marked {
                self.marked[i] = true;
                let column = i % 5;
                self.marks_by_column[column] += 1;
                let row = i / 5;
                self.marks_by_row[row] += 1;
            }
        }
    }
}

fn parse_input(s: &str) -> Bingo {
    let mut lines = s.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next();
    let mut boards = vec![];
    while let Some(l) = lines.next() {
        let mut board = Board::default();
        let values: Vec<_> = [
            l,
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        ]
        .iter()
        .flat_map(|s| s.split_whitespace().map(|v| v.parse().unwrap()))
        .collect();
        board.numbers.copy_from_slice(&values);
        boards.push(board);
        lines.next();
    }
    Bingo { numbers, boards }
}

fn part_one(bingo: &Bingo) -> u32 {
    let mut bingo = bingo.clone();
    for n in bingo.numbers {
        for board in bingo.boards.iter_mut() {
            board.mark_number(n);
            if board.is_winner() {
                return board.unmarked_numbers().iter().sum::<u32>() * n;
            }
        }
    }
    panic!("no winner")
}

fn part_two(bingo: &Bingo) -> u32 {
    let mut bingo = bingo.clone();
    let mut last_winner = None;
    for n in bingo.numbers {
        for board in bingo.boards.iter_mut() {
            if board.is_winner() {
                continue;
            }
            board.mark_number(n);
            if board.is_winner() {
                last_winner = Some((n, board.clone()));
            }
        }
    }
    let (last_winning_number, last_winning_board) = last_winner.unwrap();
    last_winning_number * last_winning_board.unmarked_numbers().iter().sum::<u32>()
}
