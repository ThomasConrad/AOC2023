use std::collections::HashMap;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn lanternfish(nums: &mut HashMap<u64, u64>) {
    let mut new_nums = HashMap::new();
    for (num, mult) in nums.iter() {
        match num {
            0 => {
                *new_nums.entry(1).or_insert(0) += mult;
            }
            // even case
            num if num.ilog10() % 2 == 1 => {
                let before = *num / 10u64.pow((num.ilog10() + 1) / 2);
                let after = *num % 10u64.pow((num.ilog10() + 1) / 2);
                *new_nums.entry(after).or_insert(0) += mult;
                *new_nums.entry(before).or_insert(0) += mult;
            }
            _ => {
                *new_nums.entry(num * 2024).or_insert(0) += mult;
            }
        }
    }

    *nums = new_nums;
}

pub fn part_one(input: &str) -> Option<u32> {
    let nums = parse(input);

    let mut hash_nums = nums.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(*num).or_insert(0) += 1;
        acc
    });

    for _ in 0..25 {
        lanternfish(&mut hash_nums);
    }

    Some(nums.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = parse(input);

    let mut hash_nums = nums.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(*num).or_insert(0) += 1;
        acc
    });

    for _ in 0..75 {
        lanternfish(&mut hash_nums);
    }

    Some(hash_nums.iter().map(|(_, &v)| v).sum())
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::submit::submit(11, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::submit::submit(11, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(55312));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(203228));
    }
}
