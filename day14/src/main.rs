fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|x| x.as_bytes()).collect();

    let mut total = 0;
    for j in 0..lines[0].len() {
        let mut n = lines.len();
        let mut load = 0;
        for i in 0..lines.len() {
            if lines[i][j] == b'#' {
                n = lines.len() - i - 1;
            } else if lines[i][j] == b'O' {
                load += n;
                n -= 1;
            }
        }
        total += load;
    }

    total
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
}
