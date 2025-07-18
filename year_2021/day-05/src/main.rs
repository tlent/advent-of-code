const INPUT: &str = include_str!("../input");

fn main() {
    let line_segments: Vec<_> = INPUT.lines().map(LineSegment::from_str).collect();
    println!("{}", part_one(&line_segments));
    println!("{}", part_two(&line_segments));
}

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl LineSegment {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(" -> ");
        let mut start = parts.next().unwrap().split(',');
        let start_x = start.next().unwrap().parse().unwrap();
        let start_y = start.next().unwrap().parse().unwrap();
        let mut end = parts.next().unwrap().split(',');
        let end_x = end.next().unwrap().parse().unwrap();
        let end_y = end.next().unwrap().parse().unwrap();
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start_y == self.end_y
    }

    fn is_vertical(&self) -> bool {
        self.start_x == self.end_x
    }

    fn points_covered(&self) -> Vec<(u32, u32)> {
        let x_values = if self.start_x < self.end_x {
            self.start_x..=self.end_x
        } else {
            self.end_x..=self.start_x
        };
        let y_values = if self.start_y < self.end_y {
            self.start_y..=self.end_y
        } else {
            self.end_y..=self.start_y
        };
        if self.is_horizontal() {
            x_values.map(|x| (x, self.start_y)).collect()
        } else if self.is_vertical() {
            y_values.map(|y| (self.start_x, y)).collect()
        } else if self.start_x > self.end_x && self.start_y > self.end_y {
            x_values.rev().zip(y_values.rev()).collect()
        } else if self.start_x > self.end_x {
            x_values.rev().zip(y_values).collect()
        } else if self.start_y > self.end_y {
            x_values.zip(y_values.rev()).collect()
        } else {
            x_values.zip(y_values).collect()
        }
    }
}

fn part_one(line_segments: &[LineSegment]) -> u32 {
    let line_segments: Vec<_> = line_segments
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .copied()
        .collect();
    count_overlaps(&line_segments)
}

fn part_two(line_segments: &[LineSegment]) -> u32 {
    count_overlaps(line_segments)
}

fn count_overlaps(line_segments: &[LineSegment]) -> u32 {
    let points: Vec<_> = line_segments
        .iter()
        .flat_map(|l| l.points_covered())
        .collect();
    let width = *points.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = *points.iter().map(|(_, y)| y).max().unwrap() + 1;
    let size = (width * height) as usize;
    let mut covered_by_count = vec![0; size];
    for (x, y) in points {
        let i = y * width + x;
        covered_by_count[i as usize] += 1;
    }
    covered_by_count.into_iter().filter(|&c| c >= 2).count() as u32
}
