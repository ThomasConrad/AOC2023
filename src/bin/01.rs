pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace();
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(&l, &r)| l.abs_diff(r))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace();
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    Some(
        left.into_iter()
            .map(|l| {
                // count occurence of l in right
                let count = right.iter().filter(|&&r| r == l).count();
                l * count as u32
            })
            .sum(),
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::submit::submit(1, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::submit::submit(1, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
