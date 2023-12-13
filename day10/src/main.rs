fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}");
}

fn part_1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let mut i = 0;
    let mut j = 0;
    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, c) in line.iter().enumerate() {
            if *c == b'S' {
                i = line_index;
                j = char_index;
                break;
            }
        }
    }

    let mut dir = Direction::Left;

    if i > 0 && lines[i - 1][j] == b'|' {
        dir = Direction::Up;
        i -= 1;
    } else if i > 0 && j > 0 && lines[i - 1][j] == b'7' {
        dir = Direction::Up;
        i -= 1;
    } else if i > 0 && j < lines[0].len() - 1 && lines[i - 1][j] == b'F' {
        dir = Direction::Up;
        i -= 1;
    } else if j > 0 && lines[i][j - 1] == b'-' {
        dir = Direction::Left;
        j -= 1;
    } else if j > 0 && i > 0 && lines[i][j - 1] == b'L' {
        dir = Direction::Left;
        j -= 1;
    } else if j > 0 && i < lines.len() - 1 && lines[i][j - 1] == b'F' {
        dir = Direction::Left;
        j -= 1;
    } else if j < lines[0].len() - 1 && lines[i][j + 1] == b'-' {
        dir = Direction::Right;
        j += 1;
    } else if j < lines[0].len() - 1 && i > 0 && lines[i][j + 1] == b'J' {
        dir = Direction::Right;
        j += 1;
    } else if j < lines[0].len() - 1 && i < lines.len() - 1 && lines[i][j + 1] == b'7' {
        dir = Direction::Right;
        j += 1;
    } else if i < lines.len() - 1 && lines[i + 1][j] == b'|' {
        dir = Direction::Down;
        i += 1;
    } else if i < lines.len() - 1 && j > 0 && lines[i + 1][j] == b'J' {
        dir = Direction::Down;
        i += 1;
    } else if i < lines.len() - 1 && j < lines[0].len() - 1 && lines[i + 1][j] == b'L' {
        dir = Direction::Down;
        i += 1;
    }
    let mut steps = 1;
    loop {
        match lines[i][j] {
            b'S' => break,
            b'|' => {
                if dir == Direction::Up {
                    i -= 1;
                } else {
                    i += 1;
                }
            }
            b'-' => {
                if dir == Direction::Left {
                    j -= 1;
                } else {
                    j += 1;
                }
            }
            b'F' => {
                if dir == Direction::Left {
                    i += 1;
                    dir = Direction::Down;
                } else {
                    j += 1;
                    dir = Direction::Right;
                }
            }
            b'J' => {
                if dir == Direction::Down {
                    j -= 1;
                    dir = Direction::Left;
                } else {
                    i -= 1;
                    dir = Direction::Up;
                }
            }
            b'7' => {
                if dir == Direction::Right {
                    i += 1;
                    dir = Direction::Down;
                } else {
                    j -= 1;
                    dir = Direction::Left;
                }
            }
            b'L' => {
                if dir == Direction::Down {
                    j += 1;
                    dir = Direction::Right;
                } else {
                    i -= 1;
                    dir = Direction::Up;
                }
            }
            _ => unreachable!(),
        }
        steps += 1;
    }

    steps / 2
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        let output = part_1(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn part_1_basic_with_clutter() {
        let input = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;
        let output = part_1(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn part_1_complex() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        let output = part_1(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn part_1_complex_with_clutter() {
        let input = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;
        let output = part_1(input);
        assert_eq!(output, 8);
    }
}
