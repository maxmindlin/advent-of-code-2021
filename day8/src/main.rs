use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Note {
    signals: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let i = std::fs::read_to_string("input/1.txt").unwrap();
    let n = parse_input(&i);
    println!("ANSWER: {}", solve(&n));
}

fn parse_input(input: &str) -> Vec<Note> {
    input
        .lines()
        .map(Note::from)
        .collect()
}

fn solve(notes: &[Note]) -> usize {
    notes.iter()
        .map(solve_row)
        .sum()
}

fn solve_row(note: &Note) -> usize {
    let mut dict: HashMap<String, usize> = HashMap::new();
    let mut reverse: HashMap<usize, String> = HashMap::new();
    for sig in &note.signals {
        let num = match sig.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        };
        if let Some(n) = num {
            dict.insert(sig.clone(), n);
            reverse.insert(n, sig.clone());
        }
    }

    // We can figure out every other clock position
    // with just one and four, along with the length of the signal.
    let one: HashSet<char> = reverse.get(&1).unwrap().chars().collect();
    let four: HashSet<char> = reverse.get(&4).unwrap().chars().collect();
    for sig in &note.signals {
        let set: HashSet<char> = sig.chars().collect();
        let n: Option<usize> = match sig.len() {
            5 => {
                match ((&set & &one).len(), (&set & &four).len()) {
                    (2, _) => Some(3),
                    (_, 3) => Some(5),
                    _ => Some(2)
                }
            },
            6 => {
                match ((&set & &one).len(), (&set & &four).len()) {
                    (1, _) => Some(6),
                    (_, 4) => Some(9),
                    _ => Some(0),
                }
            },
            _ => {
                None
            }
        };

        if let Some(n) = n {
            dict.insert(sig.clone(), n);
        }
    }

    let mut ans: Vec<usize> = Vec::new();
    'outer: for out in &note.output {
        for (k, v) in dict.iter() {
            if unordered_equal(out.chars().collect(), k.chars().collect()) {
                ans.push(*v);
                continue 'outer;
            }
        }
    }

    ans.iter()
        .map(|n| format!("{}", n))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn unordered_equal(x: Vec<char>, y: Vec<char>) -> bool {
    if x.len() != y.len() { return false; }
    for a in x.iter() {
        if !y.contains(a) {
            return false;
        }
    }

    true
}

impl From<&str> for Note {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" | ").collect();
        debug_assert_eq!(split.len(), 2);
        Self {
            signals: split[0].split_whitespace().map(String::from).collect(),
            output: split[1].split_whitespace().map(String::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let i = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let n = Note::from(i);
        assert_eq!(solve_row(&n), 5353);
    }

    #[test]
    fn base2() {
        let i = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let n = parse_input(i);
        assert_eq!(solve(&n), 61229);
    }
}
