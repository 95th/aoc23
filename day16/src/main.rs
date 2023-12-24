use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|it| it.as_bytes()).collect();

    let mut done = HashSet::<(usize, usize)>::new();
    let mut done_with_direction = HashSet::<(usize, usize, Direction)>::new();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, Direction::Right));

    while let Some((i, j, direction)) = queue.pop_front() {
        if !done_with_direction.insert((i, j, direction)) {
            continue;
        }

        done.insert((i, j));
        if let Some((i, j)) = move_in_direction(i, j, direction, &grid) {
            let tile = grid[i][j];
            match tile {
                b'.' => queue.push_back((i, j, direction)),
                b'|' => match direction {
                    Direction::Left | Direction::Right => {
                        queue.push_back((i, j, Direction::Up));
                        queue.push_back((i, j, Direction::Down));
                    }
                    _ => queue.push_back((i, j, direction)),
                },
                b'-' => match direction {
                    Direction::Up | Direction::Down => {
                        queue.push_back((i, j, Direction::Left));
                        queue.push_back((i, j, Direction::Right));
                    }
                    _ => queue.push_back((i, j, direction)),
                },
                b'/' => {
                    let reflected_direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    queue.push_back((i, j, reflected_direction));
                }
                b'\\' => {
                    let reflected_direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    queue.push_back((i, j, reflected_direction));
                }
                _ => unreachable!(),
            }
        }
    }

    done.len()
}

fn move_in_direction(
    i: usize,
    j: usize,
    direction: Direction,
    grid: &[&[u8]],
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if i == 0 {
                None
            } else {
                Some((i - 1, j))
            }
        }
        Direction::Down => {
            if i + 1 == grid.len() {
                None
            } else {
                Some((i + 1, j))
            }
        }
        Direction::Left => {
            if j == 0 {
                None
            } else {
                Some((i, j - 1))
            }
        }
        Direction::Right => {
            if j + 1 == grid[0].len() {
                None
            } else {
                Some((i, j + 1))
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let output = part_1(input);
        assert_eq!(output, 46);
    }
}
