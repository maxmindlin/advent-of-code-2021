#[derive(Clone, Debug)]
struct BitArray(Vec<bool>);

fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    let parsed = parse_input(&input);

    println!("ANSWER 1: {}", calc_consumption(&parsed));
    println!("ANSWER 2: {}", calc_life_support(&parsed));
}

fn parse_input(input: &str) -> Vec<BitArray> {
    input
        .lines()
        .map(BitArray::from)
        .collect()
}

fn calc_consumption(arrays: &[BitArray]) -> usize {
    let b = calc_common_bit_array(arrays);
    b.inverse() * b
}

fn calc_life_support(arrays: &[BitArray]) -> usize {
    let oxy = filter(arrays, false);
    let co2 = filter(arrays, true);
    oxy * co2
}

fn filter(arrays: &[BitArray], invert: bool) -> BitArray {
    let mut temp = arrays.to_vec();
    let len = arrays[0].len();
    for i in 0..len {
        let mut filter = calc_common_bit_array(&temp);
        if invert { filter = filter.inverse(); }
        temp = temp.iter().filter(|a| a[i] == filter[i]).cloned().collect();
        if temp.len() == 1 { break; }
    }

    temp[0].clone()
}

fn calc_common_bit_array(arrays: &[BitArray]) -> BitArray {
    let len = arrays[0].len(); // assume all same len
    let mut commons = Vec::new();
    for i in 0..len {
        let mut t = 0;
        let mut f = 0;
        for row in arrays.iter() {
            if row[i] { t += 1 }
            else { f += 1};
        }
        commons.push(t >= f);
    }

    BitArray::from(commons)
}

impl BitArray {
    fn inverse(&self) -> Self {
        let mut inverted = Vec::new();
        for b in &self.0 {
            inverted.push(!b);
        }

        Self(inverted)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn to_string(&self) -> String {
        let mut bits = Vec::new();
        for b in &self.0 {
            match b {
                true => bits.push('1'),
                false => bits.push('0'),
            }
        }

        bits.iter().collect()
    }
}

impl From<&str> for BitArray {
    fn from(s: &str) -> Self {
        let mut v = Vec::new();
        for c in s.chars() {
            match c {
                '1' => v.push(true),
                '0' => v.push(false),
                _ => panic!("invalid bit {}", c),
            }
        }
        Self(v)
    }
}

impl From<Vec<bool>> for BitArray {
    fn from(v: Vec<bool>) -> Self {
        Self(v)
    }
}

impl std::ops::Index<usize> for BitArray {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::Mul for BitArray {
    type Output = usize;

    fn mul(self, rhs: Self) -> Self::Output {
        usize::from_str_radix(&self.to_string(), 2).unwrap() * usize::from_str_radix(&rhs.to_string(), 2).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let v: Vec<BitArray> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ]
            .into_iter()
            .map(BitArray::from)
            .collect();
        assert_eq!(calc_consumption(v), 198);
    }

    #[test]
    fn base2() {
        let v: Vec<BitArray> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ]
            .into_iter()
            .map(BitArray::from)
            .collect();
        assert_eq!(calc_life_support(v), 230);
    }
}
