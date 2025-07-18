use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

fn main() {
    let mut input = INPUT.split("\n\n");
    let dots = DotGrid::from_input(input.next().unwrap());
    let folds: Vec<Fold> = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let fold_position = line.chars().skip(13).collect::<String>().parse().unwrap();
            match line.chars().nth(11).unwrap() {
                'x' => Fold::Horizontal(fold_position),
                'y' => Fold::Vertical(fold_position),
                _ => panic!("invalid fold"),
            }
        })
        .collect();
    println!("{}", part_one(&dots, folds[0]));
    println!("{}", part_two(&dots, &folds));
}

#[derive(Debug, Clone)]
struct DotGrid {
    width: usize,
    height: usize,
    dot_coordinates: HashSet<(usize, usize)>,
}

impl DotGrid {
    fn from_input(input: &str) -> Self {
        let mut dot_coordinates = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for line in input.lines() {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            dot_coordinates.insert((x, y));
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        Self {
            width: max_x + 1,
            height: max_y + 1,
            dot_coordinates,
        }
    }

    fn dot_count(&self) -> usize {
        self.dot_coordinates.len()
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::Horizontal(fold_position) => {
                let mut dot_coordinates = HashSet::new();
                for &(x, y) in &self.dot_coordinates {
                    if x > fold_position {
                        dot_coordinates.insert((fold_position - (x - fold_position), y));
                    } else {
                        dot_coordinates.insert((x, y));
                    }
                }
                self.width = fold_position;
                self.dot_coordinates = dot_coordinates;
            }
            Fold::Vertical(fold_position) => {
                let mut dot_coordinates = HashSet::new();
                for &(x, y) in &self.dot_coordinates {
                    if y > fold_position {
                        dot_coordinates.insert((x, fold_position - (y - fold_position)));
                    } else {
                        dot_coordinates.insert((x, y));
                    }
                }
                self.height = fold_position;
                self.dot_coordinates = dot_coordinates;
            }
        }
    }
}

impl std::fmt::Display for DotGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.dot_coordinates.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn part_one(dot_grid: &DotGrid, fold: Fold) -> usize {
    let mut dot_grid = dot_grid.clone();
    dot_grid.fold(fold);
    dot_grid.dot_count()
}

fn part_two(dot_grid: &DotGrid, folds: &[Fold]) -> DotGrid {
    let mut dot_grid = dot_grid.clone();
    for &fold in folds {
        dot_grid.fold(fold);
    }
    dot_grid
}
