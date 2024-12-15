use std::collections::HashMap;

use advent_of_code::helpers::get_safe;

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<[isize; 2]>) {
    let mut heads = Vec::new();
    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '0' {
                            heads.push([x as isize, y as isize]);
                        }
                        c as u8 - b'0'
                    })
                    .collect()
            })
            .collect(),
        heads,
    )
}

fn get_neighbors(map: &Vec<Vec<u8>>, [x, y]: &[isize; 2], height: u8) -> Vec<[isize; 2]> {
    [[-1, 0], [1, 0], [0, -1], [0, 1]]
        .iter()
        .filter_map(|[dx, dy]| {
            get_safe([x + dx, y + dy], map)
                .filter(|&&c| c == height + 1)
                .map(|_| [x + dx, y + dy])
        })
        .collect()
}

fn hike<'a>(
    map: &Vec<Vec<u8>>,
    pos: impl Iterator<Item = (&'a [isize; 2], &'a u32)>,
    height: u8,
) -> Vec<([isize; 2], u32)> {
    if height == 9 {
        return pos.map(|(p, m)| (*p, *m)).collect();
    }

    let pos = pos
        .flat_map(|(p, mult)| {
            get_neighbors(map, p, height)
                .into_iter()
                .map(move |n| (n, *mult))
        })
        .fold(HashMap::new(), |mut acc, (p, mult)| {
            *acc.entry(p).or_insert(0) += mult;
            acc
        });
    return hike(map, pos.iter(), height + 1);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, heads) = parse(input);
    Some(
        heads
            .iter()
            .map(|head| hike(&map, [(head, &1)].into_iter(), 0).len() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, heads) = parse(input);
    Some(
        hike(&map, heads.iter().map(|p| (p, &1)), 0)
            .iter()
            .map(|(_, v)| *v)
            .sum::<u32>() as u32,
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::submit::submit(10, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::submit::submit(10, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(36));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(81));
    }
}
