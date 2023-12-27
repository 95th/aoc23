use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input, 64);
    println!("{output}");
}

fn part_1(input: &str, steps: usize) -> usize {
    let grid: Vec<_> = input.lines().map(|it| it.as_bytes()).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &c)| if c == b'S' { Some((i, j)) } else { None })
        })
        .unwrap();

    let mut queue = Vec::<(usize, usize)>::new();

    queue.push(start);

    for _ in 0..steps {
        let mut next_steps = HashSet::new();
        while let Some(pos) = queue.pop() {
            move_in_direction(pos, Direction::Up, &grid, &mut next_steps);
            move_in_direction(pos, Direction::Down, &grid, &mut next_steps);
            move_in_direction(pos, Direction::Left, &grid, &mut next_steps);
            move_in_direction(pos, Direction::Right, &grid, &mut next_steps);
        }
        queue.extend(next_steps);
        if queue.is_empty() {
            break;
        }
    }

    queue.len()
}

fn move_in_direction(
    pos: (usize, usize),
    dir: Direction,
    grid: &[&[u8]],
    queue: &mut HashSet<(usize, usize)>,
) {
    let pos = neighbor(pos, dir);
    if let Some(b'.' | b'S') = get_plot(grid, pos) {
        queue.insert((pos.0 as usize, pos.1 as usize));
    }
}

fn neighbor((i, j): (usize, usize), dir: Direction) -> (isize, isize) {
    let (di, dj) = match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };
    (i as isize + di, j as isize + dj)
}

fn get_plot(grid: &[&[u8]], (i, j): (isize, isize)) -> Option<u8> {
    if i < 0 || j < 0 || i as usize >= grid.len() || j as usize >= grid[0].len() {
        return None;
    }

    Some(grid[i as usize][j as usize])
}

#[derive(Clone, Copy)]
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
    fn part_1_basic() {
        let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let output = part_1(input, 6);
        assert_eq!(output, 16);
    }
}
