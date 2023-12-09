use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(input: &Vec<i32>) -> i32 {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }
    let diffs = input
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    input.last().unwrap() + extrapolate(&diffs)
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    input
        .iter()
        .map(|line| extrapolate(line))
        .sum::<i32>()
        .into()
}

pub fn part_two(input: &str) -> Option<i32> {
    let input: Vec<Vec<i32>> = parse(input)
        .iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();
    input
        .iter()
        .map(|line| extrapolate(line))
        .sum::<i32>()
        .into()
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::submit::submit(9, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::submit::submit(9, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
