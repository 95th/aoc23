fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
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

fn win_combination(t: usize, s: usize) -> usize {
    let term = (t.pow(2) as f32 - 4.0 * s as f32).sqrt();
    let upper = (t as f32 + term) / 2.0;
    let lower = (t as f32 - term) / 2.0;
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
}
