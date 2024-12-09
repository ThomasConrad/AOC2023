use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (HashMap<char, Vec<[usize; 2]>>, [usize; 2]) {
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(line_num, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(char_num, c)| {
                        if c != '.' {
                            let x = char_num;
                            let y = line_num;
                            return Some((c, [x, y]));
                        }
                        return None;
                    })
                    .collect::<Vec<_>>()
            })
            .fold(HashMap::new(), |mut acc, (c, pos)| {
                acc.entry(c).or_insert_with(Vec::new).push(pos);
                acc
            }),
        [
            input.lines().count(),
            input.lines().next().unwrap().chars().count(),
        ],
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennae, size) = parse(input);

    Some(
        antennae
            .iter()
            .flat_map(|(_, antennas)| {
                antennas.iter().flat_map(|antenna_a| {
                    antennas
                        .iter()
                        .filter_map(|antenna_b| {
                            if antenna_a == antenna_b {
                                return None;
                            }
                            let x = (2 * antenna_a[0]).checked_sub(antenna_b[0]);
                            let y = (2 * antenna_a[1]).checked_sub(antenna_b[1]);
                            if x.map_or(true, |x| x >= size[0]) || y.map_or(true, |y| y >= size[1])
                            {
                                return None;
                            }
                            Some([x, y])
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

fn _display_map(map: HashSet<[usize; 2]>, size: [usize; 2]) {
    for y in 0..size[1] {
        for x in 0..size[0] {
            if map.contains(&[x, y]) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennae, size) = parse(input);

    Some(
        antennae
            .iter()
            .flat_map(|(_, antennas)| {
                antennas.iter().flat_map(|antenna_a| {
                    antennas
                        .iter()
                        .filter_map(|antenna_b| {
                            if antenna_a == antenna_b {
                                return None;
                            }
                            let mut antinodes = vec![antenna_a.clone()];
                            let delta_x = antenna_b[0] as isize - antenna_a[0] as isize;
                            let delta_y = antenna_b[1] as isize - antenna_a[1] as isize;
                            let mut x = antenna_a[0] as isize + delta_x;
                            let mut y = antenna_a[1] as isize + delta_y;
                            while x >= 0 && y >= 0 && x < size[0] as isize && y < size[1] as isize {
                                antinodes.push([x as usize, y as usize]);
                                x += delta_x;
                                y += delta_y;
                            }

                            Some(antinodes)
                        })
                        .flatten()
                        .collect::<Vec<_>>()
                })
            })
            .collect::<HashSet<_>>()
            .len() as u32,
    )
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

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(14));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(34));
    }
}
