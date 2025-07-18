static INPUT: &str = include_str!("../input");

fn main() {
    let boarding_passes: Vec<_> = INPUT.lines().collect();
    let max_id = boarding_passes
        .iter()
        .map(|l| find_row(&l[..7]) * 8 + find_column(&l[7..]))
        .max()
        .unwrap();
    println!("part one: {}", max_id);
    let (seat, id) = part_two(&boarding_passes);
    println!("part two seat: {:?}\npart two id: {}", seat, id);
}

fn find_row(s: &str) -> u32 {
    let (mut min, mut max) = (0, 127);
    for c in s.chars() {
        match c {
            'F' => max = (min + max) / 2,
            'B' => min = (min + max) / 2 + 1,
            _ => panic!("invalid char"),
        }
    }
    min
}

fn find_column(s: &str) -> u32 {
    let (mut min, mut max) = (0, 7);
    for c in s.chars() {
        match c {
            'L' => max = (min + max) / 2,
            'R' => min = (min + max) / 2 + 1,
            _ => panic!("invalid char"),
        }
    }
    min
}

fn part_two(boarding_passes: &[&str]) -> ((u32, u32), u32) {
    let seats: Vec<_> = boarding_passes
        .iter()
        .map(|s| (find_row(&s[..7]), find_column(&s[7..])))
        .collect();
    let (min_seat_row, min_seat_column) = seats.iter().min().copied().unwrap();
    let (max_seat_row, max_seat_column) = seats.iter().max().copied().unwrap();

    let mut expected_row_sum = 0;
    for _ in min_seat_column..8 {
        expected_row_sum += min_seat_row;
    }
    for i in min_seat_row + 1..max_seat_row {
        expected_row_sum += i * 8;
    }
    for _ in 0..=max_seat_column {
        expected_row_sum += max_seat_row;
    }
    let actual_row_sum: u32 = seats.iter().map(|(row, _)| row).sum();
    let row = expected_row_sum - actual_row_sum;

    let mut expected_column_sum = 0;
    for _ in min_seat_row + 1..max_seat_row {
        for i in 0..8 {
            expected_column_sum += i;
        }
    }
    for i in min_seat_column..8 {
        expected_column_sum += i;
    }
    for i in 0..=max_seat_column {
        expected_column_sum += i;
    }
    let actual_column_sum: u32 = seats.iter().map(|(_, column)| column).sum();
    let column = expected_column_sum - actual_column_sum;

    ((row, column), row * 8 + column)
}
