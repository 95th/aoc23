fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}"); // 2043183816

    let output = part_2(input);
    println!("Output: {output}"); // 1118
}

fn part_1(input: &str) -> isize {
    let combined = input
        .lines()
        .map(parse_list)
        .reduce(|a, b| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(a, b)| a + b)
                .collect()
        })
        .unwrap();

    let pos = combined.len() as isize;
    predict(combined, pos)
}

fn part_2(input: &str) -> isize {
    let combined = input
        .lines()
        .map(parse_list)
        .reduce(|a, b| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(a, b)| a + b)
                .collect()
        })
        .unwrap();

    predict(combined, -1)
}

fn predict(list: Vec<isize>, x: isize) -> isize {
    let mut sum = 0_f64;
    for (i, v) in list.iter().enumerate() {
        let mut product = 1_f64;
        for j in 0..list.len() {
            if j != i {
                product *= (x as f64 - j as f64) / (i as f64 - j as f64);
            }
        }
        sum += *v as f64 * product;
    }
    sum.round() as isize
}

fn parse_list(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_basic() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        let output = part_1(input);
        assert_eq!(output, 114)
    }

    #[test]
    fn part2_basic() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        let output = part_2(input);
        assert_eq!(output, 2)
    }

    #[test]
    fn part1_one_line() {
        let input = r#"0 3 6 9 12 15"#;
        let output = part_1(input);
        assert_eq!(output, 18)
    }
}
