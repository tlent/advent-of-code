use std::collections::{HashMap, HashSet};
use std::vec::IntoIter;

static INPUT: &str = include_str!("../input");
const SEA_MONSTER_PATTERN: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

fn main() {
    let tiles = parse_input(INPUT);
    println!("part one: {}", part_one(&tiles));
    println!("part two: {}", part_two(&tiles));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Tile {
    id: u32,
    lines: Vec<String>,
}

impl Tile {
    fn top(&self) -> String {
        self.lines[0].to_string()
    }

    fn left(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.chars().nth(0).unwrap())
            .collect()
    }

    fn right(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.chars().nth(l.len() - 1).unwrap())
            .collect()
    }

    fn bottom(&self) -> String {
        self.lines[self.lines.len() - 1].to_string()
    }

    fn flip_horizontal(&self) -> Self {
        let lines = flip_horizontal(&self.lines);
        Self { id: self.id, lines }
    }

    fn flip_vertical(&self) -> Self {
        let lines = flip_vertical(&self.lines);
        Self { id: self.id, lines }
    }

    fn flip_both(&self) -> Self {
        self.flip_horizontal().flip_vertical()
    }

    fn rotate_right(&self) -> Self {
        let lines = rotate_right(&self.lines);
        Self { id: self.id, lines }
    }

    fn all_variants(&self) -> IntoIter<Self> {
        vec![self.clone(), self.rotate_right()]
            .into_iter()
            .map(|t| vec![t.flip_horizontal(), t.flip_vertical(), t.flip_both(), t])
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn flip_horizontal(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|l| l.chars().rev().collect::<String>())
        .collect()
}

fn flip_vertical(lines: &[String]) -> Vec<String> {
    lines.iter().rev().cloned().collect()
}

fn flip_both(lines: &[String]) -> Vec<String> {
    flip_horizontal(&flip_vertical(lines))
}

fn rotate_right(lines: &[String]) -> Vec<String> {
    let width = lines[0].len();
    (0..width)
        .map(|i| {
            lines
                .iter()
                .rev()
                .map(|l| l.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .trim()
        .split("\n\n")
        .map(|t| {
            let mut lines = t.lines();
            let id = lines
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            let lines = lines.map(String::from).collect();
            Tile { id, lines }
        })
        .collect()
}

fn reassemble_tiles(tiles: &[Tile]) -> Vec<Tile> {
    let tile_count = tiles.len();
    let size = ((tile_count as f32).sqrt()) as usize;
    let tile_variants: Vec<_> = tiles.iter().map(|t| t.all_variants()).flatten().collect();
    let mut variants_by_top: HashMap<String, HashSet<&Tile>> = HashMap::new();
    let mut variants_by_left: HashMap<String, HashSet<&Tile>> = HashMap::new();
    for v in &tile_variants {
        variants_by_top.entry(v.top()).or_default().insert(v);
        variants_by_left.entry(v.left()).or_default().insert(v);
    }
    let mut stack: Vec<_> = tile_variants
        .iter()
        .map(|v| (vec![v], vec![v.id].into_iter().collect::<HashSet<_>>()))
        .collect();
    while let Some((solution, used_ids)) = stack.pop() {
        let i = solution.len();
        if i == tile_count {
            return solution.into_iter().cloned().collect();
        }
        let left_tile = if i % size == 0 {
            None
        } else {
            Some(solution[i - 1])
        };
        let above_tile = if i < size {
            None
        } else {
            Some(solution[i - size])
        };
        let mut candidates: Vec<_> = match (left_tile, above_tile) {
            (Some(l), Some(a)) => variants_by_left[&l.right()]
                .intersection(&variants_by_top[&a.bottom()])
                .collect(),
            (Some(l), None) => variants_by_left[&l.right()].iter().collect(),
            (None, Some(a)) => variants_by_top[&a.bottom()].iter().collect(),
            (None, None) => unreachable!(),
        };
        candidates.retain(|t| !used_ids.contains(&t.id));
        for c in candidates {
            let mut solution = solution.clone();
            solution.push(c);
            let mut used_ids = used_ids.clone();
            used_ids.insert(c.id);
            stack.push((solution, used_ids));
        }
    }
    panic!("no solution");
}

fn part_one(tiles: &[Tile]) -> u64 {
    let tile_count = tiles.len();
    let size = ((tile_count as f32).sqrt()) as usize;
    let reassembled_tiles = reassemble_tiles(tiles);
    [
        &reassembled_tiles[0],
        &reassembled_tiles[size - 1],
        &reassembled_tiles[tile_count - size],
        &reassembled_tiles[tile_count - 1],
    ]
    .iter()
    .map(|t| t.id as u64)
    .product()
}

fn reassemble_image(tiles: &[Tile]) -> Vec<String> {
    let tile_count = tiles.len();
    let size = ((tile_count as f32).sqrt()) as usize;
    let trimmed_tiles: Vec<Vec<_>> = tiles
        .iter()
        .map(|t| {
            t.lines[1..t.lines.len() - 1]
                .iter()
                .map(|l| String::from(&l[1..l.len() - 1]))
                .collect()
        })
        .collect();
    let tile_line_count = trimmed_tiles[0].len();
    let mut image = vec![];
    let mut current_row = vec![String::new(); tile_line_count];
    for (i, t) in trimmed_tiles.into_iter().enumerate() {
        if i > 0 && i % size == 0 {
            image.extend(current_row);
            current_row = vec![String::new(); tile_line_count];
        }
        for (dest, src) in current_row.iter_mut().zip(t.iter()) {
            dest.push_str(src);
        }
    }
    image.extend(current_row);
    image
}

fn monster_pound_count(image: &[String]) -> u32 {
    let monster_height = SEA_MONSTER_PATTERN.len();
    let monster_width = SEA_MONSTER_PATTERN[0].len();
    let image_height = image.len();
    let image_width = image[0].len();
    let mut monster_pound_offsets = vec![];
    for (i, line) in SEA_MONSTER_PATTERN.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                monster_pound_offsets.push((i, j));
            }
        }
    }
    let mut monster_pound_coords = HashSet::new();
    for i in 0..(image_height - monster_height) {
        for j in 0..(image_width - monster_width) {
            let mut matches = true;
            for (image_line, &monster_line) in image[i..i + monster_height]
                .iter()
                .zip(SEA_MONSTER_PATTERN.iter())
            {
                if !equals_monster_line(&image_line[j..j + monster_width], monster_line) {
                    matches = false;
                }
            }
            if matches {
                for (i_offset, j_offset) in monster_pound_offsets.iter() {
                    monster_pound_coords.insert((i + i_offset, j + j_offset));
                }
            }
        }
    }
    monster_pound_coords.len() as u32
}

fn equals_monster_line(image_line: &str, monster_line: &str) -> bool {
    for (i, m) in image_line.chars().zip(monster_line.chars()) {
        if m == ' ' {
            continue;
        }
        if i != m {
            return false;
        }
    }
    true
}

fn part_two(tiles: &[Tile]) -> u32 {
    let reassembled_tiles = reassemble_tiles(tiles);
    let image = reassemble_image(&reassembled_tiles);
    let reoriented_image = vec![rotate_right(&image), image]
        .into_iter()
        .map(|i| vec![flip_horizontal(&i), flip_vertical(&i), flip_both(&i), i].into_iter())
        .flatten()
        .find(|i| monster_pound_count(&i) > 0)
        .unwrap();
    let pound_count: u32 = reoriented_image
        .iter()
        .map(|l| l.chars().filter(|&c| c == '#').count() as u32)
        .sum();
    pound_count - monster_pound_count(&reoriented_image)
}
