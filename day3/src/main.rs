fn main() {
    let input = include_str!("../input.txt");
    let sum = part_1(input);
    println!("Part 1: {}", sum);
}

fn part_1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for i in 0..lines.len() {
        sum += find_part_number_sum(&lines, i);
    }
    sum
}

fn find_part_number_sum(lines: &[&str], index: usize) -> usize {
    let line = lines[index];
    let prev_line = if index > 0 { lines[index - 1] } else { "" };
    let next_line = if index < lines.len() - 1 {
        lines[index + 1]
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
            if is_part_number(line, prev_line, next_line, start_index, i) {
                sum += line[start_index..i].parse::<usize>().unwrap();
            }
            in_progress = false;
        }
    }

    if in_progress {
        if is_part_number(line, prev_line, next_line, start_index, line.len()) {
            sum += line[start_index..line.len()].parse::<usize>().unwrap();
        }
    }

    sum
}

fn is_symbol(line: &str, i: usize) -> bool {
    let c = line.as_bytes().get(i).copied().unwrap_or(b'.');
    !c.is_ascii_digit() && c != b'.'
}

fn is_part_number(
    line: &str,
    prev_line: &str,
    next_line: &str,
    start_index: usize,
    end_index: usize,
) -> bool {
    let start_index = start_index.saturating_sub(1);

    if is_symbol(line, start_index) || is_symbol(line, end_index) {
        return true;
    }

    for i in start_index..=end_index {
        if is_symbol(prev_line, i) || is_symbol(next_line, i) {
            return true;
        }
    }

    false
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
        let sum = super::part_1(text);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn basic2() {
        let text = r#"
467..114..
...@......
......633."#;
        let sum = super::part_1(text);
        assert_eq!(sum, 467);
    }
}
