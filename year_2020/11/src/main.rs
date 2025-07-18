use std::borrow::Cow;
use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    let layout = Layout::from_input(INPUT);
    println!("part one: {}", part_one(&layout));
    println!("part two: {}", part_two(&layout));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum SeatState {
    Empty,
    Occupied,
}

#[derive(Clone)]
struct Layout {
    seat_states: Vec<SeatState>,
    adjacent: Vec<Vec<usize>>,
    visible: Vec<Vec<usize>>,
}

impl Layout {
    fn from_input(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut seat_states = vec![];
        let mut seat_coords = vec![];
        let mut seat_index_by_coord = HashMap::new();
        for c in input.chars() {
            match c {
                '.' => x += 1,
                'L' => {
                    seat_coords.push((x, y));
                    seat_index_by_coord.insert((x, y), seat_states.len());
                    seat_states.push(SeatState::Empty);
                    x += 1;
                }
                '#' => {
                    seat_coords.push((x, y));
                    seat_index_by_coord.insert((x, y), seat_states.len());
                    seat_states.push(SeatState::Occupied);
                    x += 1;
                }
                '\n' => {
                    width = x;
                    x = 0;
                    y += 1;
                }
                _ => panic!("invalid char"),
            }
        }
        let height = y;
        let adjacent = seat_coords
            .iter()
            .map(|&(x, y)| {
                [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ]
                .iter()
                .filter_map(|c| seat_index_by_coord.get(c))
                .copied()
                .collect()
            })
            .collect();
        let visible = seat_coords
            .iter()
            .map(|&(start_x, start_y)| {
                [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .filter_map(|&(dx, dy)| {
                    let mut x = start_x + dx;
                    let mut y = start_y + dy;
                    while x >= 0
                        && x < width
                        && y >= 0
                        && y < height
                        && seat_index_by_coord.get(&(x, y)).is_none()
                    {
                        x += dx;
                        y += dy;
                    }
                    seat_index_by_coord.get(&(x, y))
                })
                .copied()
                .collect()
            })
            .collect();
        Self {
            seat_states,
            adjacent,
            visible,
        }
    }
}

fn part_one(layout: &Layout) -> u32 {
    let mut layout = layout.clone();
    loop {
        let mut new_states = Cow::from(&layout.seat_states);
        for (i, &s) in layout.seat_states.iter().enumerate() {
            let adjacent_occupied_count = layout.adjacent[i]
                .iter()
                .map(|&i| layout.seat_states[i])
                .filter(|&s| s == SeatState::Occupied)
                .count();
            if s == SeatState::Empty && adjacent_occupied_count == 0 {
                new_states.to_mut()[i] = SeatState::Occupied;
            }
            if s == SeatState::Occupied && adjacent_occupied_count >= 4 {
                new_states.to_mut()[i] = SeatState::Empty;
            }
        }
        if let Cow::Owned(v) = new_states {
            layout.seat_states = v;
        } else {
            break;
        }
    }
    layout
        .seat_states
        .iter()
        .filter(|&&s| s == SeatState::Occupied)
        .count() as u32
}

fn part_two(layout: &Layout) -> u32 {
    let mut layout = layout.clone();
    loop {
        let mut new_states = Cow::from(&layout.seat_states);
        for (i, &s) in layout.seat_states.iter().enumerate() {
            let visible_occupied_count = layout.visible[i]
                .iter()
                .map(|&i| layout.seat_states[i])
                .filter(|&s| s == SeatState::Occupied)
                .count();
            if s == SeatState::Empty && visible_occupied_count == 0 {
                new_states.to_mut()[i] = SeatState::Occupied;
            }
            if s == SeatState::Occupied && visible_occupied_count >= 5 {
                new_states.to_mut()[i] = SeatState::Empty;
            }
        }
        if let Cow::Owned(v) = new_states {
            layout.seat_states = v;
        } else {
            break;
        }
    }
    layout
        .seat_states
        .iter()
        .filter(|&&s| s == SeatState::Occupied)
        .count() as u32
}
