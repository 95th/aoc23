fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}");

    let output = part_2(input);
    println!("Output: {output}");
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

    predict_next(combined)
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

    predict_prev(combined)
}

fn predict_next(list: Vec<isize>) -> isize {
    let mut stack = vec![list];

    loop {
        let mut new = Vec::new();
        let last = stack.last().unwrap();
        let mut zeros = 0;
        for i in 0..last.len() - 1 {
            let d = last[i + 1] - last[i];
            new.push(d);
            if d == 0 {
                zeros += 1;
            }
        }
        if zeros == new.len() {
            break;
        }
        stack.push(new);
    }

    let mut curr = 0;
    while let Some(mut list) = stack.pop() {
        curr += list.pop().unwrap();
    }
    curr
}

fn predict_prev(list: Vec<isize>) -> isize {
    let mut stack = vec![list];

    loop {
        let mut new = Vec::new();
        let last = stack.last().unwrap();
        let mut zeros = 0;
        for i in 0..last.len() - 1 {
            let d = last[i + 1] - last[i];
            new.push(d);
            if d == 0 {
                zeros += 1;
            }
        }
        if zeros == new.len() {
            break;
        }
        stack.push(new);
    }

    let mut curr = 0;
    while let Some(list) = stack.pop() {
        curr = list[0] - curr;
    }
    curr
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
}
