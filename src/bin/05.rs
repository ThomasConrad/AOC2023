#[derive(Debug)]
struct BitField {
    data: Vec<u128>,
}

impl BitField {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn set(&mut self, index: [u32; 2]) {
        let idx = index[0] as usize;
        let bit = index[1] as u128;
        if idx >= self.data.len() {
            self.data.resize(idx + 1, 0);
        }
        self.data[idx] |= 1 << bit;
    }

    fn _get(&self, index: [u32; 2]) -> bool {
        let idx = index[0] as usize;
        let bit = index[1] as u128;
        if idx >= self.data.len() {
            false
        } else {
            self.data[idx] & (1 << bit) != 0
        }
    }

    fn row(&self, index: u32) -> u128 {
        self.data[index as usize]
    }

    fn factors(&self, index: u32) -> Vec<u32> {
        let mut factors = Vec::new();
        for i in 0..128 {
            if self.data[index as usize] & (1 << i) != 0 {
                factors.push(i);
            }
        }
        factors
    }
}

impl std::fmt::Display for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, &n) in self.data.iter().enumerate() {
            write!(f, "{:3}: ", i)?;
            for j in 0..128 {
                write!(f, "{}", if n & (1 << j) != 0 { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> (BitField, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut bitfield = BitField::new();
    for rule in rules.lines() {
        let (prior, posterior) = rule.split_once("|").unwrap();
        bitfield.set([prior.parse().unwrap(), posterior.parse().unwrap()]);
    }

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (bitfield, updates)
}

fn check_ordering(bitfield: &BitField, update_list: &[u32]) -> bool {
    let mut seen = vec![];
    for &update in update_list {
        if bitfield.row(update) > 0 {
            //check if factors are in illegal order
            let factors = bitfield.factors(update);
            for &factor in &factors {
                if seen.contains(&factor) {
                    return false;
                }
            }
        }
        seen.push(update);
    }
    true
}

fn fix_ordering(bitfield: &BitField, update_list: &mut Vec<u32>) {
    let mut i = 0;
    let mut seen = vec![];
    'outer: while i < update_list.len() {
        let update = update_list[i];
        if bitfield.row(update) > 0 {
            //check if factors are in illegal order
            let factors = bitfield.factors(update);
            for &factor in &factors {
                if seen.contains(&factor) {
                    let idx = seen.iter().position(|&x| x == factor).unwrap();
                    update_list.swap(i, idx);

                    //reset loop
                    seen.clear();
                    i = 0;
                    continue 'outer;
                }
            }
        }
        seen.push(update);
        i += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (bitfield, updates) = parse(input);

    Some(
        updates
            .iter()
            .filter(|&update_list| check_ordering(&bitfield, update_list))
            .map(|update_list| update_list[update_list.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (bitfield, mut updates) = parse(input);

    Some(
        updates
            .iter_mut()
            .filter(|update_list| !check_ordering(&bitfield, update_list))
            .map(|update_list| {
                fix_ordering(&bitfield, update_list);
                update_list[update_list.len() / 2]
            })
            .sum(),
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::submit::submit(5, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::submit::submit(5, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(143));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
