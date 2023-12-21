fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut dataset = vec![Vec::new()];
    for line in input.lines() {
        if line.is_empty() {
            dataset.push(Vec::new());
            continue;
        }

        dataset.last_mut().unwrap().push(line);
    }

    let mut total_r = 0;
    let mut total_c = 0;
    for valley in dataset {
        let (r, c) = get_reflection(valley);
        total_r += r;
        total_c += c;
    }

    total_c + 100 * total_r
}

fn get_reflection(valley: Vec<&str>) -> (usize, usize) {
    let mut r = 0;
    let mut c = 0;

    for i in 0..valley.len() - 1 {
        r += get_reflection_len_at_row(&valley, i);
    }

    for i in 0..valley[0].len() - 1 {
        c += get_reflection_len_at_col(&valley, i);
    }

    (r, c)
}

fn get_reflection_len_at_row(valley: &[&str], row_index: usize) -> usize {
    let mut j = row_index;
    let mut k = row_index + 1;

    loop {
        if !is_row_eq(valley, j, k) {
            break 0;
        }

        if j == 0 || k == valley.len() - 1 {
            return row_index + 1;
        }

        j -= 1;
        k += 1;
    }
}

fn is_row_eq(valley: &[&str], a: usize, b: usize) -> bool {
    for i in 0..valley[0].len() {
        if valley[a].as_bytes()[i] != valley[b].as_bytes()[i] {
            return false;
        }
    }

    true
}

fn get_reflection_len_at_col(valley: &[&str], col_index: usize) -> usize {
    let mut j = col_index;
    let mut k = col_index + 1;

    loop {
        if !is_col_eq(valley, j, k) {
            break 0;
        }

        if j == 0 || k == valley[0].len() - 1 {
            return col_index + 1;
        }

        j -= 1;
        k += 1;
    }
}

fn is_col_eq(valley: &[&str], a: usize, b: usize) -> bool {
    for v in valley {
        if v.as_bytes()[a] != v.as_bytes()[b] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        let output = part_1(input);
        assert_eq!(output, 405);
    }
}
