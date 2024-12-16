fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse::<u64>().unwrap();
            let components = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|f| f.parse().unwrap())
                .collect();
            (test_value, components)
        })
        .collect()
}

fn ilog10(n: u64) -> u32 {
    (n as f64).log10() as u32
}

fn check_tree(target: u64, components: &[u64], current: u64, part2: bool) -> bool {
    //check end
    if components.is_empty() {
        return target == current;
    }

    if current > target {
        return false;
    }

    //check current
    if check_tree(target, &components[1..], current + components[0], part2) {
        return true;
    }

    if check_tree(target, &components[1..], current * components[0], part2) {
        return true;
    }

    if part2
        && check_tree(
            target,
            &components[1..],
            current * (10u64.pow(1 + ilog10(components[0]))) + components[0],
            part2,
        )
    {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse(input);
    Some(
        data.iter()
            .filter(|(target, components)| check_tree(*target, components, 0, false))
            .map(|(target, _)| { *target })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse(input);
    Some(
        data.iter()
            .filter(|(target, components)| check_tree(*target, components, 0, true))
            .map(|(target, _)| { *target })
            .sum(),
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::submit::submit(7, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::submit::submit(7, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(3749));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(11387));
    }
}
