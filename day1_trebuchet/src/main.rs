fn main() {
    let input = include_str!("../input.txt");
    let part_1 = find_calibration(input, part1);
    println!("Part 1: {}", part_1);

    let part_2 = find_calibration(input, part2);
    println!("Part 2: {}", part_2);
}

fn part1(line: &str) -> u32 {
    let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
    let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();
    let first_digit = first_digit.to_digit(10).unwrap();
    let last_digit = last_digit.to_digit(10).unwrap();
    first_digit * 10 + last_digit
}

fn part2(line: &str) -> u32 {
    const WORD_MAP: [&[u8]; 10] = [
        b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    let mut first_digit = 0;
    let chars = line.as_bytes();
    let mut i = 0;
    'outer: while i < chars.len() {
        for (j, w) in WORD_MAP.iter().enumerate() {
            if chars[i..].starts_with(w) {
                first_digit = j as u32;
                break 'outer;
            }
        }
        if chars[i].is_ascii_digit() {
            first_digit = (chars[i] - b'0') as u32;
            break;
        }
        i += 1;
    }

    let mut last_digit = 0;
    let chars = line.as_bytes();
    let mut i = chars.len();
    'outer: while i > 0 {
        i -= 1;
        for (j, w) in WORD_MAP.iter().enumerate() {
            if chars[i..].starts_with(w) {
                last_digit = j as u32;
                break 'outer;
            }
        }
        if chars[i].is_ascii_digit() {
            last_digit = (chars[i] - b'0') as u32;
            break;
        }
    }

    first_digit * 10 + last_digit
}

fn find_calibration(calibration_doc: &str, callback: impl Fn(&str) -> u32) -> usize {
    let mut sum = 0;
    for line in calibration_doc.lines() {
        sum += callback(line) as usize;
    }
    sum
}
