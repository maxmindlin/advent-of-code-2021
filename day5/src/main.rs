use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Default)]
struct State {
    pts: HashMap<Point, usize>,
}

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let lines = parse_input(&input);
    let mut state = State::default();
    state.add_lines(lines);
    println!("ANSWER: {}", state.intersections());
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(Line::from)
        .collect()
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl State {
    fn add_line(&mut self, line: Line) {
        if line.p1.x == line.p2.x {
            for y in i32::min(line.p1.y, line.p2.y)..=i32::max(line.p1.y, line.p2.y) {
                let count = self.pts.entry(Point::new(line.p1.x, y)).or_insert(0);
                *count += 1;
            }
        } else if line.p1.y == line.p2.y {
            for x in i32::min(line.p1.x, line.p2.x)..=i32::max(line.p1.x, line.p2.x) {
                let count = self.pts.entry(Point::new(x, line.p1.y)).or_insert(0);
                *count += 1;
            }
        } else {
            let (mut left, right) = if line.p1.x < line.p2.x {
                (line.p1, line.p2)
            } else {
                (line.p2, line.p1)
            };

            let below = left.y < right.y;
            let count = self.pts.entry(left).or_insert(0);
            *count += 1;
            while left != right {
                let next_y = if below {
                    left.y + 1
                } else {
                    left.y - 1
                };
                let next = Point::new(left.x + 1, next_y);
                let count = self.pts.entry(next).or_insert(0);
                *count += 1;
                left = next;
            }
        }
    }

    fn add_lines(&mut self, lines: Vec<Line>) {
        for line in lines { self.add_line(line); }
    }

    fn intersections(&self) -> usize {
        self.pts.iter().filter(|(_, v)| **v > 1).count()
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let pts: Vec<&str> = s.split(" -> ").collect();
        debug_assert_eq!(pts.len(), 2);
        Self { p1: Point::from(pts[0]), p2: Point::from(pts[1]) }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let v: Vec<&str> = s.split(',').collect();
        debug_assert_eq!(v.len(), 2);
        Self::new(v[0].parse().unwrap(), v[1].parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let i = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let lines = parse_input(i);
        let mut s = State::default();
        s.add_lines(lines);
        assert_eq!(s.intersections(), 12);
    }
}
