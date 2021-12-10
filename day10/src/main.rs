#[derive(Debug, PartialEq, Eq)]
enum LineResult {
    Error(char),
    Incomplete(Vec<char>),
}

fn main() {
    let i = std::fs::read_to_string("input/1.txt").unwrap();
    println!("ANSWER 1: {}", solve1(&i));
    println!("ANSWER 2: {}", solve2(&i));
}

fn solve1(input: &str) -> usize {
    let cost = |c: char| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown char"),
    };
    let mut errors: Vec<char> = Vec::new();
    for l in input.lines() {
        if let LineResult::Error(err) = check_line(l) {
            errors.push(err);
        }
    }

    errors.iter().map(|c| cost(*c)).sum()
}

fn solve2(input: &str) -> usize {
    let mut scores = Vec::new();
    for l in input.lines() {
        if let LineResult::Incomplete(v) = check_line(l) {
            scores.push(calc_incomplete_score(v));
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn calc_incomplete_score(v: Vec<char>) -> usize {
    let cost = |c: char| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unknown char"),
    };
    v.iter()
        .rev()
        .map(|c| cost(closing(*c)))
        .fold(0, |acc, v| acc * 5 + v)
}

fn check_line(l: &str) -> LineResult {
    let mut openings: Vec<char> = Vec::new();
    for c in l.chars() {
        if is_opener(c) {
            openings.push(c);
        } else {
            if let Some(o) = openings.pop() {
                if closing(o) != c {
                    return LineResult::Error(c);
                }
            } else {
                return LineResult::Error(c);
            }
        }
    }

    LineResult::Incomplete(openings)
}

fn closing(c: char) -> char {
    match c {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        '<' => '>',
        _ => panic!("unknown char {}", c),
    }
}

fn is_opener(c: char) -> bool {
    match c {
        '[' | '{' | '(' | '<' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line() {
        let i = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(check_line(i), LineResult::Error('}'));
        let i = "[[<[([]))<([[{}[[()]]]";
        assert_eq!(check_line(i), LineResult::Error(')'));
        let i = "[{[{({}]{}}([{[{{{}}([]";
        assert_eq!(check_line(i), LineResult::Error(']'));
    }

    #[test]
    fn base() {
        let i = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(solve1(i), 26397);
        assert_eq!(solve2(i), 288957);
    }
}
