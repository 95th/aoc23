fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}");

    let output = part_2(input);
    println!("Output: {output}");
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap(), "Time:");
    let distance = parse_line(lines.next().unwrap(), "Distance:");

    let mut total = 1;
    for (&t, &s) in time.iter().zip(&distance) {
        total *= win_combination(t, s);
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    // Time:        51699878
    // Distance:   377117112241505

    win_combination(time, distance)
}

fn win_combination(t: usize, s: usize) -> usize {
    let term = (t.pow(2) as f64 - 4.0 * s as f64).sqrt();
    let upper = (t as f64 + term) / 2.0;
    let lower = (t as f64 - term) / 2.0;
    let upper = (upper - 1.0).ceil();
    let lower = (lower + 1.0).floor();
    (upper - lower + 1.0) as usize
}

fn parse_line(line: &str, prefix: &str) -> Vec<usize> {
    line.strip_prefix(prefix)
        .unwrap()
        .split_whitespace()
        .map(|it| it.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_basic() {
        let output = part_1(INPUT);
        assert_eq!(output, 288);
    }

    #[test]
    fn part2_basic() {
        let output = part_2(INPUT);
        assert_eq!(output, 71503);
    }
}
