use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let inputs: Vec<Input> = input.lines().map(|it| it.parse().unwrap()).collect();

    let mut path = vec![(0, 0)];

    let mut last = (0, 0);
    for input in inputs.iter() {
        for _ in 0..input.steps {
            last = move_in_direction(input.dir, last);
            path.push(last);
        }
    }

    let mut area = 0;
    for i in 1..path.len() {
        let (a, b) = path[i - 1];
        let (c, d) = path[i];
        area += a * d - b * c;
    }

    1 + area.abs() as usize / 2 + (path.len() - 1) / 2
}

fn move_in_direction(direction: u8, (i, j): (isize, isize)) -> (isize, isize) {
    match direction {
        b'U' => (i - 1, j),
        b'D' => (i + 1, j),
        b'L' => (i, j - 1),
        b'R' => (i, j + 1),
        _ => unreachable!(),
    }
}

struct Input {
    dir: u8,
    steps: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(' ').ok_or(())?;
        let (b, _c) = b.split_once(' ').ok_or(())?;

        Ok(Input {
            dir: a.as_bytes()[0],
            steps: b.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let output = part_1(input);
        assert_eq!(output, 62);
    }
}
