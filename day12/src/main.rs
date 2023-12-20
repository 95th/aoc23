fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        total += get_arrangements(line);
    }
    total
}

fn get_arrangements(row: &str) -> usize {
    let mut split = row.split(' ');
    let first = split.next().unwrap();
    let second = split.next().unwrap();

    let sizes: Vec<_> = second.split(',').map(|it| it.parse().unwrap()).collect();

    get_combinations(first.as_bytes(), &sizes, 0, 0)
}

fn get_combinations(
    pattern: &[u8],
    sizes: &[usize],
    mut pat_index: usize,
    size_index: usize,
) -> usize {
    while let Some(b'.') = pattern.get(pat_index) {
        pat_index += 1;
    }

    if size_index == sizes.len() {
        while let Some(b'?' | b'.') = pattern.get(pat_index) {
            pat_index += 1;
        }
    }

    if pat_index >= pattern.len() && size_index >= sizes.len() {
        return 1;
    }
    if pat_index >= pattern.len() || size_index >= sizes.len() {
        return 0;
    }

    let size = sizes[size_index];
    if pat_index + size > pattern.len() {
        return 0;
    }

    let mut combinations = 0;
    if pattern[pat_index..][..size]
        .iter()
        .all(|&c| c == b'?' || c == b'#')
    {
        if let Some(b'.' | b'?') = pattern.get(pat_index + size) {
            combinations = get_combinations(pattern, sizes, pat_index + size + 1, size_index + 1);
        } else if let Some(b'#') = pattern.get(pat_index + size) {
            combinations = 0;
        } else if size_index == sizes.len() - 1 {
            combinations = 1;
        } else {
            combinations = 0;
        }
    }

    if pattern[pat_index] == b'?' {
        combinations += get_combinations(pattern, sizes, pat_index + 1, size_index);
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        let output = part_1(input);
        assert_eq!(output, 21);
    }

    #[test]
    fn part_1_custom() {
        assert_eq!(part_1("??? 3"), 1);
        assert_eq!(part_1("??? 1,1"), 1);
        assert_eq!(part_1("??? 2"), 2);
        assert_eq!(part_1("??# 2"), 1);
        assert_eq!(part_1("??#? 2"), 2);
    }
}
