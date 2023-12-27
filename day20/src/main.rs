use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");

    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let (mut g, inputs) = parse_modules(input);

    let mut low_count = 0;
    let mut high_count = 0;

    let mut queue = VecDeque::<(&str, &str, Pulse)>::new();
    for _ in 0..1000 {
        queue.push_back(("button", "broadcaster", Pulse::Low));
        while let Some((input, node, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            let Some(module) = g.get_mut(node) else {
                continue;
            };
            match &mut module.kind {
                ModuleKind::Broadcast => {
                    for n in module.out.iter() {
                        queue.push_back((node, n, pulse));
                    }
                }
                ModuleKind::FlipFlop { on } => {
                    if let Pulse::Low = pulse {
                        *on = !*on;
                        if *on {
                            for n in module.out.iter() {
                                queue.push_back((node, n, Pulse::High));
                            }
                        } else {
                            for n in module.out.iter() {
                                queue.push_back((node, n, Pulse::Low));
                            }
                        }
                    }
                }
                ModuleKind::Conjunction { high } => {
                    if let Pulse::High = pulse {
                        high.insert(input);
                    } else {
                        high.remove(input);
                    }
                    if high.len() == inputs[node] {
                        for n in module.out.iter() {
                            queue.push_back((node, n, Pulse::Low));
                        }
                    } else {
                        for n in module.out.iter() {
                            queue.push_back((node, n, Pulse::High));
                        }
                    }
                }
            }
        }
    }

    low_count * high_count
}

fn part_2(input: &str) -> usize {
    let (mut g, inputs) = parse_modules(input);

    let mut button_pressed = 0;
    let mut queue = VecDeque::<(&str, &str, Pulse)>::new();

    let mut visited = HashSet::<&str>::new();
    let mut lengths = HashMap::<&str, usize>::new();

    let rx_feeder = *g
        .iter()
        .find(|(_, v)| v.out.contains(&"rx"))
        .map(|(k, _)| k)
        .unwrap();

    loop {
        button_pressed += 1;
        queue.push_back(("button", "broadcaster", Pulse::Low));

        while let Some((input, node, pulse)) = queue.pop_front() {
            if let Pulse::High = pulse {
                if node == rx_feeder {
                    visited.insert(input);
                    if !lengths.contains_key(input) {
                        lengths.insert(input, button_pressed);
                    }

                    if visited.len() == inputs[rx_feeder] {
                        let mut product = 1;
                        for len in lengths.values() {
                            product = product * len;
                        }
                        return product;
                    }
                }
            }

            let Some(module) = g.get_mut(node) else {
                continue;
            };
            match &mut module.kind {
                ModuleKind::Broadcast => {
                    for n in module.out.iter() {
                        queue.push_back((node, n, pulse));
                    }
                }
                ModuleKind::FlipFlop { on } => {
                    if let Pulse::Low = pulse {
                        *on = !*on;
                        if *on {
                            for n in module.out.iter() {
                                queue.push_back((node, n, Pulse::High));
                            }
                        } else {
                            for n in module.out.iter() {
                                queue.push_back((node, n, Pulse::Low));
                            }
                        }
                    }
                }
                ModuleKind::Conjunction { high } => {
                    if let Pulse::High = pulse {
                        high.insert(input);
                    } else {
                        high.remove(input);
                    }
                    if high.len() == inputs[node] {
                        for n in module.out.iter() {
                            queue.push_back((node, n, Pulse::Low));
                        }
                    } else {
                        for n in module.out.iter() {
                            queue.push_back((node, n, Pulse::High));
                        }
                    }
                }
            }
        }
    }
}

fn parse_modules(input: &str) -> (HashMap<&str, Module<'_>>, HashMap<&str, usize>) {
    let mut g = HashMap::<&str, Module>::new();
    let mut inputs = HashMap::<&str, usize>::new();
    for line in input.lines() {
        let (name, rest) = line.split_once(" -> ").unwrap();
        let out: Vec<_> = rest.split(", ").collect();
        let (name, kind) = if let Some(name) = name.strip_prefix('%') {
            (name, ModuleKind::FlipFlop { on: false })
        } else if let Some(name) = name.strip_prefix('&') {
            (
                name,
                ModuleKind::Conjunction {
                    high: HashSet::new(),
                },
            )
        } else {
            (name, ModuleKind::Broadcast)
        };
        for o in out.iter() {
            *inputs.entry(o).or_insert(0) += 1;
        }
        g.insert(name, Module { kind, out });
    }
    (g, inputs)
}

#[derive(Debug)]
struct Module<'a> {
    kind: ModuleKind<'a>,
    out: Vec<&'a str>,
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum ModuleKind<'a> {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { high: HashSet<&'a str> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let input = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let output = part_1(input);
        assert_eq!(output, 32000000);
    }

    #[test]
    fn part_1_interesting() {
        let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let output = part_1(input);
        assert_eq!(output, 11687500);
    }
}
