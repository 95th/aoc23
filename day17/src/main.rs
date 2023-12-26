use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");

    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    solve(input, 0, 3)
}

fn part_2(input: &str) -> usize {
    solve(input, 4, 10)
}

fn solve(input: &str, min_steps: u8, max_steps: u8) -> usize {
    let grid = Grid::parse(input);
    let target = (grid.rows() - 1, grid.cols() - 1);

    // BinaryHeap is max heap. We need min heap, hence the `Reverse`.
    let mut queue = BinaryHeap::<Reverse<(usize, (usize, usize), Direction, u8)>>::new();
    let mut seen = HashSet::<((usize, usize), Direction, u8)>::new();

    queue.push(Reverse((0, (0, 0), Direction::Right, 0)));
    queue.push(Reverse((0, (0, 0), Direction::Down, 0)));

    while let Some(Reverse((cost, position, direction, steps))) = queue.pop() {
        if position == target && steps >= min_steps {
            return cost;
        }

        if !seen.insert((position, direction, steps)) {
            continue;
        }

        if steps >= min_steps {
            let left = direction.left();
            if let Some(position) = grid.neighbor_of(position, left) {
                queue.push(Reverse((cost + grid[position] as usize, position, left, 1)));
            }

            let right = direction.right();
            if let Some(position) = grid.neighbor_of(position, right) {
                queue.push(Reverse((
                    cost + grid[position] as usize,
                    position,
                    right,
                    1,
                )));
            }
        }

        if steps < max_steps {
            if let Some(position) = grid.neighbor_of(position, direction) {
                queue.push(Reverse((
                    cost + grid[position] as usize,
                    position,
                    direction,
                    steps + 1,
                )));
            }
        }
    }

    usize::MAX
}

struct Grid {
    data: Vec<Vec<u8>>,
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|it| it.bytes().map(|c| c - b'0').collect())
                .collect::<Vec<_>>(),
        }
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }

    fn neighbor_of(&self, (i, j): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let (di, dj) = direction.move_delta();
        let i = i as isize + di;
        let j = j as isize + dj;

        if i >= 0 && i < self.rows() as isize && j >= 0 && j < self.cols() as isize {
            Some((i as usize, j as usize))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn left(self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    const fn right(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    const fn move_delta(self) -> (isize, isize) {
        use Direction::*;
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let output = part_1(input);
        assert_eq!(output, 102);
    }

    #[test]
    fn part_1_test_2() {
        let input = r"241343
321545";
        let output = part_1(input);
        assert_eq!(output, 20);
    }

    #[test]
    fn part_2_test() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let output = part_2(input);
        assert_eq!(output, 94);
    }

    #[test]
    fn part_2_test_2() {
        let input = r"111111111111
999999999991
999999999991
999999999991
999999999991";
        let output = part_2(input);
        assert_eq!(output, 71);
    }
}
