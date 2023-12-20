use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    outputs: Vec<&'a str>,
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            let outputs: Vec<&str> = right.split(", ").map(|s| s).collect();
            let (name, module_type) = if left.starts_with("%") {
                (&left[1..], ModuleType::FlipFlop(false))
            } else if left.starts_with("&") {
                (&left[1..], ModuleType::Conjunction(HashMap::new()))
            } else {
                (&left[..], ModuleType::Broadcaster)
            };
            (
                name,
                Module {
                    module_type,
                    outputs,
                },
            )
        })
        .collect();
    //prime maps
    for (key, module) in modules.clone() {
        for output in module.outputs.iter() {
            match modules.get_mut(output) {
                Some(Module {
                    module_type: ModuleType::Conjunction(map),
                    ..
                }) => {
                    map.insert(key, false);
                    map.insert(key, false);
                }
                _ => {}
            }
        }
    }
    modules
}

fn signal<'a>(
    modules: &mut HashMap<&str, Module<'a>>,
    from: &'a str,
    key: &'a str,
    hi: bool,
) -> Vec<(&'a str, &'a str, bool)> {
    let module = modules.get_mut(key);
    if module.is_none() {
        return Vec::new();
    }
    let module = module.unwrap();
    match (&mut module.module_type, hi) {
        (ModuleType::Broadcaster, hi) => module
            .outputs
            .iter()
            .map(|&output| (output, key, hi))
            .collect(),
        (ModuleType::FlipFlop(state), false) => {
            *state = !*state;
            module
                .outputs
                .iter()
                .map(|&output| (output, key, *state))
                .collect()
        }
        (ModuleType::FlipFlop(_), true) => Vec::new(),
        (ModuleType::Conjunction(map), hi) => {
            *map.get_mut(&from).unwrap() = hi;
            module
                .outputs
                .iter()
                .map(|&output| {
                    (
                        output,
                        key,
                        map.is_empty() || map.values().any(|&v| v == false),
                    )
                })
                .collect()
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut modules = parse(input);

    let mut num_signals = [0; 2];
    for _ in 0..1000 {
        let mut queue = VecDeque::from([("broadcaster", "button", false)]);
        while let Some((key, from, hi)) = queue.pop_front() {
            num_signals[hi as usize] += 1u64;
            let outputs = signal(&mut modules, from, key, hi);
            queue.extend(outputs);
        }
    }
    Some(num_signals.iter().product::<u64>())
}

fn fill_cluster<'a>(
    modules: &HashMap<&str, Module<'a>>,
    cluster: &mut HashSet<&'a str>,
    key: &'a str,
    last_conjunction: &'a str,
) {
    if key == last_conjunction || cluster.contains(key) {
        return;
    }
    cluster.insert(key);
    let module = &modules.get(key);
    if module.is_none() {
        panic!("{} is none", key);
    }
    let module = module.unwrap();
    for output in module.outputs.iter() {
        fill_cluster(modules, cluster, output, last_conjunction);
    }
}

fn euclid_gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    euclid_gcd(b, a % b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules = parse(input);

    //find last conjunction
    let last_conjunction = *modules
        .iter()
        .find(|(_, module)| module.outputs == vec!["rx"])
        .unwrap()
        .0;

    //find the clusters
    let mut clusters = Vec::new();
    for node in modules["broadcaster"].outputs.iter() {
        let mut cluster = HashSet::new();
        fill_cluster(&modules, &mut cluster, node, last_conjunction);
        let last = *cluster
            .iter()
            .find(|&&n| modules[n].outputs == vec![last_conjunction])
            .unwrap();
        clusters.push((cluster.into_iter().collect::<Vec<&str>>(), last));
    }

    let mut cycles = Vec::new();
    let mut iter = 1;
    'outer: loop {
        let mut queue = VecDeque::from([("broadcaster", "button", false)]);
        while let Some((key, from, hi)) = queue.pop_front() {
            let outputs = signal(&mut modules, from, key, hi);
            queue.extend(outputs);
            if clusters.iter().map(|(_, b)| *b).contains(&key) {
                if hi == false {
                    cycles.push(iter);
                    if cycles.len() == clusters.len() {
                        break 'outer;
                    }
                }
            }
        }
        iter += 1;
    }

    let lcm = cycles
        .iter()
        .fold(1, |acc, &c| acc * c / euclid_gcd(acc, c));
    Some(lcm)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::submit::submit(20, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::submit::submit(20, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
