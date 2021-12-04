use std::{collections::{HashMap, HashSet}, iter::FromIterator};

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    println!("ANSWER 1: {}", play(&input, 5));
    println!("ANSWER 2: {}", play_loser(&input, 5));
}

const WINS: [i32; 10] = [
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001,
];

#[derive(Default, Clone, PartialEq, Eq)]
struct Board {
    spaces: i32,
    indexes: HashMap<i32, (usize, usize)>,
    len: usize,
}

fn parse_input(input: &str, len: usize) -> (Vec<i32>, Vec<Board>) {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    let pulls: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut temp: Vec<Vec<i32>> = Vec::new();
    for i in 1..lines.len() {
        let row: Vec<i32> = lines[i]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        temp.push(row);
        if temp.len() == len {
            boards.push(Board::new(temp));
            temp = Vec::new();
        }
    }

    (pulls, boards)
}

fn play(input: &str, len: usize) -> i32 {
    let (pulls, mut boards) = parse_input(input, len);
    for pull in pulls {
        for board in &mut boards {
            board.mark(pull);
            if board.win() {
                let sum: i32 = board
                    .unmarked()
                    .iter()
                    .cloned()
                    .sum();
                return sum * pull;
            }
        }
    }

    0
}

fn play_loser(input: &str, len: usize) -> i32 {
    let (pulls, mut boards) = parse_input(input, len);
    let mut remaining: HashSet<usize> = HashSet::from_iter(0..(boards.len() - 1));
    for pull in pulls {
        for (i, board) in &mut boards.iter_mut().enumerate() {
            if !remaining.contains(&i) {
                continue;
            }

            board.mark(pull);
            if board.win() {
                remaining.remove(&i);
                if remaining.is_empty() {
                    let sum: i32 = board
                        .unmarked()
                        .iter()
                        .cloned()
                        .sum();
                    return sum * pull;
                }
            }
        }
    }

    0
}

impl Board {
    fn new(nums: Vec<Vec<i32>>) -> Self {
        let mut indexes = HashMap::new();

        for (i, row) in nums.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                indexes.insert(*val, (i, j));
            }
        }

        Self {
            len: nums.len(),
            indexes,
            ..Default::default()
        }
    }

    fn mark(&mut self, num: i32) {
        if let Some((row, col)) = self.indexes.get(&num) {
            let idx = row * self.len + col;
            let diff = self.len * self.len - (idx + 1);
            self.spaces |= 1 << diff;
            self.indexes.remove(&num);
        }
    }

    fn win(&self) -> bool {
        for win in WINS {
            if self.spaces & win == win {
                return true;
            }
        }
        false
    }

    fn unmarked(&self) -> Vec<&i32> {
        self.indexes.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(play(input, 5), 4512);
    }
}
