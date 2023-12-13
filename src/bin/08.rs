use ahash::{HashMap, HashSet, HashSetExt};

fn parse(input: &str) -> (Vec<char>, HashMap<[char; 3], [[char; 3]; 2]>) {
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().collect::<Vec<_>>();
    let map = lines
        .skip(1)
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let key = [chars[0], chars[1], chars[2]];
            let left = [chars[7], chars[8], chars[9]];
            let right = [chars[12], chars[13], chars[14]];
            (key, [left, right])
        })
        .collect::<HashMap<_, _>>();
    (moves, map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (moves, map) = parse(input);
    let mut state = ['A', 'A', 'A'];
    let mut cursor = 0;
    while state != ['Z', 'Z', 'Z'] {
        state = match moves[cursor % moves.len()] {
            'L' => map[&state][0],
            'R' => map[&state][1],
            _ => {
                panic!("Invalid move");
            }
        };
        cursor += 1;
    }
    Some(cursor as u32)
}

fn prime_factors(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut n = n;
    //factor out the 2s
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 2 {
        factors.push(n);
    }
    factors
}

pub fn part_two(input: &str) -> Option<u64> {
    let (moves, map) = parse(input);
    let states = map.keys().filter(|key| key[2] == 'A').collect::<Vec<_>>();
    let mut possible_cursors = Vec::new();
    for i in 0..states.len() {
        let mut state = states[i];
        let mut cursor = 0;
        let mut prev_states = HashSet::new();
        possible_cursors.push(Vec::new());
        while !prev_states.contains(&(state, cursor % moves.len())) {
            prev_states.insert((state, cursor % moves.len()));
            state = match moves[cursor % moves.len()] {
                'L' => &map[state][0],
                'R' => &map[state][1],
                _ => {
                    panic!("Invalid move");
                }
            };
            cursor += 1;
            if state[2] == 'Z' {
                possible_cursors[i].push(cursor);
            }
        }
    }
    let factors = possible_cursors
        .iter()
        .flat_map(|cursors| {
            cursors
                .iter()
                .flat_map(|cursor| prime_factors(*cursor as u32))
        })
        .map(u64::from)
        .collect::<HashSet<u64>>();
    Some(factors.iter().product())
    // find the lowest common multiple of the possible cursors
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::submit::submit(8, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::submit::submit(8, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let input = advent_of_code::read_file("examples", 8);
    //     assert_eq!(part_one(&input), Some(6));
    // }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(6));
    }
}
