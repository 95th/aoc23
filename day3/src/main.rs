use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let (sum, gear_sum) = solve(input);
    println!("Part 1: {sum}");
    println!("Part 2: {gear_sum}");
}

fn solve(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    let mut gear_map = HashMap::new();
    for i in 0..lines.len() {
        sum += find_part_number_sum(&lines, i, &mut gear_map);
    }
    let mut gear_sum = 0;
    for (_, numbers) in gear_map {
        if numbers.len() == 2 {
            gear_sum += numbers[0] * numbers[1];
        }
    }
    (sum, gear_sum)
}

fn find_part_number_sum(
    lines: &[&str],
    line_index: usize,
    gear_map: &mut HashMap<(usize, usize), Vec<usize>>,
) -> usize {
    let line = lines[line_index];
    let prev_line = if line_index > 0 {
        lines[line_index - 1]
    } else {
        ""
    };
    let next_line = if line_index < lines.len() - 1 {
        lines[line_index + 1]
    } else {
        ""
    };

    let mut sum = 0;
    let mut start_index = 0;
    let mut in_progress = false;

    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            if !in_progress {
                start_index = i;
                in_progress = true;
            }
        } else if in_progress {
            let value = line[start_index..i].parse::<usize>().unwrap();
            if is_part_number(
                line,
                line_index,
                prev_line,
                next_line,
                start_index,
                i,
                value,
                gear_map,
            ) {
                sum += value;
            }
            in_progress = false;
        }
    }

    if in_progress {
        let value = line[start_index..].parse::<usize>().unwrap();
        if is_part_number(
            line,
            line_index,
            prev_line,
            next_line,
            start_index,
            line.len(),
            value,
            gear_map,
        ) {
            sum += value;
        }
    }

    sum
}

fn is_symbol(
    line: &str,
    line_index: usize,
    char_index: usize,
    value: usize,
    gear_map: &mut HashMap<(usize, usize), Vec<usize>>,
) -> bool {
    let c = line.as_bytes().get(char_index).copied().unwrap_or(b'.');
    if c == b'*' {
        gear_map
            .entry((line_index, char_index))
            .or_insert_with(Vec::new)
            .push(value);
    }
    !c.is_ascii_digit() && c != b'.'
}

fn is_part_number(
    line: &str,
    line_index: usize,
    prev_line: &str,
    next_line: &str,
    start_index: usize,
    end_index: usize,
    value: usize,
    gear_map: &mut HashMap<(usize, usize), Vec<usize>>,
) -> bool {
    let start_index = start_index.saturating_sub(1);

    let mut matching = false;

    if is_symbol(line, line_index, start_index, value, gear_map) {
        matching = true;
    }

    if is_symbol(line, line_index, end_index, value, gear_map) {
        matching = true;
    }

    for i in start_index..=end_index {
        if line_index > 0 && is_symbol(prev_line, line_index - 1, i, value, gear_map) {
            matching = true;
        }
        if is_symbol(next_line, line_index + 1, i, value, gear_map) {
            matching = true;
        }
    }

    matching
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {
        let text = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let sum = super::solve(text);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn basic2() {
        let text = r#"
467..114..
...@......
......633."#;
        let sum = super::solve(text);
        assert_eq!(sum, 467);
    }
}
