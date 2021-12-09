use std::{collections::HashSet, iter::FromIterator};

fn main() {
    let i = std::fs::read_to_string("input/1.txt").unwrap();
    let v = parse_input(&i);
    println!("ANSWER: {}", solve(&v));
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve(grid: &[Vec<u32>]) -> usize {
    let mut b: Vec<usize> = basins(grid).iter().map(|h| h.len()).collect();
    b.sort_by(|a, b| b.cmp(&a));
    b[0..3].iter().fold(1, |acc, v| acc * v)
}

fn basins(grid: &[Vec<u32>]) -> Vec<HashSet<(usize, usize)>> {
    let mut basins = Vec::new();
    let h = grid.len();
    let w = grid[0].len();
    for y in 0..h {
        for x in 0..w {
            if low_point(grid, x, y, h, w) {
                let b = basin(grid, (y, x), h, w);
                basins.push(b);
            }
        }
    }

    basins
}

fn basin(grid: &[Vec<u32>], start: (usize, usize), height: usize, width: usize) -> HashSet<(usize, usize)> {
    let mut basin = HashSet::new();
    basin.insert(start);
    let mut next = higher_pts(grid, start.1, start.0, height, width);
    while !next.is_empty() {
        let next_set: HashSet<(usize, usize)> = HashSet::from_iter(next.iter().cloned());
        basin.extend(&next_set);
        let mut tmp = Vec::new();
        for pt in next {
            tmp.append(&mut higher_pts(grid, pt.1, pt.0, height, width));
        }
        next = tmp;
    }

    basin
}

fn higher_pts(grid: &[Vec<u32>], x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let val = grid[y][x];
    let mut highers: Vec<(usize, usize)> = Vec::new();
    if x != 0 {
        let comp = grid[y][x - 1];
        if comp > val && comp != 9 {
            highers.push((y, x - 1));
        }
    }
    if x != width - 1 {
        let comp = grid[y][x + 1];
        if comp > val && comp != 9 {
            highers.push((y, x + 1));
        }
    }
    if y != 0 {
        let comp = grid[y - 1][x];
        if comp > val && comp != 9 {
            highers.push((y - 1, x));
        }
    }
    if y != height - 1 {
        let comp = grid[y + 1][x];
        if comp > val && comp != 9 {
            highers.push((y + 1, x));
        }
    }
    highers
}

fn low_point(grid: &[Vec<u32>], x: usize, y: usize, height: usize, width: usize) -> bool {
    let val = grid[y][x];
    if x != 0 {
        if grid[y][x - 1] <= val { return false; }
    }
    if x != width - 1 {
        if grid[y][x + 1] <= val { return false; }
    }
    if y != 0 {
        if grid[y - 1][x] <= val { return false; }
    }
    if y != height - 1 {
        if grid[y + 1][x] <= val { return false; }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let i = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let v = parse_input(i);
        assert_eq!(solve(&v), 1134);
    }
}
