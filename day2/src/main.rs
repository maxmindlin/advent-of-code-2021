enum InstrKind {
    Fwd,
    Up,
    Down,
}

struct Instr {
    kind: InstrKind,
    val: usize,
}

#[derive(Default)]
struct Pos {
    lat: usize,
    aim: usize,
    depth: usize,
}

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let instrs = parse_input(&input);

    let mut pos = Pos::default();
    pos.process_instr(&instrs);
    println!("ANSWER: {}", pos.lat * pos.depth);
}

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(Instr::from)
        .collect()
}

impl Pos {
    fn incr(&mut self, instr: &Instr) {
        match instr.kind {
            InstrKind::Fwd => {
                self.lat += instr.val;
                self.depth += self.aim * instr.val;
            },
            InstrKind::Up => { self.aim -= instr.val },
            InstrKind::Down => {self.aim += instr.val },
        };
    }

    fn process_instr(&mut self, instrs: &[Instr]) {
        instrs.iter().for_each(|i| self.incr(i));
    }
}

impl From<&str> for Instr {
    fn from(s: &str) -> Self {
        let v = s.split_whitespace().collect::<Vec<&str>>();
        Self {
            kind: InstrKind::from(v[0]),
            val: v[1].parse().unwrap()
        }
    }
}

impl From<&str> for InstrKind {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Fwd,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("unknown instruction {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let i = parse_input(&s);
        let mut pos = Pos::default();
        pos.process_instr(&i);
        assert_eq!(pos.depth, 60);
        assert_eq!(pos.lat, 15);
    }
}
