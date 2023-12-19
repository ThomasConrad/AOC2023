fn hash(input: &str) -> u8 {
    let mut hash = 0u8;
    input
        .bytes()
        .for_each(|c| hash = hash.wrapping_add(c).wrapping_mul(17));
    hash
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split(',')
        .map(|v| hash(&v.trim_end()) as u32)
        .sum::<u32>()
        .into()
}

#[derive(Clone)]
struct Lens {
    label: String,
    f: u8,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Lens {}

struct HashMap<T> {
    boxes: Vec<Vec<T>>,
}

impl<T: Clone + Eq> HashMap<T> {
    pub fn new() -> Self {
        Self {
            boxes: vec![Vec::new(); 256],
        }
    }

    pub fn insert(&mut self, key: u8, value: T) {
        if let Some(item) = self.boxes[key as usize].iter_mut().find(|l| **l == value) {
            *item = value;
            return;
        }
        self.boxes[key as usize].push(value);
    }

    pub fn remove(&mut self, key: u8, value: &T) {
        if let Some(idx) = self.boxes[key as usize].iter().position(|l| *l == *value) {
            self.boxes[key as usize].remove(idx);
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = HashMap::new();
    input.split(',').for_each(|v| {
        let sep_idx = v.find(|c| c == '-' || c == '=').unwrap();
        let label = v[..sep_idx].to_string();
        match v.chars().nth(sep_idx) {
            Some('-') => {
                map.remove(hash(&label), &Lens { label, f: 0 });
            }
            Some('=') => {
                let f = v[sep_idx + 1..].parse::<u8>().unwrap();
                map.insert(hash(&label), Lens { label, f });
            }
            _ => panic!("Invalid input"),
        }
    });
    Some(
        map.boxes
            .iter()
            .enumerate()
            .flat_map(|(i, bucket)| {
                bucket
                    .iter()
                    .enumerate()
                    .map(move |(j, lens)| (i + 1) * (j + 1) * lens.f as usize)
            })
            .sum(),
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::submit::submit(15, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::submit::submit(15, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(1320));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(145));
    }
}
