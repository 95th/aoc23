fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for s in input.trim().split(',') {
        total += hash(s);
    }
    total
}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.bytes() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let h = hash("HASH");
        assert_eq!(h, 52);
    }

    #[test]
    fn part_1_basic() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let output = part_1(input);
        assert_eq!(output, 1320);
    }
}
