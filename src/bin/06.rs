fn parse(input: &str) -> Vec<[f64; 2]> {
    let mut lines = input.lines();
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .zip(lines.next().unwrap().split_whitespace().skip(1))
        .map(|(time, dist)| [time.parse().unwrap(), dist.parse().unwrap()])
        .collect::<Vec<[f64; 2]>>()
}

fn get_combinations(input: &[f64; 2]) -> u32 {
    let time = input[0];
    let dist = input[1];

    let first_root = 0.5 * (time - (time * time - 4.0 * dist).sqrt());
    let second_root = 0.5 * (time + (time * time - 4.0 * dist).sqrt());
    0f64.max(second_root.ceil() - first_root.floor()) as u32 - 1
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).iter().map(get_combinations).product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse(input);
    let mut time = "".to_string();
    let mut dist = "".to_string();
    for i in 0..parsed.len() {
        time.push_str(&format!("{}", parsed[i][0]));
        dist.push_str(&format!("{}", parsed[i][1]));
    }
    let real_data: [f64; 2] = [time.parse::<f64>().unwrap(), dist.parse::<f64>().unwrap()];
    Some(get_combinations(&real_data))
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::submit::submit(6, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::submit::submit(6, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(288));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(71503));
    }
}
