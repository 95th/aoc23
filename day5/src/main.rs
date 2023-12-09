fn main() {
    let input = include_str!("../input.txt");
    let location = day_1(input);
    println!("Lowest Location: {}", location);
}

fn day_1(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().unwrap());
    lines.next(); // skip empty line

    let mut categories = Vec::new();

    while let Some(_) = lines.next().and_then(parse_map_category) {
        let mut mapping = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let (dst, src, count) = parse_map_line(line);
            mapping.push((dst..dst + count, src..src + count));
        }
        categories.push(mapping);
    }

    let mut location = usize::MAX;

    for seed in seeds {
        let mut current = seed;
        for mapping in &categories {
            for (dst_range, src_range) in mapping {
                if src_range.contains(&current) {
                    current = dst_range.start + (current - src_range.start);
                    break;
                }
            }
        }
        if current < location {
            location = current;
        }
    }

    location
}

fn parse_map_line(line: &str) -> (usize, usize, usize) {
    let mut split = line.split(" ");
    let a = split.next().unwrap().parse().unwrap();
    let b = split.next().unwrap().parse().unwrap();
    let c = split.next().unwrap().parse().unwrap();
    (a, b, c)
}

fn parse_map_category(line: &str) -> Option<&str> {
    line.strip_suffix(" map:")
}

fn parse_seeds(line: &str) -> Vec<usize> {
    let line = line.strip_prefix("seeds: ").unwrap();
    line.split(" ").map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[test]
    fn basic() {
        let input = r#"seeds: 79 14 55 13

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
        let location = day_1(input);
        assert_eq!(location, 35);
    }

    #[test]
    fn foo() {
        let a: Range<usize> = 11..53;
        assert!(!a.contains(&53));
    }
}
