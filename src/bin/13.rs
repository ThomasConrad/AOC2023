use std::ops::Sub;

struct ClawConfig {
    a_vec: [f64; 2],
    b_vec: [f64; 2],
    price: [f64; 2],
}

fn parse(input: &str) -> Vec<ClawConfig> {
    input
        .split("\n\n")
        .map(|group| {
            let regex = regex::Regex::new(r"\d+").unwrap();

            let nums = regex
                .captures_iter(group)
                .map(|cap| cap[0].parse().unwrap())
                .collect::<Vec<_>>();
            ClawConfig {
                a_vec: [nums[0], nums[1]],
                b_vec: [nums[2], nums[3]],
                price: [nums[4], nums[5]],
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let configs = parse(input);

    let mut min_cost = 0.;

    for config in configs {
        let det = 1.0 / (config.a_vec[0] * config.b_vec[1] - config.a_vec[1] * config.b_vec[0]);
        let inverse = [
            [config.b_vec[1] * det, -config.b_vec[0] * det],
            [-config.a_vec[1] * det, config.a_vec[0] * det],
        ];

        let [a, b] = [
            inverse[0][0] * config.price[0] + inverse[0][1] * config.price[1],
            inverse[1][0] * config.price[0] + inverse[1][1] * config.price[1],
        ];

        if a.round().sub(a).abs() > 0.001 || b.round().sub(b).abs() > 0.001 || a > 100. || b > 100.
        {
            continue;
        }

        min_cost += a * 3. + b;
    }

    Some(min_cost.round() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let configs = parse(input);

    let mut min_cost = 0.;

    let conversion_error = 10000000000000.0;
    for config in configs {
        let det = 1.0 / (config.a_vec[0] * config.b_vec[1] - config.a_vec[1] * config.b_vec[0]);
        let inverse = [
            [config.b_vec[1] * det, -config.b_vec[0] * det],
            [-config.a_vec[1] * det, config.a_vec[0] * det],
        ];

        let [a, b] = [
            inverse[0][0] * (config.price[0] + conversion_error)
                + inverse[0][1] * (config.price[1] + conversion_error),
            inverse[1][0] * (config.price[0] + conversion_error)
                + inverse[1][1] * (config.price[1] + conversion_error),
        ];

        if a.round().sub(a).abs() > 0.001 || b.round().sub(b).abs() > 0.001 {
            continue;
        }

        min_cost += a.round() * 3. + b.round();
    }

    Some(min_cost as u64)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::submit::submit(13, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::submit::submit(13, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(480));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
