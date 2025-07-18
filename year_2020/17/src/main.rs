use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    let mut grid = Grid::from_input(INPUT, 3);
    for _ in 0..6 {
        grid.cycle();
    }
    println!("part one: {}", grid.active_coordinates.len());
    let mut grid = Grid::from_input(INPUT, 4);
    for _ in 0..6 {
        grid.cycle();
    }
    println!("part two: {}", grid.active_coordinates.len());
}

type Coordinate = Vec<i64>;

#[derive(Debug, Clone)]
struct Grid {
    active_coordinates: HashSet<Coordinate>,
    active_neighbors: HashMap<Coordinate, u8>,
}

impl Grid {
    fn from_input(input: &str, dimensions: usize) -> Self {
        let mut active_coordinates = HashSet::new();
        let mut active_neighbors = HashMap::new();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    let mut coordinate = vec![0; dimensions];
                    coordinate[0] = x as i64;
                    coordinate[1] = y as i64;
                    active_coordinates.insert(coordinate.clone());
                    let neighbors = neighboring_coordinates(&coordinate);
                    for c in neighbors {
                        *active_neighbors.entry(c).or_insert(0) += 1;
                    }
                }
            }
        }
        Self {
            active_coordinates,
            active_neighbors,
        }
    }

    fn cycle(&mut self) {
        let mut new = self.clone();
        for c in &self.active_coordinates {
            let active_neighbors = self.active_neighbors.get(c).copied().unwrap_or(0);
            if active_neighbors != 2 && active_neighbors != 3 {
                new.active_coordinates.remove(c);
                let neighbors = neighboring_coordinates(&c);
                for c in neighbors {
                    *new.active_neighbors.get_mut(&c).unwrap() -= 1;
                }
            }
        }
        let inactive_to_active = self
            .active_neighbors
            .iter()
            .filter(|&(c, &n)| !self.active_coordinates.contains(c) && n == 3)
            .map(|(c, _)| c);
        for c in inactive_to_active {
            new.active_coordinates.insert(c.clone());
            let neighbors = neighboring_coordinates(&c);
            for c in neighbors {
                *new.active_neighbors.entry(c).or_insert(0) += 1;
            }
        }
        *self = new;
    }
}

fn neighboring_coordinates(c: &Coordinate) -> Vec<Coordinate> {
    let mut neighbors: Vec<_> = ((c[0] - 1)..=(c[0] + 1)).map(|v| vec![v]).collect();
    for i in 1..c.len() {
        let mut new_neighbors = Vec::with_capacity(neighbors.len() * 3);
        for n in neighbors {
            for v in (c[i] - 1)..=(c[i] + 1) {
                let mut new_coord = n.clone();
                new_coord.push(v);
                if new_coord != *c {
                    new_neighbors.push(new_coord);
                }
            }
        }
        neighbors = new_neighbors;
    }
    neighbors
}
