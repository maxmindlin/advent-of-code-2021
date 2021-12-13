#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FoldDir {
    Up,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    pts: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Instr {
    val: usize,
    dir: FoldDir,
}

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let (mut grid, instrs) = parse_input(&input);
    grid.process(instrs);
    println!("{}", grid);
}

fn parse_input(input: &str) -> (Grid, Vec<Instr>) {
    let v: Vec<&str> = input.split("\n\n").collect();

    let grid = Grid::from(v[0]);
    let instr = v[1].lines()
        .map(Instr::from)
        .collect();

    (grid, instr)
}

impl Grid {
    fn process_single(&mut self, instr: Instr) {
        match instr.dir {
            FoldDir::Up => {
                for y in (instr.val + 1)..self.height {
                    let j = instr.val - (y - instr.val);
                    for x in 0..self.width {
                        self.pts[j][x] |= self.pts[y][x];
                    }
                }
                self.pts.drain(instr.val..);
                self.height = self.pts.len();
            }
            FoldDir::Left => {
                for y in 0..self.height {
                    for x in (instr.val + 1)..self.width {
                        let j = instr.val - (x - instr.val);
                        self.pts[y][j] |= self.pts[y][x];
                    }
                    self.pts[y].drain(instr.val..);
                }
                self.width = self.pts[0].len();
            }
        }
    }

    fn process(&mut self, instrs: Vec<Instr>) {
        for instr in instrs { self.process_single(instr); }
    }

    fn count(&self) -> usize {
        self.pts
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum()
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let mut pts: Vec<Vec<bool>> = Vec::new();
        for coord in s.lines() {
            let v: Vec<usize> = coord.split(',')
                .map(|c| c.parse().unwrap())
                .collect();

            let curr_y = pts.len();
            if v[1] >= curr_y {
                let diff = (v[1] + 1) - curr_y;
                pts.extend(vec![Vec::new(); diff]);
            }

            for row in 0..pts.len() {
                let curr_x = pts[row].len();
                if v[0] >= curr_x {
                    let diff = (v[0] + 1) - curr_x;
                    pts[row].extend(vec![false; diff]);
                }
            }

            pts[v[1]][v[0]] = true;
        }

        let height = pts.len();
        let width = pts[0].len();
        Self { pts, height, width }
    }
}

impl From<&str> for Instr {
    fn from(s: &str) -> Self {
        let v: Vec<&str> = s.split("along ").collect();
        let d: Vec<&str> = v[1].split('=').collect();
        let dir = match d[0] {
            "y" => FoldDir::Up,
            "x" => FoldDir::Left,
            _ => panic!("unknown fold dir"),
        };

        Self {
            val: d[1].parse().unwrap(),
            dir
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.pts[y][x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let i = "0,3
1,2
0,1

fold along y=1
fold along x=2";
        let (g, i) = parse_input(i);

        let exp_g = Grid{ pts: vec![
            vec![false, false],
            vec![true, false],
            vec![false, true],
            vec![true, false],
        ], width: 2, height: 4};
        let exp_i = vec![
            Instr { val: 1, dir: FoldDir::Up },
            Instr { val: 2, dir: FoldDir::Left },
        ];

        assert_eq!(g, exp_g);
        assert_eq!(i, exp_i);
    }

    #[test]
    fn base() {
        let i = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (mut g, instrs) = parse_input(i);
        g.process_single(instrs[0]);
        assert_eq!(g.count(), 17);
    }
}
