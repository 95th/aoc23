fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");
    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(' ').unwrap();
        let sizes: Vec<_> = second.split(',').map(|it| it.parse().unwrap()).collect();
        total += get_combinations(first.as_bytes(), &sizes);
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(' ').unwrap();

        let first = [first; 5].join("?");
        let sizes = second
            .split(',')
            .map(|it| it.parse().unwrap())
            .collect::<Vec<_>>()
            .repeat(5);
        total += get_combinations(first.as_bytes(), &sizes);
    }
    total
}

fn get_combinations(pattern: &[u8], sizes: &[usize]) -> usize {
    let mut cache = vec![vec![-1; pattern.len()]; sizes.len()];
    get_combinations_cached(pattern, sizes, 0, 0, &mut cache)
}

fn get_combinations_cached(
    pattern: &[u8],
    sizes: &[usize],
    pat_index: usize,
    size_index: usize,
    cache: &mut Vec<Vec<i64>>,
) -> usize {
    match cache.get(size_index).and_then(|it| it.get(pat_index)) {
        Some(&n) if n >= 0 => return n as usize,
        _ => (),
    }

    if size_index >= sizes.len() {
        return if pat_index < pattern.len() && pattern[pat_index..].contains(&b'#') {
            0
        } else {
            1
        };
    }

    if pat_index >= pattern.len() {
        return 0;
    }

    let mut combinations = 0;

    if let b'.' | b'?' = pattern[pat_index] {
        combinations += get_combinations_cached(pattern, sizes, pat_index + 1, size_index, cache);
    }

    if let b'#' | b'?' = pattern[pat_index] {
        let end = pat_index + sizes[size_index];
        if end <= pattern.len()
            && !pattern[pat_index..end].contains(&b'.')
            && (end == pattern.len() || pattern[end] != b'#')
        {
            combinations += get_combinations_cached(pattern, sizes, end + 1, size_index + 1, cache)
        }
    }

    cache[size_index][pat_index] = combinations as i64;
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
        assert_eq!(part_1("??.???.???.???.???. 2,2,2"), 56);
    }

    #[test]
    fn part_2_basic() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        let output = part_2(input);
        assert_eq!(output, 525152);
    }

    #[test]
    fn part_2_custom() {
        assert_eq!(part_2("??? 3"), 1);
        assert_eq!(part_2("??? 1,1"), 1);
        assert_eq!(part_2("??. 2"), 16);
    }
}
