fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input, 2);
    println!("{output}");

    let output = solve(input, 1_000_000);
    println!("{output}");
}

fn solve(input: &str, expansion_factor: usize) -> usize {
    let grid: Vec<_> = input.lines().map(|it| it.as_bytes()).collect();

    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    let mut galaxies = Vec::new();

    for (i, &row) in grid.iter().enumerate() {
        if row.iter().all(|c| *c == b'.') {
            empty_rows.push(i);
        }
        for (j, &c) in row.iter().enumerate() {
            if c == b'#' {
                galaxies.push((i, j));
            }
        }
    }

    'outer: for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] != b'.' {
                continue 'outer;
            }
        }
        empty_cols.push(j);
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (i1, j1) = galaxies[i];
            let (i2, j2) = galaxies[j];

            let mut distance = i1.abs_diff(i2) + j1.abs_diff(j2);
            for k in empty_rows.iter() {
                if (i1..i2).contains(k) || (i2..i1).contains(k) {
                    distance += expansion_factor - 1;
                }
            }
            for k in empty_cols.iter() {
                if (j1..j2).contains(k) || (j2..j1).contains(k) {
                    distance += expansion_factor - 1;
                }
            }
            sum += distance;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_simple() {
        const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        let output = solve(INPUT, 2);
        assert_eq!(output, 374);

        let output = solve(INPUT, 10);
        assert_eq!(output, 1030);

        let output = solve(INPUT, 100);
        assert_eq!(output, 8410);
    }
}
