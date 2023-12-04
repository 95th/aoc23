fn main() {
    let input = include_str!("../input.txt");
    let part_1 = find_calibration(input, part1_check_digit);
    println!("Part 1: {}", part_1);

    let part_2 = find_calibration(input, part2_check_digit);
    println!("Part 2: {}", part_2);
}

fn find_calibration(
    calibration_doc: &str,
    digit_callback: impl Fn(&str, usize) -> Option<u8>,
) -> usize {
    let mut sum = 0;
    for line in calibration_doc.lines() {
        sum += calibration_from_line(line, &digit_callback) as usize;
    }
    sum
}

fn calibration_from_line(line: &str, digit_callback: impl Fn(&str, usize) -> Option<u8>) -> u32 {
    let mut first_digit = 0;
    for i in 0..line.len() {
        if let Some(digit) = digit_callback(line, i) {
            first_digit = digit as u32;
            break;
        }
    }

    let mut last_digit = 0;
    for i in (0..line.len()).rev() {
        if let Some(digit) = digit_callback(line, i) {
            last_digit = digit as u32;
            break;
        }
    }

    first_digit * 10 + last_digit
}

fn part1_check_digit(text: &str, index: usize) -> Option<u8> {
    if let Some(c) = text.as_bytes().get(index) {
        if c.is_ascii_digit() {
            return Some(c - b'0');
        }
    }
    None
}

fn part2_check_digit(text: &str, index: usize) -> Option<u8> {
    const WORD_MAP: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    if let Some(digit) = part1_check_digit(text, index) {
        return Some(digit);
    }

    for (word_index, word) in WORD_MAP.iter().enumerate() {
        if text[index..].starts_with(word) {
            return Some(word_index as u8);
        }
    }
    None
}
