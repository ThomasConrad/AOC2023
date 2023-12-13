use ahash::HashSet;

fn parse(input: &str) -> Vec<(HashSet<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split('|').collect::<Vec<_>>();
            let winning = parts[0]
                .split(':')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let nums: Vec<u32> = parts[1]
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (winning, nums)
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let games = parse(input);
    let mut score = 0;
    for (winning, nums) in games {
        let mut num_matches = 0;
        for num in nums {
            if winning.contains(&num) {
                num_matches += 1;
            }
        }
        if num_matches > 0 {
            score += 1 << (num_matches - 1);
        }
    }
    Some(score as u32)
}

struct Game {
    num: u32,
    amount: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse(input);
    let mut queue = Vec::with_capacity(games.len());
    for i in 0..games.len() {
        queue.push(Game {
            num: i as u32,
            amount: 1,
        });
    }

    for i in 0..queue.len() {
        let game_num = queue[i].num;
        let game_amount = queue[i].amount;
        let mut num_matches = 0;

        for num in &games[game_num as usize].1 {
            if games[game_num as usize].0.contains(num) {
                num_matches += 1;
            }
        }
        for j in 0..num_matches {
            queue[i + j + 1].amount += game_amount;
        }
    }

    Some(queue.into_iter().map(|g| g.amount).sum())
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::submit::submit(4, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::submit::submit(4, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(30));
    }
}
