fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("output: {}", output);

    let output = part_2(input);
    println!("output: {}", output);
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = get_seeds(lines.next().unwrap());

    let mut maps = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(":") {
            maps.push(Vec::new());
        } else {
            let map = maps.last_mut().unwrap();
            map.push(read_map_line(line));
        }
    }

    let mut min_location = usize::MAX;
    for seed in seeds {
        let mut current = seed;
        for map in maps.iter() {
            for (target, src, count) in map {
                if current >= *src && current < *src + *count {
                    current = target + current - src;
                    break;
                }
            }
        }
        if current < min_location {
            min_location = current;
        }
    }

    min_location
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = get_seeds(lines.next().unwrap());

    let mut maps = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(":") {
            maps.push(Vec::new());
        } else {
            let map = maps.last_mut().unwrap();
            map.push(read_map_line(line));
        }
    }

    let mut min_location = usize::MAX;
    for i in 0..seeds.len() / 2 {
        let seed = seeds[2 * i];
        let seed_count = seeds[2 * i + 1];

        let mut current = vec![Range::new(seed, seed + seed_count)];
        for map in maps.iter() {
            let mut pending = Vec::new();
            let mut done = Vec::new();
            for &(target, src, count) in map {
                let target = Range::new(target, target + count);
                let src = Range::new(src, src + count);
                for current in current.drain(..) {
                    if let Some(intersection) = src.intersection(current) {
                        done.push(intersection.translate(src, target));
                        if intersection.start > current.start {
                            pending.push(Range::new(current.start, intersection.start));
                        }
                        if intersection.end < current.end {
                            pending.push(Range::new(intersection.end, current.end))
                        }
                    } else {
                        pending.push(current);
                    }
                }
                current.extend(pending.drain(..));
            }
            current.extend(done.drain(..));
        }

        for r in current {
            if r.start < min_location {
                min_location = r.start;
            }
        }
    }

    min_location
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn intersection(self, other: Range) -> Option<Range> {
        let l = self.start.max(other.start);
        let r = self.end.min(other.end);
        if l < r {
            Some(Self::new(l, r))
        } else {
            None
        }
    }

    fn translate(self, src: Self, target: Self) -> Self {
        let l = self.start + target.start - src.start;
        let r = self.end + target.start - src.start;
        Self::new(l, r)
    }
}

fn get_seeds(line: &str) -> Vec<usize> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn read_map_line(line: &str) -> (usize, usize, usize) {
    let mut line = line.split_whitespace();
    let a = line.next().unwrap().parse().unwrap();
    let b = line.next().unwrap().parse().unwrap();
    let c = line.next().unwrap().parse().unwrap();
    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1_basic() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, 35);
    }

    #[test]
    fn part2_basic() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, 46);
    }
}
