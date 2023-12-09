fn char_to_u8(c: char) -> u8 {
    match c {
        '1' => 1,
        '2'..='9' => c as u8 - b'0',
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card"),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
}

impl Hand {
    fn new(cards: impl IntoIterator<Item = char>) -> Self {
        Hand {
            cards: cards.into_iter().map(char_to_u8).collect(),
        }
    }

    fn get_kind(&self) -> u8 {
        //five of a kind

        let mut cards = [0; 14];
        for &card in self.cards.iter() {
            cards[card as usize - 1] += 1;
        }

        let mut highest = 0;
        let mut second_highest = 0;
        for &card in cards.iter().skip(1) {
            if card > highest {
                second_highest = highest;
                highest = card;
            } else if card > second_highest {
                second_highest = card;
            }
        }

        let num_jokers = cards[0];
        highest += num_jokers;

        match (highest, second_highest) {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _ => 0,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //detect hand type
        let self_kind = self.get_kind();
        let other_kind = other.get_kind();
        if self_kind != other_kind {
            return self_kind.cmp(&other_kind);
        }
        //detect first highest card
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

fn parse(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = Hand::new(split.next().unwrap().chars());
            let bet = split.next().unwrap().parse().unwrap();
            (hand, bet)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parsed = parse(input);
    parsed.sort();
    Some(
        parsed
            .iter()
            .enumerate()
            .map(|(i, (_, bet))| (i as u32 + 1) * bet)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input.replace("J", "1").as_str())
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
