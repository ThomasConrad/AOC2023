struct Mapping {
    source: u64,
    destination: u64,
    range: u64,
}

struct MappingSet {
    mappings: Vec<Mapping>,
}

impl Mapping {
    fn new(source: u64, destination: u64, range: u64) -> Self {
        Self {
            source,
            destination,
            range,
        }
    }

    fn get_destination(&self, source: u64) -> Option<u64> {
        if source >= self.source && source < self.source + self.range {
            Some(self.destination + source - self.source)
        } else {
            None
        }
    }
}

impl MappingSet {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    fn get_destination(&self, source: u64) -> (u64, u64) {
        for mapping in &self.mappings {
            if let Some(destination) = mapping.get_destination(source) {
                return (
                    destination,
                    mapping.range - (destination - mapping.destination),
                );
            }
        }
        (source, self.get_next(source).unwrap_or(u64::MAX) - source)
    }

    fn push(&mut self, mapping: Mapping) {
        self.mappings.push(mapping);
    }

    fn get_next(&self, source: u64) -> Option<u64> {
        let mut closest = None;
        for mapping in &self.mappings {
            if mapping.source > source && (closest.is_none() || mapping.source < closest.unwrap()) {
                closest = Some(mapping.source);
            }
        }
        closest
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<MappingSet>) {
    let seeds: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    let mut mapping_sets = Vec::new();
    for chunk in input.split("\n\n").skip(1) {
        let mut mappings = MappingSet::new();
        for line in chunk.lines().skip(1) {
            let mapping = line.split_whitespace().collect::<Vec<_>>();
            let source = mapping[1].parse().unwrap();
            let destination = mapping[0].parse().unwrap();
            let range = mapping[2].parse().unwrap();
            mappings.push(Mapping::new(source, destination, range));
        }
        mapping_sets.push(mappings);
    }
    (seeds, mapping_sets)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mapping_sets) = parse(input);
    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut current = seed;
        for mapping_set in &mapping_sets {
            current = mapping_set.get_destination(current).0;
        }
        if lowest > current {
            lowest = current;
        }
    }
    Some(lowest)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mapping_sets) = parse(input);
    let mut seed_ranges = Vec::<[u64; 2]>::new();
    for pair in seeds.chunks(2) {
        seed_ranges.push([pair[0], pair[1]]);
    }
    let mut seed_ranges_swap = Vec::<[u64; 2]>::new();
    for mapping_set in &mapping_sets {
        while let Some(seed_range) = seed_ranges.pop() {
            let (destination, range) = mapping_set.get_destination(seed_range[0]);
            if range < seed_range[1] {
                seed_ranges_swap.push([destination, range]);
                seed_ranges.push([seed_range[0] + range, seed_range[1] - range]);
            } else {
                seed_ranges_swap.push([destination, seed_range[1]]);
            }
        }
        std::mem::swap(&mut seed_ranges, &mut seed_ranges_swap);
    }

    Some(seed_ranges.iter().min().unwrap()[0])
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
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
