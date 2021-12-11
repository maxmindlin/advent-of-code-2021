use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    spaces: Vec<Vec<u32>>,
    height: usize,
    width: usize,
    exploded: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Pt {
    x: usize,
    y: usize,
}

fn main() {
    let i = std::fs::read_to_string("input/1.txt").unwrap();
    let mut g = Grid::from(i.as_str());
    println!("ANSWER: {}", g.find_sync());
}

impl Grid {
    fn incr(&mut self) {
        let mut exploded: HashSet<Pt> = HashSet::new();
        let mut next: Vec<Pt> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pt = Pt::new(x, y);
                if !exploded.contains(&pt) {
                    let mut val = self.spaces[y][x] + 1;
                    if val > 9 {
                        exploded.insert(pt);
                        val = 0;
                        self.check_adj(&mut next, &pt);
                    }
                    self.spaces[y][x] = val;
                }
            }
        }

        while !next.is_empty() {
            let p = next.pop().unwrap();
            if !exploded.contains(&p) {
                let mut val = self.spaces[p.y][p.x] + 1;
                if val > 9 {
                    exploded.insert(p);
                    val = 0;
                    self.check_adj(&mut next, &p);
                }
                self.spaces[p.y][p.x] = val;
            }
        }

        self.exploded += exploded.len();
    }

    fn incr_n(&mut self, n: usize) {
        for _ in 0..n {
            self.incr()
        }
    }

    fn find_sync(&mut self) -> usize {
        let mut i = 0;
        let t = self.width * self.height;
        while self.exploded != t {
            self.exploded = 0;
            i += 1;
            self.incr();
        }
        i
    }

    fn check_adj(&self, buf: &mut Vec<Pt>, pt: &Pt) {
        if pt.y > 0 {
            buf.push(Pt::new(pt.x, pt.y - 1));
        }
        if pt.y < self.height - 1 {
            buf.push(Pt::new(pt.x, pt.y + 1));
        }
        if pt.x > 0 {
            buf.push(Pt::new(pt.x - 1, pt.y));
        }
        if pt.x < self.width - 1 {
            buf.push(Pt::new(pt.x + 1, pt.y));
        }
        if pt.y > 0 && pt.x > 0 {
            buf.push(Pt::new(pt.x - 1, pt.y - 1));
        }
        if pt.y > 0 && pt.x < self.width - 1 {
            buf.push(Pt::new(pt.x + 1, pt.y - 1));
        }
        if pt.y < self.height - 1 && pt.x < self.width - 1 {
            buf.push(Pt::new(pt.x + 1, pt.y + 1));
        }
        if pt.y < self.height - 1 && pt.x > 0 {
            buf.push(Pt::new(pt.x - 1, pt.y + 1));
        }
    }
}

impl Pt {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let m: Vec<Vec<u32>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let height = m.len();
        let width = m[0].len();
        Self {
            spaces: m,
            width,
            height,
            exploded: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single() {
        let i = "11111
19991
19191
19991
11111";
        let mut g = Grid::from(i);
        g.incr();
        assert_eq!(g.exploded, 9);
    }

    #[test]
    fn base() {
        let i = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut g = Grid::from(i);
        g.incr_n(10);
        assert_eq!(g.exploded, 204);
        g.incr_n(90);
        assert_eq!(g.exploded, 1656);
    }

    #[test]
    fn base_sync() {
        let i = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut g = Grid::from(i);
        assert_eq!(g.find_sync(), 195);
    }
}
