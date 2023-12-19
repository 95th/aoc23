fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("Output: {output}");

    let output = part_2(input);
    println!("Output: {output}");
}

fn part_1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let mut start = (0, 0);
    'outer: for (line_index, line) in lines.iter().enumerate() {
        for (char_index, c) in line.iter().enumerate() {
            if *c == b'S' {
                start = (line_index, char_index);
                break 'outer;
            }
        }
    }

    let (mut i, mut j) = start;

    let mut dir = if i > 0 && b"|7F".contains(&lines[i - 1][j]) {
        Direction::Up
    } else if j > 0 && b"-LF".contains(&lines[i][j - 1]) {
        Direction::Left
    } else if j < lines[0].len() - 1 && b"-J7".contains(&lines[i][j + 1]) {
        Direction::Right
    } else if i < lines.len() - 1 && b"|JL".contains(&lines[i + 1][j]) {
        Direction::Down
    } else {
        unreachable!()
    };
    let mut steps = 0;
    loop {
        match dir {
            Direction::Left => j -= 1,
            Direction::Right => j += 1,
            Direction::Up => i -= 1,
            Direction::Down => i += 1,
        }

        match lines[i][j] {
            b'F' => {
                dir = if let Direction::Up = dir {
                    Direction::Right
                } else {
                    Direction::Down
                };
            }
            b'J' => {
                dir = if let Direction::Down = dir {
                    Direction::Left
                } else {
                    Direction::Up
                };
            }
            b'7' => {
                dir = if let Direction::Right = dir {
                    Direction::Down
                } else {
                    Direction::Left
                };
            }
            b'L' => {
                dir = if let Direction::Down = dir {
                    Direction::Right
                } else {
                    Direction::Up
                };
            }
            _ => (),
        }

        steps += 1;
        if (i, j) == start {
            break;
        }
    }

    steps / 2
}

fn part_2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let mut start = (0, 0);
    'outer: for (line_index, line) in lines.iter().enumerate() {
        for (char_index, c) in line.iter().enumerate() {
            if *c == b'S' {
                start = (line_index, char_index);
                break 'outer;
            }
        }
    }

    let (mut i, mut j) = start;

    let mut dir = if i > 0 && b"|7F".contains(&lines[i - 1][j]) {
        Direction::Up
    } else if j > 0 && b"-LF".contains(&lines[i][j - 1]) {
        Direction::Left
    } else if i < lines.len() - 1 && b"|JL".contains(&lines[i + 1][j]) {
        Direction::Down
    } else if j < lines[0].len() - 1 && b"-J7".contains(&lines[i][j + 1]) {
        Direction::Right
    } else {
        unreachable!()
    };

    let mut path = vec![start];

    loop {
        match dir {
            Direction::Left => j -= 1,
            Direction::Right => j += 1,
            Direction::Up => i -= 1,
            Direction::Down => i += 1,
        }

        match lines[i][j] {
            b'F' => {
                dir = if let Direction::Up = dir {
                    Direction::Right
                } else {
                    Direction::Down
                };
            }
            b'J' => {
                dir = if let Direction::Down = dir {
                    Direction::Left
                } else {
                    Direction::Up
                };
            }
            b'7' => {
                dir = if let Direction::Right = dir {
                    Direction::Down
                } else {
                    Direction::Left
                };
            }
            b'L' => {
                dir = if let Direction::Down = dir {
                    Direction::Right
                } else {
                    Direction::Up
                };
            }
            _ => (),
        }

        path.push((i, j));
        if (i, j) == start {
            break;
        }
    }

    let mut area = 0_isize;

    for i in 0..path.len() - 1 {
        let (a, b) = path[i];
        let (c, d) = path[i + 1];
        area += (a * d) as isize - (b * c) as isize;
    }

    1 + (area.abs() / 2) as usize - (path.len() - 1) / 2
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

    #[test]
    fn part_2_simple() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        let output = part_2(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn part_2_complex() {
        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        let output = part_2(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn part_2_complex_2() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        let output = part_2(input);
        assert_eq!(output, 10);
    }
}
