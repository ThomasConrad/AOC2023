use regex::Regex;
use std::sync::OnceLock;

static REGEX: OnceLock<Regex> = OnceLock::new();

fn get_regex() -> &'static Regex {
    REGEX.get_or_init(|| {
        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
        Regex::new(pattern).unwrap()
    })
}

#[derive(Clone, Copy)]
enum Action {
    Do,
    Dont,
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = get_regex();

    Some(
        re.captures_iter(input)
            .map(|capture| capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = get_regex();

    let mut actions = vec![(0, Action::Do)];

    input.match_indices("do()").for_each(|(i, _)| {
        actions.push((i, Action::Do));
    });
    input.match_indices("don't()").for_each(|(i, _)| {
        actions.push((i, Action::Dont));
    });
    actions.sort_by_key(|(i, _)| *i);

    Some(
        re.captures_iter(input)
            .map(|capture| {
                let idx = capture.get(0).unwrap().start();
                let action = actions.iter().rev().find(|(i, _)| *i < idx).unwrap().1;
                if let Action::Dont = action {
                    return 0;
                }
                capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap()
            })
            .sum(),
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::submit::submit(3, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::submit::submit(3, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(48));
    }
}
