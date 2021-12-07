use std::iter::FromIterator;

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let mut v = parse_input(&input);
    println!("ANSWER: {}", sum(&mut v));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|c| c.replace("\n", "").parse().unwrap())
        .collect()
}

fn sum(v: &mut [i32]) -> i32 {
    let m = v.iter().max().unwrap();
    let n = v.iter().min().unwrap();
    let z = Vec::from_iter(*n..=*m);
    let mut idx = z.len() / 2;
    let mut prev = i32::MAX;
    let mut rev = false;
    loop {
        if !rev {
            let temp = distance(v, z[idx]);
            if temp < prev {
                idx += 1;
                prev = temp;
            } else {
                idx = (z.len() / 2) - 1;
                rev = true;
            }
        } else {
            let temp = distance(v, z[idx]);
            if temp < prev {
                idx -= 1;
                prev = temp;
            } else {
                break;
            }
        }
    }

    prev
}

fn distance(v: &[i32], from: i32) -> i32 {
    v.iter()
        .map(|v| tri_number((v - from).abs()))
        .sum()
}

fn tri_number(n: i32) -> i32 {
    (n*(n+1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let i = "16,1,2,0,4,2,7,1,2,14\n";
        let mut v = parse_input(i);
        assert_eq!(sum(&mut v), 168);
    }
}
