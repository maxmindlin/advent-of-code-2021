use std::collections::HashMap;

struct Pond {
    groups: HashMap<usize, usize>,
}

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let mut p = Pond::from(input.as_str());
    p.incr_n(256);
    println!("ANSWER: {}", p.len());
}

impl Pond {
    fn incr(&mut self) {
        let mut new = 0;
        for i in 0..=8 {
            if i == 0 {
                new = *self.groups.get(&i).unwrap_or(&0);
            } else {
                let count = *self.groups.get(&i).unwrap_or(&0);
                self.groups.insert(i - 1, count);
            }
            self.groups.insert(i, 0);
        }
        let resets = self.groups.entry(6).or_insert(0);
        *resets += new;
        let news = self.groups.entry(8).or_insert(0);
        *news += new;
    }

    fn incr_n(&mut self, n: usize) {
        for _ in 0..n { self.incr(); }
    }

    fn len(&self) -> usize {
        self.groups.values().sum()
    }
}

impl From<&str> for Pond {
    fn from(s: &str) -> Self {
        let mut groups = HashMap::new();

        for c in s.split(',').map(|c| c.replace("\n", "")) {
            let v: usize = c.parse().unwrap();
            let entry = groups.entry(v).or_insert(0);
            *entry += 1;
        }

        Self { groups }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let i = "3,4,3,1,2";
        let mut p = Pond::from(i);
        p.incr_n(18);
        assert_eq!(p.len(), 26);
        p.incr_n(62);
        assert_eq!(p.len(), 5934);
    }
}
