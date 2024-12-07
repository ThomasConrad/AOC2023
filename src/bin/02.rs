pub fn part_one(input: &str) -> Option<u32> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    Some(
        reports
            .into_iter()
            .map(|row| {
                let diffs: Vec<i32> = row
                    .windows(2)
                    .map(|window| window[1] as i32 - window[0] as i32)
                    .collect();

                (*diffs.iter().max().unwrap(), *diffs.iter().min().unwrap())
            })
            .filter(|(max, min)| {
                max * min > 0 && max.abs().max(min.abs()) <= 3 && max.abs().min(min.abs()) >= 1
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    Some(
        reports
            .into_iter()
            .filter(|row| {
                let test_fn = |row: Vec<u32>| {
                    let diffs: Vec<i32> = row
                        .windows(2)
                        .map(|window| window[1] as i32 - window[0] as i32)
                        .collect();

                    let max = diffs.iter().max().unwrap();
                    let min = diffs.iter().min().unwrap();

                    max * min > 0 && max.abs().max(min.abs()) <= 3 && max.abs().min(min.abs()) >= 1
                };

                if !test_fn(row.clone()) {
                    for i in 0..row.len() {
                        let mut new_row = row.clone();
                        new_row.remove(i);
                        if test_fn(new_row) {
                            return true;
                        }
                    }
                    return false;
                }
                true
            })
            .count() as u32,
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::submit::submit(2, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::submit::submit(2, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(4));
    }
}
