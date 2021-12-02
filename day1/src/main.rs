fn main() {
    let f = std::fs::read_to_string("input/1.txt").unwrap();
    let input = parse_input(&f);
    println!("ANSWER 1: {}", count(&input));
    println!("ANSWER 2: {}", count_windows(&input));
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse().unwrap())
        .collect()
}

fn count(input: &[usize]) -> usize {
    let mut curr = input[0];
    let mut count = 0;
    for i in 1..input.len() {
        let next = input[i];
        if next > curr {
            count += 1;
        }
        curr = next;
    }

    count
}

fn count_windows(input: &[usize]) -> usize {
    let mut curr = std::usize::MAX;
    let mut count = 0;
    for w in input.windows(3) {
        let next = w.iter().sum();
        if next > curr {
            count += 1;
        }
        curr = next;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given() {
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];
        assert_eq!(count(&input), 7);
    }

    #[test]
    fn given_windows() {
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];
        assert_eq!(count_windows(&input), 5);
    }
}
