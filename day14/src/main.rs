fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");

    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut board: Vec<_> = input.lines().map(|x| x.as_bytes().to_vec()).collect();
    tilt_north(&mut board);
    check_load(&board)
}

fn part_2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|x| x.as_bytes().to_vec()).collect();

    let board_1 = &mut lines.clone();
    let board_2 = &mut lines.clone();

    let mut i = 0;
    loop {
        i += 1;
        spin(board_1);
        for _ in 0..2 {
            spin(board_2);
        }

        if board_1 == board_2 {
            break;
        }
    }

    let mut loop_len = 0;
    loop {
        loop_len += 1;
        spin(board_2);
        if board_1 == board_2 {
            break;
        }
    }

    let remaining = (1_000_000_000 - i) % loop_len;
    for _ in 0..remaining {
        spin(board_2);
    }

    check_load(board_2)
}

fn check_load(board: &Vec<Vec<u8>>) -> usize {
    board
        .iter()
        .enumerate()
        .map(|(i, row)| (board.len() - i) * row.iter().filter(|&&c| c == b'O').count())
        .sum()
}

fn spin(board: &mut Vec<Vec<u8>>) {
    tilt_north(board);
    tilt_east(board);
    tilt_south(board);
    tilt_west(board);
}

fn tilt_north(board: &mut Vec<Vec<u8>>) {
    for j in 0..board[0].len() {
        let mut k = 0;
        for i in 0..board.len() {
            if board[i][j] == b'O' {
                if i != k {
                    board[i][j] = board[k][j];
                    board[k][j] = b'O';
                }
                k += 1;
            } else if board[i][j] == b'#' {
                k = i + 1;
            }
        }
    }
}

fn tilt_east(board: &mut Vec<Vec<u8>>) {
    for i in 0..board.len() {
        let mut k = 0;
        for j in 0..board[0].len() {
            if board[i][j] == b'O' {
                if j != k {
                    board[i][j] = board[i][k];
                    board[i][k] = b'O';
                }
                k += 1;
            } else if board[i][j] == b'#' {
                k = j + 1;
            }
        }
    }
}

fn tilt_south(board: &mut Vec<Vec<u8>>) {
    for j in 0..board[0].len() {
        let mut k = board.len() - 1;
        for i in (0..board.len()).rev() {
            if board[i][j] == b'O' {
                if i != k {
                    board[i][j] = board[k][j];
                    board[k][j] = b'O';
                }
                k = k.saturating_sub(1);
            } else if board[i][j] == b'#' {
                k = i.saturating_sub(1);
            }
        }
    }
}

fn tilt_west(board: &mut Vec<Vec<u8>>) {
    for i in 0..board.len() {
        let mut k = board[0].len() - 1;
        for j in (0..board[0].len()).rev() {
            if board[i][j] == b'O' {
                if j != k {
                    board[i][j] = board[i][k];
                    board[i][k] = b'O';
                }
                k = k.saturating_sub(1);
            } else if board[i][j] == b'#' {
                k = j.saturating_sub(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let output = part_1(input);
        assert_eq!(output, 136);
    }

    #[test]
    fn part_2_basic() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let output = part_2(input);
        assert_eq!(output, 64);
    }

    fn to_vec(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(|x| x.to_string().into_bytes()).collect()
    }

    fn to_string(v: Vec<Vec<u8>>) -> String {
        v.into_iter()
            .map(|x| String::from_utf8(x).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn north_tilt_test() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let output = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        let mut x = to_vec(input);
        tilt_north(&mut x);
        let x = to_string(x);
        assert_eq!(x, output);
    }

    #[test]
    fn east_tilt_test() {
        let input = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        let output = r"OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....";
        let mut x = to_vec(input);
        tilt_east(&mut x);
        let x = to_string(x);
        assert_eq!(x, output);
    }

    #[test]
    fn south_tilt_test() {
        let input = r"OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....";

        let output = r".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....";
        let mut x = to_vec(input);
        tilt_south(&mut x);
        let x = to_string(x);
        assert_eq!(x, output);
    }

    #[test]
    fn west_tilt_test() {
        let input = r".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....";

        let output = r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let mut x = to_vec(input);
        tilt_west(&mut x);
        let x = to_string(x);
        assert_eq!(x, output);
    }
}
