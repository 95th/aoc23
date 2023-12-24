fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for s in input.trim().split(',') {
        total += hash(s);
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];

    for s in input.trim().split(',') {
        if let Some(s) = s.strip_suffix('-') {
            let h = hash(s);
            let b = &mut boxes[h];
            if let Some(i) = b.iter().position(|(lens, _focal_len)| s == *lens) {
                b.remove(i);
            }
        } else {
            let (lens, focal_len) = s.split_once('=').unwrap();
            let h = hash(lens);
            let b = &mut boxes[h];
            let focal_len = focal_len.parse().unwrap();
            if let Some(i) = b.iter().position(|(l, _)| *l == lens) {
                b[i].1 = focal_len;
            } else {
                b.push((lens, focal_len));
            }
        }
    }

    let mut sum = 0;
    for (i, lenses) in boxes.into_iter().enumerate() {
        for (j, (_lens, focal_len)) in lenses.into_iter().enumerate() {
            sum += (i + 1) * (j + 1) * focal_len;
        }
    }

    sum
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

    #[test]
    fn part_2_basic() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let output = part_2(input);
        assert_eq!(output, 145);
    }
}
